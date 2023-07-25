#![allow(clippy::needless_range_loop)]

mod tuner_val;
mod tuning;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name: &str = args.get(1).expect("expected data file to read!");
    let threads: usize = args.get(2).map_or_else(|| 1, |x| x.parse::<usize>().unwrap_or(1));

    let mut tuner = tuning::Tuner::new(threads);
    tuner.load_from_file(file_name);
    tuner.train();
}
