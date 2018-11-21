extern crate ncurses;
use ncurses::*;

mod board;
use board::Board;

fn main() {

    initscr();
    getch();

    let board = Board::init(10, 10);

    loop {
        board.print();
        let key = getch();
        if key == ('q' as i32) { break }
        board.tick();
    }

    endwin();
}
