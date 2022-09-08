use crate::event::ImageLoadEvent;
use crate::model::home::ContentSet;
use crate::ui::tile::ShelfTile;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::Window;

/// Represents a "shelf", or a row of tile images
/// that a user can scroll left and right through
/// Thesse are formed via [`ContentSet`]s, which
/// are "Curated Sets" or "Collections"
/// such as "New to Disney+"
pub struct Shelf {
    title: String,
    height: u32,
    selected: bool,
    tiles: Vec<ShelfTile>,
    selected_tile: usize,
    padding: u32,
    rect: Rect,
    content_set: ContentSet,
    label_height: Option<u32>,
}
impl Shelf {

    /// Loads a Shelf
    /// Loads tiles that live on the shelf
    pub fn load(content_set: ContentSet, height: u32, tile_padding: u32, y: i32) -> Shelf {
        let mut shelf = Shelf {
            title: content_set.title().clone(),
            height,
            selected: false,
            tiles: Vec::new(),
            selected_tile: 0,
            padding: tile_padding,
            rect: Rect::new(tile_padding as i32, y, 1920, height),
            content_set,
            label_height: None,
        };
        shelf.load_tiles();
        shelf
    }

    fn load_tiles(&mut self) {
        // load the items as tiles, from left to right
        // starting at an initial offset
        let mut x_pos = self.padding as i32;
        let y_pos = self.y() + self.padding as i32;

        for item in self.content_set.items_iter() {
            let tile = ShelfTile::load(
                item.clone(),
                x_pos,
                y_pos,
                self.height() - self.padding as u32,
            );
            x_pos = x_pos + tile.width() as i32 + self.padding as i32;
            self.tiles.push(tile);
        }
    }

    pub fn on_image_load(&mut self, event: ImageLoadEvent) {
        // find the tile matching the url and update
        let maybe_found_tile = self
            .tiles
            .iter_mut()
            .find(|tile| tile.image_url() == &event.img_url);

        if let Some(tile) = maybe_found_tile {
            tile.set_img(event.bytes);
        } else {
            println!("Unable to find tile for image {:?}", event.img_url);
        }
    }

    pub fn draw(&mut self, font: &Font, canvas: &mut Canvas<Window>) {
        // only draw if we are within view
        let viewport = canvas.viewport();
        if self.y() <= viewport.bottom() && self.bottom() >= viewport.top() {
            self.draw_label(font, canvas);
            canvas.draw_rect(self.rect).unwrap();

            if self.selected {
                // draw the selected one last so it appears above the others
                self.tiles
                    .iter()
                    .filter(|t| !t.selected())
                    .for_each(|tile| tile.draw(canvas));

                // draw the selected tile last so it is on top
                self.tiles[self.selected_tile].draw(canvas);
            } else {
                self.tiles.iter().for_each(|tile| tile.draw(canvas));
            }
        }
    }

    fn draw_label(&mut self, font: &Font, canvas: &mut Canvas<Window>) {
        let surface = font.render(&self.title).blended(Color::WHITE).unwrap();
        let tc = canvas.texture_creator();
        let texture = tc.create_texture_from_surface(&surface).unwrap();

        // Determine the size of the text
        let TextureQuery {
            width: text_width,
            height: text_height,
            ..
        } = texture.query();

        // Size the title rect and copy the font onto it
        self.label_height = Some(text_height);

        // We need to make sure the tiles stay below the shelf label
        // so reset the tiles y position once the label is set
        // and we know the exact height
        self.reset_tile_y();
        let title_rect = Rect::new(self.x(), self.y(), text_width, text_height);
        canvas.copy(&texture, None, Some(title_rect)).unwrap();
    }

    /// Update the y position for this shelf
    /// Do not forget to update the y position for all tiles as well
    pub fn set_y(&mut self, new_y: i32) {
        self.rect.set_y(new_y);
        self.reset_tile_y();
    }

    /// Resets the y position for all tiles
    fn reset_tile_y(&mut self) {
        let new_y = self.y();
        let tile_y = self
            .label_height
            .map(|lh| lh as i32 + new_y)
            .unwrap_or(new_y);
        self.tiles
            .iter_mut()
            .for_each(|tile| tile.set_y(tile_y + 10));
    }

    pub fn bottom(&self) -> i32 {
        self.rect.bottom()
    }

    pub fn x(&self) -> i32 {
        self.rect.x()
    }

    pub fn y(&self) -> i32 {
        self.rect.y()
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn select(&mut self) {
        self.selected = true;
        self.tiles[self.selected_tile].select();
    }

    pub fn unselect(&mut self) {
        self.selected = false;
        self.tiles[self.selected_tile].unselect();
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn on_key_right(&mut self) {
        // only move to new tile if this row is selected
        // safeguard
        if self.selected {
            // if we are at the end already, don't do anything
            if self.selected_tile < self.tiles.len() - 1 {
                let current_selection = self.selected_tile;
                self.selected_tile += 1;
                self.tiles[current_selection].unselect();

                // move all tiles to the left
                if current_selection == 0 {
                    // shift left everything by half
                    // since this is the first tile
                    for tile in self.tiles.iter_mut() {
                        tile.set_x(tile.x() - (tile.width() as i32 / 2) - self.padding as i32)
                    }
                } else {
                    // shift everything left by a full tile
                    let mut prior_x = 0 - self.tiles[0].width() as i32 / 2 - self.padding as i32;
                    for tile in self.tiles.iter_mut() {
                        let next_x = tile.x();
                        tile.set_x(prior_x);
                        prior_x = next_x;
                    }
                }
                self.tiles[self.selected_tile].select();
            }
        }
    }

    pub fn on_key_left(&mut self) {
        // scroll back left
        if self.selected {
            // if we are at the beginning cannot go any further
            if self.selected_tile > 0 {
                let current_selection = self.selected_tile;
                self.selected_tile -= 1;
                self.tiles[current_selection].unselect();

                // if we are going to 0, then we shift by a smaller amount
                if self.selected_tile == 0 {
                    // shift left everything by half
                    for tile in self.tiles.iter_mut() {
                        tile.set_x(tile.x() + (tile.width() as i32 / 2) + self.padding as i32)
                    }
                } else {
                    let mut prior_x =
                        self.tiles[self.tiles.len() - 1].right() + self.padding as i32;
                    for tile in self.tiles.iter_mut().rev() {
                        let next_x = tile.x();
                        tile.set_x(prior_x);
                        prior_x = next_x;
                    }
                }
                self.tiles[self.selected_tile].select();
            }
        }
    }
}
