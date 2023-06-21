mod tuning;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name: &str = args.get(1).expect("expected data file to read!");

    let mut tuner = tuning::Tuner::new(false);
    tuner.load_from_file(file_name);
    tuner.train();
}
