use super::game_object_trait::GameObjectTrait;
use termion::raw::RawTerminal;
use std::io::{Write, StdoutLock};
use termion::{clear, cursor};

#[derive(Clone, Debug)]
pub struct Enemy {
    x: u16,
    y: u16,
    live: u16,
    model: String,
    is_alive: bool,
    line: u16,
}

impl Enemy {
    pub fn new(x: u16, y: u16, line: u16, model: String) -> Enemy {
        Enemy {
            x,
            y,
            live: 1,
            model,
            is_alive: true,
            line,
        }
    }

    pub fn get_line(&self) -> u16 {
        self.line
    }
}

impl GameObjectTrait for Enemy {
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
        self.is_alive
    }

    fn destroy(&mut self) {
        self.is_alive = false;
        self.model = "".to_string();
    }

    fn draw(&self, stdout: &mut RawTerminal<StdoutLock>) {
        if self.y >= 3 {
            write!(stdout, " {} ", cursor::Goto(self.x, self.y));
            stdout.write(self.get_model_bytes());
            // TODO dirty hack lets find the way to update it correctly!
            if self.get_line() == 0 {
                write!(stdout, " {} ", cursor::Goto(self.x, self.y - 1));
                stdout.write(" ".as_bytes());
            }
        }
    }
}