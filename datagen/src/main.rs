use engine::search::SearchLimit;

mod datagen;
mod rng;

fn main() {
    let mut data_generator = datagen::DataGenerator::new(SearchLimit::Depth(3), "data.txt");
    data_generator.generate_data(10);
}
