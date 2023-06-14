struct Gradient {
    linear: [i8; 1]
}

struct Feature {
    data: i8,
    index: usize,
}

struct Entry {
    feature_vec: Vec<Feature>,
    phase: u8,
    result: i8,
}

struct Tuner {
    entries: Vec<Entry>,
    gradient: Gradient,
}
