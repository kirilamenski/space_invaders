use super::game_object_trait::GameObjectTrait;
use termion::raw::RawTerminal;
use std::io::{Write, StdoutLock};
use termion::{clear, cursor};

#[derive(Debug)]
pub enum BulletDirection {
    Up,
    Down,
}

pub struct Bullet {
    x: u16,
    y: u16,
    model: String,
    is_exists: bool,
    direction: BulletDirection,
}

impl Bullet {
    pub fn new(x: u16, y: u16, model: String, direction: BulletDirection) -> Bullet {
        Bullet {
            x,
            y,
            model,
            is_exists: true,
            direction,
        }
    }

    pub fn get_direction(&self) -> &BulletDirection {
        &self.direction
    }
}

impl GameObjectTrait for Bullet {
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
        self.is_exists
    }

    fn destroy(&mut self) {
        self.is_exists = false;
        self.model = " ".to_string();
    }

    fn draw(&self, stdout: &mut RawTerminal<StdoutLock>) {
        write!(stdout, "{}", cursor::Goto(self.x, self.y));
        stdout.write(self.get_model_bytes());
        match self.get_direction() {
            BulletDirection::Up => {
                write!(stdout, "{}", cursor::Goto(self.x, self.y + 1));
            }
            BulletDirection::Down => {
                write!(stdout, "{}", cursor::Goto(self.x, self.y - 1));
            }
        }
        stdout.write(" ".as_bytes());
    }
}