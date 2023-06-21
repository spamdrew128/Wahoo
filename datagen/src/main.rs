use engine::search::SearchLimit;

mod datagen;
mod rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name: &str = match args.get(1) {
        Some(s) => s.as_str(),
        None => "data.txt",
    };

    let mut data_generator = datagen::DataGenerator::new(SearchLimit::Nodes(100_000), file_name);
    data_generator.generate_data(5000);
}
