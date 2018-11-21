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
            let x = *position % 10;
            let y = *position / 10;
            let cell = Cell { x, y };
            cells.insert(cell.hash_key(), cell);
        }
        Board { size, cells }
    }

    pub fn tick(&self) {
        let mut marked_for_death: Vec<&Cell> = Vec::new();
        for cell in self.cells.values() {
            let live_neighbor_count = self.living_neighbor_count(cell);
            if live_neighbor_count < 2 || live_neighbor_count > 3 {
                marked_for_death.push(cell);
            }
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
        for cell in self.cells.values() {
            cell.println(self);
        }
    }

    fn life_at(&self, cell: &Cell) -> bool {
        if self.cells.contains_key(&cell.hash_key()) {
            true
        } else {
            false
        }
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
            neighbors.push(Cell {x:n_x, y}); // left
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

    pub fn println(&self, board: &Board) {
        print! ("{:?} living neighbors: {}\r\n", self, board.living_neighbor_count(self))
    }
}
