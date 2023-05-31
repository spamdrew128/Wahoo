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
    use crate::perft::split_perft;
    use crate::board_representation::START_FEN;

    split_perft(START_FEN, 2);
}
