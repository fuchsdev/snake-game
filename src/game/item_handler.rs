use rand::Rng;
use raylib::{color::{self, Color}, prelude::{RaylibDraw, RaylibDrawHandle}};
use super::{apple, field::{BOX_HEIGHT, BOX_WHIDTH, FIELD_WIDTH}, snake::{self, Snake}, Apple};

pub struct ItemHandler {
    apple: Apple
}

impl ItemHandler {
    pub fn new(snake: &Snake) -> Self {
        Self {
            apple: Apple::new(snake)
        }
    }

    fn is_free(snake: &Snake, x: i32, y: i32) -> bool {
        for s in 0..snake.boxes.len() {
            while snake.boxes[s][0] == x && snake.boxes[s][1] == y {
                return false;
            }
        }

        true
    }

    pub fn find_free_space(snake: &Snake) -> [i32; 2] {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0..19);
        let mut y = rng.gen_range(0..19);

        while !Self::is_free(snake, x, y) {
            x = rng.gen_range(0..19);
            y = rng.gen_range(0..19);
        }

        [x, y]
    }

    pub fn check_for_item(&mut self, snake: &mut Snake) {
        if snake.boxes[0] == self.apple.position{
            self.apple = Apple::new(snake);
            snake.boxes.push(snake.boxes[snake.boxes.len() - 1]);
        }
    }

    pub fn draw_items(&mut self, snake: &mut Snake, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_circle((self.apple.position[0] * BOX_WHIDTH) + (BOX_WHIDTH / 2), 
                                (self.apple.position[1] * BOX_HEIGHT) + (BOX_WHIDTH / 2), 
                                BOX_WHIDTH as f32 / 2.0, Color::RED);
    }
}