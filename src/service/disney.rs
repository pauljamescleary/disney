use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use reqwest::{Client, StatusCode};

use crate::{
    event::{ImageLoadBatchEvent, ImageLoadEvent},
    model::{ContentSet, HomeRoot, HomeScreen, SetRef},
};

pub struct DisneyService {
    concurrency: usize,
    http: Client,
}
impl DisneyService {
    pub fn new(concurrency: usize) -> Self {
        Self {
            concurrency,
            http: Client::new(),
        }
    }

    /// Produces an async stream that background
    /// loads all of the iamges in a controlled manner
    /// Can tweak parallelism in here
    pub async fn stream_tile_images(
        &self,
        content_sets: Vec<ContentSet>,
        image_size: String,
    ) -> impl Stream<Item = ImageLoadBatchEvent> + '_ {
        // Iterate over pairs (content_set_tile, tile_img_url)
        // filtering out (and logging) those tile images without a url
        let curated_items = content_sets.into_iter().flat_map(move |cs| {
            let cs_title = cs.title().clone();
            let img_size = image_size.clone();
            cs.items().into_iter().filter_map(move |item| {
                if let Some(img_url) = item.tile_image_url(&img_size) {
                    Some((cs_title.clone(), img_url.clone()))
                } else {
                    println!("No image found for size {:?} and item {:?}", img_size, item);
                    None
                }
            })
        });

        // Main flow, for each image, fetch the image bytes from the cdn
        // logging any failures along the way
        let fetch_image_futures = futures::stream::iter(curated_items).map(
            move |(content_set_title, item_image_url)| async move {
                self.load_tile_image_bytes(&item_image_url)
                    .await
                    .map(|image_bytes| ImageLoadEvent {
                        img_url: item_image_url.clone(),
                        bytes: image_bytes,
                        content_set_title: content_set_title.clone(),
                    })
                    .map(Some)
                    .unwrap_or_else(|e| {
                        println!("Failed fetching image url {:?}", e);
                        None
                    })
            },
        );

        // Taking our stream, run it concurrently (buffered)
        // and then group together 15 images at a time or until the stream is finished
        // sending batches of images to the UI is faster than
        // sending individual images to the ui
        fetch_image_futures
            .buffered(self.concurrency)
            .ready_chunks(15)
            .map(|img_load_events| ImageLoadBatchEvent {
                events: img_load_events.into_iter().flatten().collect(),
            })
    }

    /// Loads home page content sets
    ///
    /// When requesting the home page, some of the
    /// content sets are returned as "refs", and
    /// "refs" do not have any images in them.
    ///
    /// Load any missing content sets
    pub async fn load_home_content_sets(&self) -> Result<Vec<ContentSet>> {
        // load the home screen
        let home_screen = self.load_home_screen().await?;

        // We want to maintain order of the content sets, so all will flow
        // through the loader, the ones that exist already will go through completed
        let fetched_content_sets = futures::stream::iter(home_screen.content_sets())
            .map(|cs| async {
                if let Some(ref_id) = cs.ref_id() {
                    self.load_set_ref(ref_id)
                        .await
                        .context("Loading content set")
                } else {
                    Ok(cs)
                }
            })
            .buffered(self.concurrency)
            .collect::<Vec<Result<ContentSet>>>()
            .await;

        let content_sets_without_failures: Vec<ContentSet> = fetched_content_sets
            .into_iter()
            .inspect(|result| {
                if let Err(e) = result {
                    println!("Failure loading content set: {:?}", e);
                }
            })
            .filter_map(|r| r.ok())
            .collect();

        Ok(content_sets_without_failures)
    }

    pub async fn load_home_screen(&self) -> Result<HomeScreen> {
        // Load the raw response from the cdn
        let response = self
            .http
            .get("https://cd-static.bamgrid.com/dp-117731241344/home.json")
            .send()
            .await?;

        // parse the bytes out of the response
        let data = response.bytes().await?;

        // deserialize the home Home, this has an added "data" element
        let root: HomeRoot = serde_json::from_slice(&data)?;
        Ok(root.home_screen())
    }

    pub async fn load_set_ref(&self, ref_id: &String) -> Result<ContentSet> {
        let url = format!(
            "https://cd-static.bamgrid.com/dp-117731241344/sets/{}.json",
            ref_id
        );
        let response = self
            .http
            .get(url)
            .send()
            .await
            .context("Fetching ref from url")?;
        let data = response
            .bytes()
            .await
            .context("Fetching bytes from response")?;
        let set_ref: SetRef = serde_json::from_slice(&data)
            .context(format!("Deserializing curated set ref {}", ref_id))?;
        set_ref
            .content_set()
            .ok_or_else(|| anyhow!("Unable to find a curated set"))
    }

    pub async fn load_tile_image_bytes(&self, img_url: &String) -> Result<Bytes> {
        let response = reqwest::get(img_url).await?;
        if response.status() == StatusCode::OK {
            response
                .bytes()
                .await
                .context("Retreiving bytes from response")
        } else {
            Err(anyhow::anyhow!(
                "Unable to find image data: invalid image response code {:?}",
                response.status()
            ))
        }
    }
}
