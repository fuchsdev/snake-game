use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};

use super::{apple, field::{self, BOX_HEIGHT, BOX_WHIDTH}, game};

pub struct Snake {
    pub boxes: Vec<[i32; 2]>,
    pub direction: Direction
}

impl Snake {

    pub fn new() -> Self {
        Self {
            boxes: vec![[9, 9], [8, 9], [7, 9]],
            direction: Direction::Right
        }
    }

    pub fn move_snake(&mut self) {

        for s in (1..self.boxes.len()).rev() {
            self.boxes[s] = self.boxes[s - 1];
        }
        
        if self.direction == Direction::Right {
            self.boxes[0][0] += 1;
        }
        else if self.direction == Direction::Left {
            self.boxes[0][0] -= 1;
        }
        else if self.direction == Direction::Up {
            self.boxes[0][1] -= 1;
        }
        else if self.direction == Direction::Down {
            self.boxes[0][1] += 1;
        }
    }

    pub fn draw_snake(&mut self, draw_handle: &mut RaylibDrawHandle) {
        for s in 0..self.boxes.len() {
            if s == 0 {
                draw_handle.draw_rectangle(
                    self.boxes[s][0] * BOX_WHIDTH, self.boxes[s][1] * BOX_HEIGHT,
                    BOX_WHIDTH, BOX_WHIDTH, Color::new(100, 255, 0, 255));
            }
            else{
                let box_color: f32 = 240.0 - ((s as f32 / self.boxes.len() as f32) * (200.0));
                draw_handle.draw_rectangle(
                    self.boxes[s][0] * BOX_WHIDTH, self.boxes[s][1] * BOX_HEIGHT,
                    BOX_WHIDTH, BOX_WHIDTH, Color::new(0, box_color as u8, 0, 255));
            }
        }
    }

    pub fn check_game_over(&mut self,  field_width: i32, field_height: i32) -> bool{
        for s in 1..self.boxes.len() {
            if self.boxes[0] == self.boxes[s] {
                return true;
            }
        }
        if self.boxes[0][0] >=  field_width || self.boxes[0][0] <  0 || self.boxes[0][1] <  0
            || self.boxes[0][1] >= field_height {
                return true;
        } 
        else {
            false
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}



