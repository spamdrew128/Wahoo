mod build_script_dependencies;
pub mod random_number_generation;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use build_script_dependencies::lmr_init::lmr_init_string;
use build_script_dependencies::magic_table_builder::magic_table_init_string;
use build_script_dependencies::trophism_init::trophism_table_init_string;
use build_script_dependencies::zobrist_init::zobrist_keys_init_string;

#[cfg(feature = "syzygy")]
fn build_fathom() {
    let cc = &mut cc::Build::new();
    cc.file("./3rdparty/fathom/src/tbprobe.c");
    cc.include("./3rdparty/fathom/src/");
    cc.define("_CRT_SECURE_NO_WARNINGS", None);
    cc.flag("-march=native");
    cc.flag("-w");

    // MSVC doesn't support stdatomic.h, so use clang on Windows
    if std::env::consts::OS == "windows" {
        cc.compiler("clang");
    }

    cc.compile("fathom");
}

#[cfg(feature = "syzygy")]
fn generate_fathom_bindings() {
    let bindings = bindgen::Builder::default()
        .header("./3rdparty/fathom/src/tbprobe.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .generate()
        .unwrap();

    bindings
        .write_to_file("./src/tablebase/bindings.rs")
        .unwrap();
}

fn build_syzygy_code() {
    #[cfg(feature = "syzygy")]
    build_fathom();
    #[cfg(feature = "syzygy")]
    generate_fathom_bindings();
}

fn gen_output_file(name: &str, generator: fn() -> String) {
    let mut out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    out_dir.push(name);

    let mut out_file = BufWriter::new(File::create(out_dir).unwrap());
    write!(&mut out_file, "{}", generator()).unwrap();
}

fn main() {
    gen_output_file("magic_lookup_init.rs", magic_table_init_string);
    gen_output_file("zobrist_keys_init.rs", zobrist_keys_init_string);
    gen_output_file("lmr_init.rs", lmr_init_string);
    gen_output_file("trophism_init.rs", trophism_table_init_string);
    build_syzygy_code();
}
