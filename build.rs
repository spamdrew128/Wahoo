mod build_script_dependencies;

use crate::build_script_dependencies::magic_table_builder::generate_magic_table;
fn main() {
    let contents = generate_magic_table();
    println!("Hello, world!");
}