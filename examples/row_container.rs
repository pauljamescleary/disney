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
use std::rc::Rc;
use std::time::Duration;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

struct RowTile {
    img: String, // Url to the image
    height: u32,
    width: u32,
}
impl RowTile {
    fn render(&self, x: i32, y: i32, canvas: &mut Canvas<Window>) {
        // let's just draw an inner tile
        // the container will handle the spacing, height, width, xy coodrinates
        let old_color = canvas.draw_color();
        canvas.set_draw_color(Color::GREEN);
        canvas.draw_rect(Rect::new(x, y, self.width, self.height)).unwrap();
        canvas.set_draw_color(old_color);
    }
}

struct RowContainer {
    title: String,
    height: u32,
    width: u32,
    x: i32,
    y: i32,
    tiles: Vec<RowTile>,    
}
impl RowContainer {
    fn render(&self, font: &Font, canvas: &mut Canvas<Window>) {
        // TODO: I don't love passing in the font here, but it has a few lifetimes
        // so making it an attribute of the RowContainer is annoying,
        // It feels odd to load the same font to draw every container

        // for now, just render the container with the title
        let surface = font
            .render(&self.title)
            .blended(Color::WHITE)
            .unwrap();

        let tc = canvas.texture_creator();
        let texture = tc
            .create_texture_from_surface(&surface)            
            .unwrap();

        // Determine the size of the text
        let TextureQuery { width: text_width, height: text_height, .. } = texture.query(); 

        // Place the title at the x,y coordinates
        let title_rect = Rect::new(self.x, self.y, text_width , text_height);

        // The row height is the height of the title - the height of myself
        let row_rect = Rect::new(self.x, self.y + text_height as i32, self.width, self.height - text_height);

        canvas.copy(&texture, None, Some(title_rect)).unwrap();

        // Not sure if it makes sense to draw the tiles first or the row?
        let margin = 10_i32;        
        let mut posx = self.x + margin;        
        let posy = self.y + margin + text_height as i32;
        for tile in self.tiles.iter() {
            tile.render(posx, posy, canvas);
            posx = posx + tile.width as i32 + (margin * 2);
        }
        canvas.draw_rect(row_rect).unwrap();        
    }
}

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
        let y = border_margin + i * (150 + 20 + border_margin);
        let rc = RowContainer {
            title: format!("Title at {}", i),
            height: 150,
            width: SCREEN_WIDTH * 2,
            x: border_margin,
            y,
            tiles: vec![
                RowTile {
                    img: "ONE".to_string(),
                    height: 100,
                    width: 150,
                },
                RowTile {
                    img: "TWO".to_string(),
                    height: 100,
                    width: 150,
                },
                RowTile {
                    img: "THREE".to_string(),
                    height: 100,
                    width: 150,
                },
                RowTile {
                    img: "FOUR".to_string(),
                    height: 100,
                    width: 150,
                }                    
            ]
        };

        rc.render(&font, &mut canvas);
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
