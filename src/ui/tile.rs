use crate::model::home::ContentSetItem;
use bytes::Bytes;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Represents an individual tile on a shelf
/// related to some content, a program or a tv series
pub struct ShelfTile {
    img_raw: Option<Bytes>,
    img_url: String,
    rect: Rect,
    selected: bool,
    default_width: u32,
    default_height: u32,
}
impl ShelfTile {
    /// Loads a tile
    pub fn load(item: ContentSetItem, x: i32, y: i32, height: u32) -> ShelfTile {
        let default_width = (height as f32 * 1.78) as u32;
        let default_height = height;
        let img_url = item.tile_image_url("1.78").expect("Missing tile image");
        ShelfTile {
            img_raw: None,
            img_url: img_url.clone(),
            rect: Rect::new(x, y, default_width, default_height),
            selected: false,
            default_width,
            default_height,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        // only draw if we are in view
        let viewport = canvas.viewport();
        if self.right() >= 0 && self.left() <= viewport.right() {
            if let Some(bytes) = &self.img_raw {
                let texture_creator = canvas.texture_creator();
                let texture = texture_creator.load_texture_bytes(bytes).unwrap();

                if self.selected {
                    canvas.set_draw_color(Color::WHITE);
                    canvas
                        .fill_rect(Rect::new(
                            self.x() - 5,
                            self.y() - 5,
                            self.width() + 10,
                            self.height() + 10,
                        ))
                        .unwrap();
                    canvas.copy(&texture, None, Some(self.rect)).unwrap();
                    canvas.set_draw_color(Color::BLACK);
                } else {
                    canvas.set_draw_color(Color::BLACK);
                    canvas.copy(&texture, None, Some(self.rect)).unwrap();
                }
            } else {
                // Empty
                canvas.set_draw_color(Color::WHITE);
                canvas.draw_rect(self.rect).unwrap();
                canvas.set_draw_color(Color::BLACK);
            }
        }
    }

    pub fn set_img(&mut self, bytes: Bytes) {
        self.img_raw = Some(bytes);
    }

    pub fn image_url(&self) -> &String {
        &self.img_url
    }

    pub fn set_x(&mut self, new_x: i32) {
        self.rect.set_x(new_x);
    }

    pub fn x(&self) -> i32 {
        self.rect.x()
    }

    pub fn set_y(&mut self, new_y: i32) {
        self.rect.set_y(new_y);
    }

    pub fn y(&self) -> i32 {
        self.rect.y()
    }

    pub fn left(&self) -> i32 {
        self.rect.left()
    }

    pub fn right(&self) -> i32 {
        self.rect.right()
    }

    pub fn height(&self) -> u32 {
        self.rect.height()
    }

    pub fn width(&self) -> u32 {
        self.rect.width()
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn select(&mut self) {
        if !self.selected {
            self.selected = true;

            // Scale the image slightly
            let new_width = self.rect.width() + 36;
            let new_height = self.rect.height() + 20;
            let new_x = self.rect.x - 18;
            let new_y = self.rect.y - 10;

            self.rect.set_x(new_x);
            self.rect.set_y(new_y);
            self.rect.set_height(new_height);
            self.rect.set_width(new_width);
        }
    }

    pub fn unselect(&mut self) {
        if self.selected {
            self.selected = false;

            // descale the image
            let new_x = self.rect.x + 18_i32;
            let new_y = self.rect.y + 10_i32;
            self.rect.set_x(new_x);
            self.rect.set_y(new_y);
            self.rect.set_height(self.default_height);
            self.rect.set_width(self.default_width);
        }
    }
}
