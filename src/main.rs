mod cell;

fn main() {
    let cell = cell::Cell::create(0,0);
    cell.println();
    cell.kill();
}
