use crate::event::{ImageLoadBatchEvent, ImageLoadEvent};
use crate::home::HomePage;
use crate::model::ContentSet;
use crate::service::DisneyService;
use anyhow::{Context, Error, Result};
use futures::stream::StreamExt;
use futures::Stream;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use std::path::Path;
use std::sync::Arc;

mod event;
mod home;
mod model;
mod service;
mod shelf;
mod tile;

const DEFAULT_IMAGE_SIZE: &str = "1.78";
const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

#[tokio::main]
async fn main() -> Result<()> {
    let concurrency = 20_usize;
    let disney = DisneyService::new(concurrency);

    // we have all of the content sets, they don't have images loaded yet, but we can load the screen
    let fetched_content_sets = load_home_content_sets(&disney, concurrency).await?;

    // send content sets into the home page, consuming the content sets
    let mut ui = HomePage::load(fetched_content_sets.clone(), 50, 180, 20);

    // Load the SDL context
    let sdl_context = sdl2::init().map_err(Error::msg)?;
    let video_subsystem = sdl_context.video().map_err(Error::msg)?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).map_err(Error::msg)?;
    let ttf_context = sdl2::ttf::init().map_err(Error::msg)?;

    let window = video_subsystem
        .window("Home", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    let font = ttf_context
        .load_font(Path::new("./assets/Roboto-Regular.ttf"), 18)
        .map_err(Error::msg)?;

    // Draw the initial UI (it will be blank mostly until we have images)
    ui.draw(&font, &mut canvas);

    // Initialize the event loop
    let mut event_pump = sdl_context.event_pump().map_err(Error::msg)?;
    let ev = sdl_context.event().map_err(Error::msg)?;
    ev.register_custom_event::<ImageLoadBatchEvent>()
        .map_err(Error::msg)?;
    let evs = ev.event_sender();
    let event_sender = Arc::new(evs);

    // kick off background process to async load images and send events
    // each batch of images are sent to the main event loop
    // to update the view
    tokio::spawn({
        let event_sender = Arc::clone(&event_sender);
        async move {
            stream_tile_images(
                &disney,
                fetched_content_sets,
                20,
                DEFAULT_IMAGE_SIZE.to_string(),
            )
            .await
            .for_each(|image_batch_event| {
                let event_sender = Arc::clone(&event_sender);
                async move {
                    event_sender
                        .push_custom_event(image_batch_event)
                        .expect("Unable to push custom event");
                }
            })
            .await;
        }
    });

    // This moves all the things we just drew to the foreground
    canvas.present();

    'running: loop {
        // Handle events forever
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    ui.on_key_left();
                    update_ui(&mut canvas, &font, &mut ui);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    ui.on_key_right();
                    update_ui(&mut canvas, &font, &mut ui);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    ui.on_key_down();
                    update_ui(&mut canvas, &font, &mut ui);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    ui.on_key_up();
                    update_ui(&mut canvas, &font, &mut ui);
                }
                custom_event if custom_event.is_user_event() => {
                    let ce = custom_event
                        .as_user_event_type::<ImageLoadBatchEvent>()
                        .expect("Unable to cast custom event to ImageLoadBatchEvent");

                    for event in ce.events {
                        ui.on_image_load(event);
                    }
                    update_ui(&mut canvas, &font, &mut ui);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn update_ui(canvas: &mut Canvas<Window>, font: &Font, ui: &mut HomePage) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    ui.draw(font, canvas);
    canvas.present();
}

/// Produces an async stream that background
/// loads all of the iamges in a controlled manner
/// Can tweak parallelism in here
async fn stream_tile_images<'a>(
    disney: &'a DisneyService,
    content_sets: Vec<ContentSet>,
    concurrency: usize,
    image_size: String,
) -> impl Stream<Item = ImageLoadBatchEvent> + '_ {
    // Iterate over pairs (content_set_tile, tile_img_url)
    // filtering out (and logging) those tile images without a url
    let disney = Arc::new(disney);
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
    let disney = Arc::clone(&disney);
    let fetch_image_futures =
        futures::stream::iter(curated_items).map(move |(content_set_title, item_image_url)| {
            let disney = Arc::clone(&disney);
            async move {
                disney
                    .load_tile_image_bytes(&item_image_url)
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
            }
        });

    // Taking our stream, run it concurrently (buffered)
    // and then group together 15 images at a time or until the stream is finished
    // sending batches of images to the UI is faster than
    // sending individual images to the ui
    fetch_image_futures
        .buffered(concurrency)
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
async fn load_home_content_sets(
    disney: &DisneyService,
    concurrency: usize,
) -> Result<Vec<ContentSet>> {
    // load the home screen
    let home_screen = disney.load_home_screen().await?;

    // We want to maintain order of the content sets, so all will flow
    // through the loader, the ones that exist already will go through completed
    let fetched_content_sets = futures::stream::iter(home_screen.content_sets())
        .map(|cs| async {
            if let Some(ref_id) = cs.ref_id() {
                disney
                    .load_set_ref(ref_id)
                    .await
                    .context("Loading content set")
            } else {
                Ok(cs)
            }
        })
        .buffered(concurrency)
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
