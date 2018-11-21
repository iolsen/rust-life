extern crate rand;
use self::rand::seq::SliceRandom;

#[derive(Debug)]
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
            let cell = Cell::create(x, y);
            cells.push(cell);
        }
        Board { size, cells }
    }

    pub fn print(&self) {
        for y in (0..self.size).rev() {
            for x in 0..self.size {
                let mut found = false;
                for cell in self.cells.iter() {
                    if cell.x == x && cell.y == y {
                        print!(" X ");
                        found = true;
                        break;
                    }
                }
                if !found { print!(" • ") }
            }
            print!("\n");
        }
    }
}

#[derive(Debug)]
pub struct Cell {
    x: u16,
    y: u16
}

impl Cell {
    pub fn create(x: u16, y: u16) -> Cell {
        Cell {x,y}
    }
    pub fn _kill(self) {
    }
    pub fn _println(&self) {
        println!("{:?}", self);
    }
}
