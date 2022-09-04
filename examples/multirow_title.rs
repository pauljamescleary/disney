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
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem.window("game tutorial", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");    

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();        
    let font = ttf_context.load_font(Path::new("./assets/Roboto-Regular.ttf"), 18)?;
    // font.set_style(sdl2::ttf::FontStyle::);        

    // create a bunch of rects, the width extends beyond the screen edge
    // and the height looks like the example in the instructions
    // Add several rows to go beyond the bottom of the main viewport
    let border_margin = 40;

    // there are 13 rows in the home.json, so let's redner that many
    canvas.set_draw_color(Color::BLACK);
    // IMPORTANT, clear eliminates any kind of weird noise that you might see, not exactly sure when to use it still
    canvas.clear();    

    // When we have a title, each row is not really a single rect
    // but rather 2 rects, one of them has the text to display for the title
    canvas.set_draw_color(Color::WHITE);
    (0..13_i32).for_each(|i| {
        // Every row will have 2 rects
        // one will be the title bar, the second 
        // will be the TILE bar

        // Create the text
        let surface = font
            .render(format!("Title at {}", i).as_str())
            .blended(Color::WHITE)
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)            
            .unwrap();

        // Determine the size of the text
        let TextureQuery { width, height, .. } = texture.query(); 

        let y = border_margin + i * (150 + 20 + border_margin + height as i32);
        let title_rect = Rect::new(border_margin, y, width , height);
        let tile_rect = Rect::new(border_margin, y + height as i32, SCREEN_WIDTH * 2, 150);
        
        canvas.copy(&texture, None, Some(title_rect)).unwrap();
        canvas.draw_rect(tile_rect).unwrap();
    });

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
