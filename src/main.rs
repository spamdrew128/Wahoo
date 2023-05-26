#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)] // remove later

mod board_representation;
mod chess_move;
mod lookup;
mod magic_table_builder;
mod util_macros;

fn main() {
    println!("Hello, world!");
}
