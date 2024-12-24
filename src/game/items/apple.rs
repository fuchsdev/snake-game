use crate::game::{field::{self, FIELD_HEIGHT, FIELD_WIDTH}, item_handler::{self, ItemHandler}, snake::Snake};

#[derive(Copy, Clone)]
pub struct Apple {
    pub position: [i32; 2]
}

impl Apple {
    pub fn new(snake: &Snake) -> Self {
        Self {
            position: ItemHandler::find_free_space(&snake)
        }
    }
}