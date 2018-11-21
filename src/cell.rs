#[derive(Debug)]
pub struct Cell {
    x: i32,
    y: i32
}

impl Cell {
    pub fn create(x: i32, y: i32) -> Cell {
        Cell {x,y}
    }
    pub fn kill(self) {
    }
    pub fn println(&self) {
        println!("{:?}", self)
    }
}
