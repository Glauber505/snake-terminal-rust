use std::{thread, time::Duration};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crate::{
    game::Game,
    game::GameStatus,
    input::read_input,
    render::render
};

mod game;
mod snake;
mod map;
mod input;
mod render;

fn main() {
    enable_raw_mode().unwrap(); // modo raw do terminal

    let mut game = Game::new(10,11);

    loop {
        if game.status == GameStatus::GameOver {
            break;
        }

        if let Some(dir) = read_input() {
            game.snake.set_direction(dir);
        }

        game.tick();
        render(&game.map);

        thread::sleep(Duration::from_millis(150));
    }

    disable_raw_mode().unwrap();
}
