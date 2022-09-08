use crate::event::ImageLoadEvent;
use crate::model::home::ContentSet;
use crate::ui::shelf::Shelf;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

/// The main screen for the application
/// The root of the application
pub struct HomePage {
    shelf_padding: u32,
    tile_padding: u32,
    shelves: Vec<Shelf>,
    selected_row: usize,
    shelf_height: u32,
}
impl HomePage {

    /// Loads a home page component, but does 
    /// not do any rendering
    pub fn load(
        content_sets: Vec<ContentSet>,
        shelf_padding: u32,
        shelf_height: u32,
        tile_padding: u32,
    ) -> HomePage {
        // the parent is responsible for positioning the children
        // here, we place each new shelf below the other
        let mut home_page = HomePage {
            shelf_padding,
            tile_padding,
            shelves: Vec::new(),
            selected_row: 0,
            shelf_height,
        };
        home_page.load_shelves(content_sets);
        home_page
    }

    /// Load the shelves onto the home screen
    fn load_shelves(&mut self, content_sets: Vec<ContentSet>) {
        // Maintain proper positioning on create
        // Each shelf should initially be before the one below it
        let mut y_pos = self.shelf_padding as i32;
        for cs in content_sets {
            let next_shelf = Shelf::load(cs, self.shelf_height, self.tile_padding, y_pos);
            y_pos = next_shelf.y() + next_shelf.height() as i32 + self.shelf_padding as i32;
            self.shelves.push(next_shelf);
        }

        // select the first row so that when the
        // screen appears we should be ready to go
        if let Some(shelf) = self.shelves.get_mut(0) {
            shelf.select()
        };
    }

    /// Re-render whatever is needed
    pub fn draw(&mut self, font: &Font, canvas: &mut Canvas<Window>) {
        self.shelves.iter_mut().for_each(|s| s.draw(font, canvas));
    }

    /// Process an image load event, find the tile and
    /// send away
    pub fn on_image_load(&mut self, event: ImageLoadEvent) {
        let maybe_found_row = self
            .shelves
            .iter_mut()
            .find(|row| *row.title() == event.content_set_title);
        if let Some(found_row) = maybe_found_row {
            found_row.on_image_load(event);
        } else {
            println!(
                "DID NOT FIND ROW FOR EVENT IMG {:?}",
                event.content_set_title
            );
        }
    }

    pub fn on_key_down(&mut self) {
        // when going down, we have to subtract the height from all shelves, forcing some to go negative
        // unless the current selection is the last row
        let current_selection = self.selected_row;
        if current_selection < self.shelves.len() - 1 {
            // only slide if we are past the 2nd row
            // otherwise, leave the rows alone

            // unselect the current selection
            self.shelves[current_selection].unselect();
            if current_selection > 0 {
                for shelf in self.shelves.iter_mut() {
                    // move the shelves up (scroll down)]
                    // by shifting the y position for each shelf up
                    shelf.set_y(shelf.y() - shelf.height() as i32 - self.shelf_padding as i32);
                }
            }
            self.selected_row += 1;
            self.shelves[self.selected_row].select();
        }
    }

    pub fn on_key_up(&mut self) {
        // when scrolling up, we do not scroll past 0
        let current_selection = self.selected_row;
        if current_selection > 0 {
            self.shelves[current_selection].unselect();
            // only shift up if we aren't going to 0
            if current_selection >= 2 {
                for shelf in self.shelves.iter_mut() {
                    shelf.set_y(shelf.y() + shelf.height() as i32 + self.shelf_padding as i32);
                }
            }
            self.selected_row -= 1;
            self.shelves[self.selected_row].select();
        }
    }

    pub fn on_key_right(&mut self) {
        // advance shelf to the next tile
        self.shelves[self.selected_row].on_key_right();
    }

    pub fn on_key_left(&mut self) {
        // scroll the shelf to the previous tile
        self.shelves[self.selected_row].on_key_left();
    }
}
