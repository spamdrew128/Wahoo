mod build_script_dependencies;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use build_script_dependencies::zobrist_init::table_init_string;
use build_script_dependencies::magic_table_builder::generate_magic_table;

fn gen_magic_table() {
    let mut out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    out_dir.push("magic_lookup_init.txt");

    let mut out_file = BufWriter::new(File::create(out_dir).unwrap());
    write!(&mut out_file, "{}", generate_magic_table()).unwrap();
}

fn gen_zobrist_table() {
    let mut out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    out_dir.push("zobrist_keys_init.txt");

    let mut out_file = BufWriter::new(File::create(out_dir).unwrap());
    write!(&mut out_file, "{}", table_init_string()).unwrap();
}

fn main() {
    gen_magic_table();
    gen_zobrist_table();
}
