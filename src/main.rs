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

mod home;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

fn run(font_path: &Path) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsys
        .window("SDL2_TTF Example", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Load a font
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
        .render("Hello Rust!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
    canvas.clear();

    let TextureQuery { width, height, .. } = texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 64;
    let target = get_centered_rect(
        width,
        height,
        SCREEN_WIDTH - padding,
        SCREEN_HEIGHT - padding,
    );

    canvas.copy(&texture, None, Some(target))?;
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    run(Path::new("./assets/Roboto-Regular.ttf"))?;
    Ok(())

    // let sdl_context = sdl2::init()?;
    // let video_subsystem = sdl_context.video()?;
    // let font_context = sdl2::ttf::init().unwrap();

    // let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    // let window = video_subsystem.window("game tutorial", SCREEN_WIDTH, SCREEN_HEIGHT)
    //     .position_centered()
    //     .resizable()
    //     .build()
    //     .expect("could not initialize video subsystem");

    // let mut canvas = window.into_canvas().build()
    //     .expect("could not make a canvas");

    // // Load an image
    // let bg_image = load_image();
    // let texture_creator = canvas.texture_creator();
    // let texture = texture_creator.load_texture_bytes(bg_image.as_ref()).expect("Failed loading texture bytes");            

    // let mut scroller = Rect::new(0, 10, canvas.viewport().width() * 2, 200);

    // let font = font_context.load_font("./assets/Roboto-Regular.ttf", 12).unwrap();
    // let msg = font.render("What is happening here why is this thing doing this I have no idea why it is doing what it is doing...").blended(Color::RED).unwrap();
    // let texture_creator = canvas.texture_creator();
    // let msg_texture = texture_creator.create_texture_from_surface(msg).unwrap();
    // canvas.copy(&msg_texture, None, Some(scroller)).unwrap();
    // canvas.present();

    // let mut event_pump = sdl_context.event_pump()?;
    // let mut i = 0;
    // 'running: loop {
    //     // Handle events
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit {..} |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                 break 'running;
    //             },
    //             Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
    //                 println!("RIGHT!");
    //             }
    //             _ => {}
    //         }
    //     }

    //     i += 1;
    //     if i > 1200 {
    //         i = 0;
    //     }

    //     // canvas.set_draw_color(Color::BLACK);
    //     // canvas.clear();
    //     // canvas.set_draw_color(Color::WHITE);
    //     // // scroller.set_x(i);
    //     // // canvas.draw_rect(scroller).unwrap(); 
        
    //     // canvas.present();       
    //     // render_scrollable_rect(&mut canvas, i);
    //     // Time management!  60 frames per second
    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }

    // Ok(())
}

fn render_scrollable_rect(canvas: &mut Canvas<Window>, i: i32) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    // create a new rect, that is twice as wide as the window
    // leaving a margin on the left and the top
    let left_edge = 20;
    let top_edge = 20;
    let mut scroll = Rect::new(left_edge, top_edge, canvas.viewport().width() * 2, 200);

    // so we can set x to POSITIVE to move to left, and NEGATIVE to move to right
    scroll.set_x(i);

    canvas.set_draw_color(Color::WHITE);
    canvas.draw_rect(scroll).unwrap();
    // canvas.set_clip_rect(scroll);
    canvas.present();
}

fn load_image() -> Bytes {
    // TODO: obviously we want to do this as non blocking    
    let URL = "https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/BA5D31B7889E04AE0499D1B83A6E563E95B031436225C68D69E4C4789E13F183/scale?format=jpeg&quality=90&scalingAlgorithm=lanczos3&width=500";

    // TODO: Ugly, need better conversion into anyhow here
    reqwest::blocking::get(URL).expect("Failed fetching image").bytes().expect("Failed loading bytes from URL")
}

fn render_image(canvas: &mut Canvas<Window>) -> Result<(), String> {
    let bg_image = load_image();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture_bytes(bg_image.as_ref()).expect("Failed loading texture bytes");
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
    Ok(())
}

fn render(canvas: &mut Canvas<Window>) -> Result<(), String> {
        // Render
        // render(&mut canvas, Color::RGB(i, 64, 255 - i));
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // TODO: We won't have to worry about margins I don't think
        // in this below
        let ui_square = {
            let (x, y) = canvas.viewport().size();
            
            if x > y {
                // landscape
                let left_edge = (x / 2) - (y / 2);
                Rect::new(left_edge as i32, 0, y, y)
            } else {
                // portrait
                let top_edge = (y / 2) - (x / 2);
                Rect::new(0, top_edge as i32, y, y)
            }            
        };

        let mut middle_section = ui_square;
        // middle_section.set_width(middle_section.width() / 2);
        middle_section.set_width(middle_section.width() * 2);
        // middle_section.center_on(ui_square.center());

        canvas.set_draw_color(Color::WHITE);
        
        // canvas.draw_rect(ui_square)?; // draws a filled rectangle, draw_rect draws a rect with a border only using the draw color
        canvas.set_clip_rect(ui_square); // draws a filled rectangle, draw_rect draws a rect with a border only using the draw color
        canvas.draw_rect(middle_section).expect("Failed drawing middle section");
        canvas.present(); 
        
        Ok(())
}