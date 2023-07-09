#![allow(clippy::needless_range_loop)]

mod tuning;
mod prev_weights;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name: &str = args.get(1).expect("expected data file to read!");

    let mut tuner = tuning::Tuner::new(true);
    tuner.load_from_file(file_name);
    tuner.train();
}
