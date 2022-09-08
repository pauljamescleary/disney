use anyhow::{Error, Result};
use event::ImageLoadBatchEvent;
use futures::StreamExt;
use model::home::ContentSet;
use sdl2::event::{Event, EventSender};
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use service::disney::DisneyService;
use std::path::Path;
use std::sync::Arc;
use ui::home_page::HomePage;

mod event;
mod model;
mod service;
mod ui;

const DEFAULT_IMAGE_SIZE: &str = "1.78";
const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;
const DEFAULT_CONCURRENCY: usize = 20;

#[tokio::main]
async fn main() -> Result<()> {
    let disney = DisneyService::new(DEFAULT_CONCURRENCY);

    // we have all of the content sets, they don't have images loaded yet, but we can load the screen
    let fetched_content_sets = disney.load_home_content_sets().await?;

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
    // let event_sender = Arc::new(evs);

    // kick off background process to async load images and send events
    // each batch of images are sent to the main event loop
    // to update the view
    background_load_images(fetched_content_sets, disney, evs);

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

/// Actually paints the screen
fn update_ui(canvas: &mut Canvas<Window>, font: &Font, ui: &mut HomePage) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    ui.draw(font, canvas);
    canvas.present();
}

/// Background loads the tile images
/// piping them through to the evnet loop
/// via the EventSender
fn background_load_images(
    fetched_content_sets: Vec<ContentSet>,
    disney: DisneyService,
    event_sender: EventSender,
) {
    let event_sender = Arc::new(event_sender);
    let disney = Arc::new(disney);
    tokio::spawn({
        async move {
            disney
                .stream_tile_images(fetched_content_sets, DEFAULT_IMAGE_SIZE.to_string())
                .await
                .for_each(|image_batch_event| {
                    let event_sender = Arc::clone(&event_sender);
                    async move {

                        // Sends a batch of images into the main event loop
                        event_sender
                            .push_custom_event(image_batch_event)
                            .expect("Unable to push custom event");
                    }
                })
                .await;
        }
    });
}
