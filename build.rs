mod build_dependencies;

use crate::build_dependencies::magic_table_builder::generate_magic_table;
fn main() {
    let contents = generate_magic_table();
    println!("Hello, world!");
}