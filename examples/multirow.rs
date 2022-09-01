use bytes::Bytes;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Canvas, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::Window;
use std::cmp::min;
use std::path::Path;
use std::time::Duration;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("game tutorial", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    // create a bunch of rects, the width extends beyond the screen edge
    // and the height looks like the example in the instructions
    // Add several rows to go beyond the bottom of the main viewport
    let border_margin = 20;

    // there are 13 rows in the home.json, so let's redner that many
    canvas.set_draw_color(Color::BLACK);
    // IMPORTANT, clear eliminates any kind of weird noise that you might see, not exactly sure when to use it still
    canvas.clear();
    
    let rects: Vec<Rect> = (0..13_i32).map(|i| {
        let y = border_margin + i * (150 + border_margin);
        Rect::new(border_margin, y, SCREEN_WIDTH * 2, 150)
    }).collect();

    canvas.set_draw_color(Color::WHITE);
    canvas.set_scale(1.0, 1.0).unwrap();
    canvas.draw_rects(rects.as_slice()).unwrap();    

    // This moves all the things we just drew to the foreground
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    println!("RIGHT!");
                }
                _ => {}
            }
        }
    }
    Ok(())
}


fn load_image() -> Bytes {
    // TODO: obviously we want to do this as non blocking    
    let URL = "https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/BA5D31B7889E04AE0499D1B83A6E563E95B031436225C68D69E4C4789E13F183/scale?format=jpeg&quality=90&scalingAlgorithm=lanczos3&width=500";

    // TODO: Ugly, need better conversion into anyhow here
    reqwest::blocking::get(URL).expect("Failed fetching image").bytes().expect("Failed loading bytes from URL")
}
