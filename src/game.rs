use rand::{Rng, rngs::ThreadRng};

use crate::{
    map::{HEIGHT, Item, Map, WIDTH}, 
    snake::{Snake}
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus{
    Running,
    GameOver
}

pub struct Game{
    pub map:Map,
    pub snake:Snake,
    pub status:GameStatus,
    rng:ThreadRng
}

impl Game{
    pub fn new(start_x:usize,start_y:usize) -> Self {
        let mut map = Map::new();
        let snake = Snake::new(start_x,start_y);

        for &(x, y) in &snake.body_cords {
            map.write_cell(x, y, Item::Snake);
        }

        let mut game = Self {
            map,
            snake,
            status: GameStatus::Running,
            rng: rand::thread_rng(),
        };

        game.create_apple();
        game
    }

    pub fn tick(&mut self){
        if self.status == GameStatus::GameOver  {
            return;
        }

        let head_position:(usize, usize) = self.snake.next_position();
        let x: usize = head_position.0;
        let y: usize = head_position.1;
        
        let item_head:Option<&Item> = self.map.get_cell(x, y);
        
        match item_head {
            Some(Item::Apple) => {
                self.eat(head_position);
                return;
            }
            Some(Item::Free) => {
                self.walk(head_position);
                return;
            }
            Some(Item::Snake) => {
                self.status = GameStatus::GameOver;
                return;
            } 
            None => {
                self.status = GameStatus::GameOver;
                return;
            } 
        }
    }

    fn eat(&mut self, head_position:(usize, usize)) {
        self.snake.walk(head_position);
        self.map.write_cell(head_position.0,head_position.1,Item::Snake);
        self.create_apple();
    }

    fn walk(&mut self, head_position:(usize, usize)) {
        self.snake.walk(head_position);
        let tail_cord = self.snake.remove_tail().unwrap();
        self.map.write_cell(head_position.0,head_position.1,Item::Snake);
        self.map.write_cell(tail_cord.0,tail_cord.1,Item::Free);
    }

    fn create_apple (&mut self,){
        for _ in 0..400{
            let apple_x: usize = self.rng.gen_range(0..=WIDTH);
            let apple_y: usize = self.rng.gen_range(0..=HEIGHT);

            if self.map.is_free(apple_x, apple_y){
                self.map.write_cell(apple_x,apple_y,Item::Apple);
                return;
            }
        }
        
        self.status = GameStatus::GameOver;
        
    }
}       