use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};



pub struct GameState {
    frame_count: usize
}



impl GameState {

    pub fn new() -> Self {
        Self {
            frame_count: 0
        }
    }


    pub fn update(&mut self, draw_handle: &mut RaylibDrawHandle) {

        self.frame_count += 1;
        println!("{}", self.frame_count);
    }

}
