mod datagen;
mod rng;

fn main() {
    let mut data_generator = datagen::DataGenerator::new(50);
    data_generator.generate_data(10);
}
