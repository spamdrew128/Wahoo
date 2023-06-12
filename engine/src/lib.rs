#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_lines)]
#![allow(dead_code)] // remove later

mod attacks;
pub mod bench;
mod board_representation;
mod chess_move;
mod evaluation;
mod magic;
mod movegen;
mod perft;
mod pv_table;
mod search;
mod time_management;
pub mod uci;
mod util_macros;
mod zobrist;
mod zobrist_stack;