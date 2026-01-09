pub enum Direction{
    Right,
    Left,
    Back,
    Front
}

pub struct Snake {
    pub body_cords:Vec<(usize, usize)>,
    direction:Direction
}

impl Snake {
    pub fn new(start_x: usize, start_y: usize) -> Self {
        let body_cords = vec![
            (start_x, start_y),
            (start_x - 1, start_y),
            (start_x - 2, start_y),
        ];

        Self {
            body_cords,
            direction: Direction::Right,
        }
    }

    pub fn next_position(&self) -> (usize,usize) {
        let x:usize = self.body_cords[0].0;
        let y:usize = self.body_cords[0].1;

        match self.direction {
            Direction::Right => (x.saturating_add(1), y), 
            Direction::Left => (x.saturating_sub(1), y), 
            Direction::Front => (x, y.saturating_add(1)), 
            Direction::Back => (x, y.saturating_sub(1)), 
        }
    }

    pub fn walk (&mut self, head_position:(usize, usize)) {
        self.body_cords.insert(0, head_position);
    }

    pub fn _tail_position(&self) -> Option<&(usize, usize)> {
        self.body_cords.last()
    }

    pub fn _head_position(&self) -> Option<&(usize, usize)> {
        self.body_cords.first()
    }

    pub fn remove_tail(&mut self) -> Option<(usize, usize)> {
        self.body_cords.pop()
    }

    pub fn set_direction(&mut self, dir:Direction){
        self.direction = dir;
    }


}
