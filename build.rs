mod build_script_dependencies;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use crate::build_script_dependencies::magic_table_builder::generate_magic_table;

fn main() {
    let mut out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    out_dir.push("magic_table.txt");

    let mut out_file = BufWriter::new(File::create(out_dir).unwrap());
    write!(&mut out_file, "{}", generate_magic_table()).unwrap();
}
