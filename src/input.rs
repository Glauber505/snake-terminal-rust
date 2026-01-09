use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

use crate::snake::Direction;

pub fn read_input() -> Option<Direction> {
    if event::poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            return match key.code {
                KeyCode::Up    => Some(Direction::Back),
                KeyCode::Down  => Some(Direction::Front),
                KeyCode::Left  => Some(Direction::Left),
                KeyCode::Right => Some(Direction::Right),
                _ => None,
            };
        }
    }
    None
}
