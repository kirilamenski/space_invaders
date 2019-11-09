use termion::{clear, cursor};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::TermRead;
use termion::event::Key;
use termion::cursor::Goto;
use rand::{thread_rng, Rng};
use std::io::{Write, stdout, StdoutLock};
use std::mem;
use std::io::Error;
use super::objects::game_object_trait::GameObjectTrait;
use super::objects::bullet::{BulletDirection, Bullet};
use super::objects::player::Player;
use super::objects::enemy::Enemy;
use super::objects::wall::Wall;
use std::borrow::BorrowMut;

const GAME_OVER: &str = "Game Over! Want to try again? Y/N";

pub struct Game {
    width: u16,
    height: u16,
    game_over: bool,
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    walls: Vec<Wall>,
    is_started: bool,
    speed: u64,
    iter: i32,
    enemies_size: usize,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Game {
        Game {
            width,
            height,
            game_over: false,
            player: Player::new(width / 2 - 1, height - 1, "===".to_string()),
            enemies: Vec::new(),
            bullets: Vec::new(),
            walls: Vec::new(),
            is_started: false,
            speed: 120,
            iter: 0,
            enemies_size: 0,
        }
    }

    pub fn get_speed(&self) -> u64 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: u64) {
        self.speed = speed;
    }

    pub fn set_start(&mut self, is_started: bool) {
        self.is_started = is_started;
    }

    pub fn restart(&mut self, stdout: &mut RawTerminal<StdoutLock>) {
        self.enemies.clear();
        self.bullets.clear();
        self.game_over = false;
        self.player = Player::new(self.width / 2 - 1, self.height - 1, "===".to_string());
        self.iter = 0;
        self.start(stdout)
    }

    pub fn start(&mut self, stdout: &mut RawTerminal<StdoutLock>) {
        self.create_enemies();
        self.create_window_bounds(stdout);
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }

    pub fn update(&mut self, dt: u128) {
        if dt >= 1500 {
            self.iter += 1;
            self.update_enemies();
            self.create_player_bullet();
            self.create_enemies_bullets();
            if self.iter == 10 {
                self.iter = 0;
            }
        }
        self.update_bullets();
    }

    pub fn draw(&mut self, stdout: &mut RawTerminal<StdoutLock>, fps: i32) {
        self.draw_player(stdout);
        self.draw_enemies(stdout);
        self.draw_walls(stdout);
        self.draw_bullets(stdout);

        self.bullets.retain(|mut bullet| {
            bullet.is_alive()
        });
        self.walls.retain(|mut wall| {
            wall.is_alive()
        });

        write!(stdout, "{}", cursor::Goto(1, self.height + 5));
        stdout.write(format!(
            "Enemies left: {}, Total mem: {}, FPS: {}, lives: {}",
            self.enemies_size,
            mem::size_of_val(self),
            fps,
            self.player.get_lives()
        ).as_bytes());

        if self.game_over {
            self.draw_message(GAME_OVER, stdout);
        }
    }

    pub fn key_pressed(&mut self, key: &Option<Result<u8, Error>>) {
        let (x, y) = self.player.get_position();
        match key {
            Some(Ok(b'z')) => {
                if x > 3 {
                    self.player.move_object(x - 1, y);
                }
            }
            Some(Ok(b'x')) => {
                if x < self.width - 3 {
                    self.player.move_object(x + 1, y);
                }
            }
            _ => {}
        }
    }

    fn create_enemies(&mut self) {
        let center_point = self.width / 2 - 30;
        for i in 0..6 {
            for j in center_point..center_point * 4 {
                if (j + i) % 3 == 0 {
                    let mut enemy = Enemy::new( j, i, i, "֎ ".to_string());
                    self.enemies.push(enemy);
                }
            }
        }
        self.enemies_size = self.enemies.len();
    }

    fn create_player_bullet(&mut self) {
        if self.player.is_alive() {
            let (x, y) = self.player.get_position();
            let bullet = Bullet::new(
                x + 2,
                y - 1,
                "|".to_string(),
                BulletDirection::Up,
            );
            self.bullets.push(bullet);
        }
    }

    fn create_enemies_bullets(&mut self) {
        let mut rand = thread_rng();
        let random_index = rand.gen_range(0, self.enemies.len() - 1) as usize;
        let enemy = &mut self.enemies[random_index];
        let (x, y) = enemy.get_position();
        if y >= 3 && enemy.is_alive() {
            let bullet = Bullet::new(
                x,
                y + 1,
                "|".to_string(),
                BulletDirection::Down,
            );
            self.bullets.push(bullet);
        } else {
            self.create_enemies_bullets();
        }
    }

    fn create_window_bounds(&mut self, stdout: &mut RawTerminal<StdoutLock>) {
        for x in 2..self.width {
            let top_wall = Wall::new(x, 1, 1, "=".to_string());
            top_wall.draw(stdout);
            let bottom_wall = Wall::new(x, self.height, 1, "=".to_string());
            bottom_wall.draw(stdout);
            let block = Wall::new(x, self.height - 5, 2, "=".to_string());
            self.walls.push(block);
        }
        for y in 0..self.height {
            let left_wall = Wall::new(2, y, 1, "ǁ".to_string());
            left_wall.draw(stdout);
            let right_wall = Wall::new(self.width, y, 1, "ǁ".to_string());
            right_wall.draw(stdout);
        }
    }

    fn update_bullets(&mut self) {
        for bullet in &mut self.bullets {
            let (x, y) = bullet.get_position();
            match bullet.get_direction() {
                BulletDirection::Up => {
                    if y >= 3 {
                        bullet.move_object(x, y - 1);
                    } else {
                        bullet.destroy()
                    }
                    if bullet.is_alive() {
                        for enemy in &mut self.enemies {
                            if enemy.is_alive() && bullet.is_collide(enemy) {
                                enemy.destroy();
                                bullet.destroy();
                                self.enemies_size -= 1;
                            }
                        }
                    }
                }
                BulletDirection::Down => {
                    if y > self.height - 2 {
                        bullet.destroy();
                    } else {
                        bullet.move_object(x, y + 1);
                        if bullet.is_collide(&self.player) {
                            bullet.destroy();
                            self.player.destroy();
                            self.game_over = !self.player.is_alive()
                        }
                    }
                }
                _ => {}
            }
            for wall in &mut self.walls {
                if bullet.is_collide(wall) {
                    bullet.destroy();
                    wall.destroy();
                }
            }
        }
    }

    fn update_enemies(&mut self) {
        for enemy in &mut self.enemies {
            let (mut x, mut y) = enemy.get_position();
            match self.iter {
                10 => y += 1,
                0...4 => x -= 1,
                5...8 => x += 1,
                _ => {}
            }
            enemy.move_object(x, y);
        }
    }

    fn draw_player(&self, stdout: &mut RawTerminal<StdoutLock>) {
        self.player.draw(stdout)
    }

    fn draw_enemies(&mut self, stdout: &mut RawTerminal<StdoutLock>) {
        for enemy in &mut self.enemies {
            enemy.draw(stdout)
        }
    }

    fn draw_bullets(&mut self, stdout: &mut RawTerminal<StdoutLock>) {
        for bullet in &mut self.bullets {
            bullet.draw(stdout);
        }
    }

    fn draw_walls(&mut self, stdout: &mut RawTerminal<StdoutLock>) {
        for wall in &mut self.walls {
            wall.draw(stdout);
        }
    }

    fn draw_message(&self, message: &str, stdout: &mut RawTerminal<StdoutLock>) {
        let x = self.width / 2 - (message.len() / 2) as u16;
        let y = self.height / 2;
        write!(stdout, "{}", cursor::Goto(x, y));
        stdout.write(message.as_bytes());
    }
}