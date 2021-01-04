mod piece;
mod board;
use crate::board::Board;


fn main() {
    let mut main_board = Board;
    main_board::init_board();
    main_board::print_board();

    println!("Hello, world!");
}
