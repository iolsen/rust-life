extern crate ncurses;
use ncurses::*;

mod board;
use board::Board;

fn main() {
    initscr();
    noecho();

    getch();

    let mut board = Board::init(40, 200);

    loop {
        board.print();
        let key = getch();
        if key == ('q' as i32) { break }
        board = board.tick();
    }

    endwin();
}
