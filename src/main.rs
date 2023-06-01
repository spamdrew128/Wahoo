#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)] // remove later

use crate::{board_representation::Board, chess_move::Move};

mod attacks;
mod board_representation;
mod chess_move;
mod magic;
mod movegen;
mod perft;
mod util_macros;

fn main() {
    use crate::perft::*;
    use crate::chess_move::*;

    std::env::set_var("RUST_BACKTRACE", "1");
    split_perft("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", 2);
    // split_perft("r3k2r/8/8/8/8/8/8/2KR3R b kq - 1 1", 1);
    // run_test_suite();

    // let board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
    // let mv = Move::new_qs_castle(board.king_sq());
    // let board = board.try_play_move(mv).unwrap();
    // board.print();
}
