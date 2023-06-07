use crate::build_script_dependencies::rng::Rng;
use crate::build_script_dependencies::dummy_types::{NUM_SQUARES, NUM_PIECES, NUM_COLORS};

const NUM_CASTLING_CONFIGURATIONS: usize = 16;
const NUM_FILES: usize = 2;

macro_rules! init_keys {
    ($rng:ident, $table:ident, $count:ident) => {{
        $table.push_str("[");
        for _ in 0..$count {
            $table.push_str(format!("{:#x}, ", $rng.rand_u64()).as_str());
        }
        $table.push_str("],\n");
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
            init_keys!(rng, table, NUM_SQUARES);
        }
        table.push_str("],");
    }
    table.push_str("],");

    init_keys!(rng, table, NUM_CASTLING_CONFIGURATIONS);

    table.push_str("ep_file: ");
    init_keys!(rng, table, NUM_FILES);

    table.push_str(format!("side_to_move: [0, {:#x}],\n", rng.rand_u64()).as_str());

    table.push_str("}\n");

    table
}