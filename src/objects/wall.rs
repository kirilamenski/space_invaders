use super::game_object_trait::GameObjectTrait;
use termion::raw::RawTerminal;
use std::io::{Write, StdoutLock};
use termion::{clear, cursor};

#[derive(Clone, Debug)]
pub struct Wall {
    x: u16,
    y: u16,
    lives: u16,
    model: String,
}

impl Wall {
    pub fn new(x: u16, y: u16, lives: u16, model: String) -> Wall {
        Wall {
            x,
            y,
            lives,
            model,
        }
    }
}

impl GameObjectTrait for Wall {
    fn get_position(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    fn get_model_bytes(&self) -> &[u8] {
        self.model.as_bytes()
    }

    fn move_object(&mut self, x: u16, y: u16) {}

    fn get_size(&self) -> (u16, u16, u16, u16) {
        (self.x, self.y, self.model.len() as u16, 1)
    }

    fn is_alive(&self) -> bool {
        self.lives > 0
    }

    fn destroy(&mut self) {
        self.lives -= 1;
        if self.lives == 0 {
            self.model = " ".to_string();
        }
    }

    fn draw(&self, stdout: &mut RawTerminal<StdoutLock>) {
        write!(stdout, "{}", cursor::Goto(self.x, self.y));
        stdout.write(self.get_model_bytes());
    }
}