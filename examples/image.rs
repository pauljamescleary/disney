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

    // Load an image
    let bg_image = load_image();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture_bytes(bg_image.as_ref()).expect("Failed loading texture bytes");  
    
    // let's create a rect with some borders, and put the image there
    let border_margin = 20;
    let tile = Rect::new(border_margin, border_margin, SCREEN_WIDTH - border_margin as u32 * 2, SCREEN_HEIGHT - border_margin as u32 * 2);
    canvas.set_draw_color(Color::BLACK);

    // Note: you do not need to explicitly draw the rect, when it is copied below
    // we are copying the texture (the image) into the RECT, and putting that onto the current canvas rendering target
    // which is the surface of the WINDOW itself
    // THAT SAID, it appears it doesn't matter either way
    // canvas.draw_rect(tile).unwrap();

    // IMPORTANT, clear eliminates any kind of weird noise that you might see, not exactly sure when to use it still
    canvas.clear();

    // Copies the texture into the canvas, targeting the tile RECT
    // the TILE rect has the border, and this will force both to be rendered
    canvas.copy(&texture, None, Some(tile)).unwrap();

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
