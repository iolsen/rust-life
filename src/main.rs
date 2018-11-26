extern crate getch;
use getch::Getch;

mod board;
use board::Board;

fn main() {
    let mut board = Board::init(40, 200);
    let getch = Getch::new();

    loop {
        board.print();
        let key = getch.getch().expect("Failed to read a key");
        if key == ('q' as u8) { break }
        board = board.tick();
    }
}
