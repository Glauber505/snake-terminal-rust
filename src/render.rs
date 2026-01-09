use std::io::{stdout, Write};
use crossterm::{
    execute,
    cursor,
    terminal::{Clear, ClearType},
};

use crate::map::{Map, Item, WIDTH, HEIGHT};

pub fn render(map: &Map) {
    let mut out = stdout();

    execute!(
        out,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    ).unwrap();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ch = match map.get_cell(x, y) {
                Some(Item::Snake) => '█',
                Some(Item::Apple) => '🍎',
                Some(Item::Free) => '+',
                _ => ' ',
            };
            print!("{}", ch);
        }
        println!();
    }

    out.flush().unwrap();
}
