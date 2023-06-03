#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)] // remove later

mod attacks;
mod board_representation;
mod chess_move;
mod evaluation;
mod magic;
mod movegen;
mod perft;
mod search;
mod time_management;
mod uci;
mod util_macros;

fn main() {
    let mut uci_handler = uci::UciHandler::new();
    std::env::set_var("RUST_BACKTRACE", "1");
    while matches!(uci_handler.execute_instructions(), uci::ProgramStatus::Run) {}
}
