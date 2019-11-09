extern crate termion;

mod game;
mod objects;

use termion::{clear, cursor};
use termion::async_stdin;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::TermRead;
use termion::event::Key;
use std::io::{Write, stdout, stdin, StdinLock, StdoutLock, Read};
use std::time::{Instant, Duration};
use self::game::*;
use std::thread::sleep;

const WELCOME_MSG: &str =
    "======================\n\r\
        Welcome to Gaminal \n\r\
        | q | quit\n\r\
        | s | start\n\r\
        | r | restart\n\r\
        | p | pause\n\r\
        | z, x | left and right\n\r\
        ======================";
const WIDTH: u16 = 100;
const HEIGHT: u16 = 30;

fn main() {
    let mut game = Game::new(WIDTH, HEIGHT);
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let stdout = stdout();
    let mut stdout = stdout.lock()
                           .into_raw_mode()
                           .unwrap();

    write!(stdout, "{}{}{}", clear::All, cursor::Hide, cursor::Goto(1, 1));
    stdout.write(WELCOME_MSG.as_bytes());
    stdout.flush().unwrap();

    let mut start = Instant::now();
    let mut before = Instant::now();
    let mut input = async_stdin().bytes();
    let interval = 1000 / game.get_speed();
    loop {
        let now = Instant::now();
        let dt = (now.duration_since(before).subsec_nanos() / 1_000_000) as u64;
        if dt < interval {
            sleep(Duration::from_millis(interval - dt));
            continue;
        }

        let input_option = input.next();
        if game.is_started() {
            let delta = start.elapsed().as_millis();
            let fps: i32 = (1000000000 / (now.duration_since(before).subsec_nanos())) as i32;
            game.draw(&mut stdout, fps);
            game.update(delta);
            game.key_pressed(&input_option);

            if delta >= 1500 {
                start = now;
            }
        }
        before = now;

        match input_option {
            Some(Ok(b'q')) => {
                write!(stdout, "{}", clear::All);
                break;
            }
            Some(Ok(b's')) => {
                write!(stdout, "{}", clear::All);
                game.set_start(true);
                game.start(&mut stdout)
            }
            Some(Ok(b'p')) => {
                let is_started = game.is_started();
                game.set_start(!is_started);
            }
            Some(Ok(b'y')) | Some(Ok(b'r')) => {
                if game.is_started() {
                    write!(stdout, "{}", clear::All);
                    game.restart(&mut stdout);
                }
            }
            Some(Ok(b'n')) => {
                if game.is_started() {
                    write!(stdout, "{}", clear::All);
                    break;
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
