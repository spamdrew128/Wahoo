use crate::build_script_dependencies::dummy_types::{NUM_COLORS, NUM_PIECES, NUM_SQUARES};
use crate::random_number_generation::rng::Rng;

const NUM_CASTLING_CONFIGURATIONS: usize = 16;
const NUM_FILES: usize = 8;

macro_rules! init_keys {
    ($rng:ident, $table:ident, $count:ident) => {{
        $table.push_str("[");
        for _ in 0..$count {
            $table.push_str(format!("{:#x}, ", $rng.rand_u64()).as_str());
        }
        $table.push_str("],\n");
    }};
}

pub fn zobrist_keys_init_string() -> String {
    let mut rng = Rng::new();
    let mut table = String::new();
    table.push_str("ZobristKeys {\n");

    table.push_str("pieces: [\n");
    for _ in 0..NUM_COLORS {
        table.push_str("[\n");
        for _ in 0..NUM_PIECES {
            init_keys!(rng, table, NUM_SQUARES);
        }
        table.push_str("],\n");
    }
    table.push_str("],\n");

    table.push_str("castling: ");
    init_keys!(rng, table, NUM_CASTLING_CONFIGURATIONS);

    table.push_str("ep_file: ");
    init_keys!(rng, table, NUM_FILES);

    table.push_str(format!("black_to_move: {:#x},\n", rng.rand_u64()).as_str());

    table.push_str("}\n");

    table
}
