mod board;
use board::Board;

fn main() {
    // let cell = board::Cell::create(0,0);
    // cell.println();
    // cell.kill();

    let board = Board::init(10, 10);
    board.print();
}
