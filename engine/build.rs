mod build_script_dependencies;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use build_script_dependencies::magic_table_builder::magic_table_init_string;
use build_script_dependencies::zobrist_init::zobrist_keys_init_string;

fn gen_output_file(name: &str, generator: fn() -> String) {
    let mut out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    out_dir.push(name);

    let mut out_file = BufWriter::new(File::create(out_dir).unwrap());
    write!(&mut out_file, "{}", generator()).unwrap();
}

fn main() {
    gen_output_file("magic_lookup_init.rs", magic_table_init_string);
    gen_output_file("zobrist_keys_init.rs", zobrist_keys_init_string);
}
