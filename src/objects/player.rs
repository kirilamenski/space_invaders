use super::game_object_trait::GameObjectTrait;
use termion::raw::RawTerminal;
use std::io::{Write, StdoutLock};
use termion::{clear, cursor};

#[derive(Clone)]
pub struct Player {
    x: u16,
    y: u16,
    live: usize,
    model: String,
}

impl Player {
    pub fn new(x: u16, y: u16, model: String) -> Player {
        Player {
            x,
            y,
            live: 3,
            model,
        }
    }

    pub fn get_lives(&self) -> usize {
        self.live
    }
}

impl GameObjectTrait for Player {
    fn get_position(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    fn get_model_bytes(&self) -> &[u8] {
        self.model.as_bytes()
    }

    fn move_object(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    fn get_size(&self) -> (u16, u16, u16, u16) {
        (self.x, self.y, self.model.len() as u16, 1)
    }

    fn is_alive(&self) -> bool {
        self.live > 0
    }

    fn destroy(&mut self) {
        if self.live > 0 {
            self.live -= 1;
            let model = match self.live {
                2 => " ==",
                1 => " = ",
                _ => "   "
            }.to_string();
            self.model = model;
        }
    }

    fn draw(&self, stdout: &mut RawTerminal<StdoutLock>) {
        write!(stdout, " {} ", cursor::Goto(self.x, self.y));
        stdout.write(self.get_model_bytes());
    }
}


