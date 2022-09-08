use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use reqwest::{Client, StatusCode};

use crate::model::{ContentSet, HomeRoot, HomeScreen, SetRef};

pub struct DisneyService {
    pub concurrency: usize,
    http: Client,
}
impl DisneyService {
    pub fn new(concurrency: usize) -> Self {
        Self {
            concurrency,
            http: Client::new(),
        }
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
