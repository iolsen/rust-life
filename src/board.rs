extern crate rand;
use self::rand::seq::SliceRandom;

pub struct Board {
    size: u16,
    cells: Vec<Cell>
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

        let mut cells: Vec<Cell> = Vec::new();
        for position in initial_positions.iter()  {
            let x = *position % 10;
            let y = *position / 10;
            let cell = Cell { x, y };
            cells.push(cell);
        }
        Board { size, cells }
    }

    pub fn tick(&self) {
        let mut marked_for_death: Vec<&Cell> = Vec::new();
        for cell in self.cells.iter() {
            let live_neighbor_count = self.living_neighbor_count(cell);
            if live_neighbor_count < 2 || live_neighbor_count > 3 {
                marked_for_death.push(cell);
            }
        }
    }

    pub fn print(&self) {
        for y in (0..self.size).rev() {
            for x in 0..self.size {
                if self.life_at(x, y) {
                    print!("⚪️");
                } else {
                    print!("⚫️");
                }
            }
            print!("\r\n");
        }
    }

    fn life_at(&self, x: u16, y: u16) -> bool {
        for cell in self.cells.iter() {
            if cell.x == x && cell.y == y {
                return true;
            }
        }
        false
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
            if self.life_at(cell.x, cell.y) { count += 1 }
        }
        count
    }
}

#[derive(Debug)]
pub struct Cell {
    x: u16,
    y: u16
}
