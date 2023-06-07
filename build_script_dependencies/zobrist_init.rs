use crate::build_script_dependencies::rng::Rng;
use crate::build_script_dependencies::dummy_types::{NUM_SQUARES, NUM_PIECES, NUM_COLORS};

const NUM_CASTLING_CONFIGURATIONS: usize = 16;
const NUM_FILES: usize = 2;

macro_rules! init_keys {
    ($rng:ident, $table:ident, $count:ident) => {{
        $table.push_str(format!("["));
        for _ in 0..$count {
            $table.push_str(format!("{}, ", rng.rand_u64()).as_str());
        }
        $table.push_str(format!("]"));
    }};
}

pub fn table_init_string() -> String {
    let mut rng = Rng::new();
    let mut table = String::new();
    table.push_str("ZobristKeys {\n");

    table.push_str("pieces: [\n");
    for _ in 0..NUM_COLORS {
        table.push_str("[\n");
        for _ in 0..NUM_PIECES {
            table.push_str("[\n");
            for _ in 0..NUM_SQUARES {
                table.push_str(format!("{}, ", rng.rand_u64()).as_str());
            }
            table.push_str("],");
        }
        table.push_str("],");
    }
    table.push_str("],");

    table.push_str("castling: [\n");
    for _ in 0..NUM_CASTLING_CONFIGURATIONS {
        table.push_str(format!("{}, ", rng.rand_u64()).as_str());
    }
    table.push_str("],");

    table.push_str("ep_file: [\n");
    for _ in 0..NUM_FILES {
        table.push_str(format!("{}, ", rng.rand_u64()).as_str());
    }
    table.push_str("],");

    table.push_str(format!("side_to_move: [0, {}],\n", rng.rand_u64()).as_str());
    table.push_str("}\n");

    table
}