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
mod uci;
mod util_macros;

fn main() {
    let uci_handler = uci::UciHandler::new();

    while matches!(uci_handler.execute_instructions(), uci::ProgramStatus::Run) {}
}
