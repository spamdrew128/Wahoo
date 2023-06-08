#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)] // remove later

mod attacks;
mod bench;
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
mod zobrist;
mod zobrist_stack;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        if arg == "bench" {
            bench::bench();
            return;
        }
    }

    std::env::set_var("RUST_BACKTRACE", "1");

    let mut uci_handler = uci::UciHandler::new();
    while matches!(uci_handler.execute_instructions(), uci::ProgramStatus::Run) {}
}
