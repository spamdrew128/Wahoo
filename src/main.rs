#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)] // remove later

mod attacks;
mod board_representation;
mod chess_move;
mod magic;
mod movegen;
mod perft;
mod util_macros;

fn main() {
    use crate::perft::*;

    std::env::set_var("RUST_BACKTRACE", "1");
    split_perft("r3k2r/8/8/8/8/8/8/2KR3R b kq - 1 1", 1);
    // run_test_suite();
}
