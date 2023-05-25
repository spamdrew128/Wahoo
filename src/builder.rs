use crate::magic_table_builder::generate_magic_table;
use std::fs::File;
use std::io::prelude::*;

pub fn build_file() {
    let mut file = File::create("magic_table.rs").expect("couldn't create file");
    let contents = generate_magic_table();
    write!(&mut file, "{contents}").unwrap();
}
