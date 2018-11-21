extern crate rand;
use self::rand::seq::SliceRandom;

use std::collections::HashMap;

pub struct Board {
    size: u16,
    cells: HashMap<String, Cell>
}

impl Board {
    pub fn init(size: u16, count: u16) -> Board {
        let possible_positions = size.pow(2);
        if count > possible_positions {
            panic!("too many cells for that board size");
        }

        let mut rng = &mut rand::thread_rng();
        let all_positions: Vec<u16>= (0..possible_positions).collect();
        let initial_positions: Vec<&u16> =
            all_positions.choose_multiple(&mut rng, count as usize).collect();

        let mut cells: HashMap<String, Cell> = HashMap::new();
        for position in initial_positions.iter()  {
            let x = *position % size;
            let y = *position / size;
            let cell = Cell { x, y };
            cells.insert(cell.hash_key(), cell);
        }
        Board { size, cells }
    }

    pub fn tick(self) -> Board {
        let mut new_cells: HashMap<String, Cell> = HashMap::new();

        for y in (0..self.size).rev() {
            for x in 0..self.size {
                let cell = Cell {x, y};
                let live_neighbor_count = self.living_neighbor_count(&cell);
                // live cells continue living with 2 or 3 neighbors
                // dead cells come alive with 3 neighbors
                if live_neighbor_count == 2 && self.life_at(&cell) {
                    new_cells.insert(cell.hash_key(), cell);
                } else if live_neighbor_count == 3 {
                    new_cells.insert(cell.hash_key(), cell);
                }
            }
        }

        Board {
            size: self.size,
            cells: new_cells
        }
    }

    pub fn print(&self) {
        for y in (0..self.size).rev() {
            for x in 0..self.size {
                if self.life_at(&Cell {x, y}) {
                    print!("⚪️");
                } else {
                    print!("⚫️");
                }
            }
            print!("\r\n");
        }
        print!("\r\n");
        // for cell in self.cells.values() {
        //     cell._println(self);
        // }
    }

    fn life_at(&self, cell: &Cell) -> bool {
        self.cells.contains_key(&cell.hash_key())
    }

    fn neighbors_of(&self, cell: &Cell) -> Vec<Cell> {
        let mut neighbors: Vec<Cell> = Vec::new();
        let x = cell.x;
        let y = cell.y;

        // cells to the left
        if x > 0 {
            let n_x = x-1;
            if y < self.size - 1 { neighbors.push(Cell {x:n_x, y:y+1}) } // above left
            neighbors.push(Cell {x:n_x, y}); // left
            if y > 0 { neighbors.push(Cell {x:n_x, y:y-1}) } // below left
        }

        if y < self.size - 1 { neighbors.push(Cell {x, y:y+1}) } // above
        if y > 0 { neighbors.push(Cell {x, y:y-1}) } // below

        // cells to the right
        if x < self.size - 1 {
            let n_x = x+1;
            if y < self.size - 1 { neighbors.push(Cell {x:n_x, y:y+1}) } // above right
            neighbors.push(Cell {x:n_x, y}); // right
            if y > 0 { neighbors.push(Cell {x:n_x, y:y-1}) } // below right
        }
        neighbors
    }

    fn living_neighbor_count(&self, cell: &Cell) -> u8 {
        let mut count = 0;
        let neighbors = self.neighbors_of(cell);
        for cell in neighbors.iter() {
            if self.life_at(cell) { count += 1 }
        }
        count
    }
}

#[derive(Debug)]
pub struct Cell {
    x: u16,
    y: u16
}

impl Cell {
    pub fn hash_key(&self) -> String {
        let mut key = String::with_capacity(10);
        key.push_str(&self.x.to_string());
        key.push(',');
        key.push_str(&self.y.to_string());
        key
    }

    pub fn _println(&self, board: &Board) {
        print! ("{:?} living neighbors: {}\r\n", self, board.living_neighbor_count(self))
    }
}
