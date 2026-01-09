pub const HEIGHT: usize = 21;
pub const WIDTH: usize = 21;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Item{
    Apple,
    Snake,
    Free
}

pub struct Map {
    pub coords: [[Item; WIDTH]; HEIGHT]  
}

impl Map {
    pub fn new() -> Self {
        Self {
            coords: [[Item::Free; WIDTH]; HEIGHT],
        }
    }
    pub fn get_cell(&self, x:usize, y:usize) -> Option<&Item>{
        self.coords
            .get(x)
            .and_then(|row| row.get(y))
    }
    
    pub fn _is_snake(&self, x:usize, y:usize) -> bool {
        self.get_cell(x, y)
            .map_or(false, |&item| item == Item::Snake)
    }

    pub fn is_free(&self, x:usize, y:usize) -> bool {
        self.get_cell(x, y) 
            .map_or(false, |&item| item == Item::Free)
    }

    pub fn _is_apple(&self, x:usize, y:usize) -> bool {
        self.get_cell(x, y) 
            .map_or(false, |&item| item == Item::Apple)
    }

    fn get_mut_cell (&mut self, x:usize, y:usize) -> Option<&mut Item>{
        self.coords
            .get_mut(x)
            .and_then(|row| row.get_mut(y))
    
    }

    pub fn write_cell(&mut self, x:usize, y:usize, item:Item) {
        match self.get_mut_cell(x, y) {
            Some(cell_ref) => *cell_ref = item,
            None => {}
        }
    }
}
