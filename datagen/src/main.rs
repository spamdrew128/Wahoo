use engine::search::SearchLimit;

mod datagen;
mod rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name: &str = match args.get(1) {
        Some(s) => s.as_str(),
        None => "data.txt",
    };

    let search_limits = vec![SearchLimit::Nodes(150_000), SearchLimit::Time(1000)];
    let mut data_generator = datagen::DataGenerator::new(search_limits, file_name);
    data_generator.generate_data(7500);
}
