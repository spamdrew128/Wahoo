pub const MAX_MOVECOUNT: u8 = u8::MAX;
pub const MAX_PLY: u8 = i8::MAX as u8;
type Depth = i8;

type ReductionTable = [[Depth; MAX_MOVECOUNT as usize]; MAX_PLY as usize];

#[allow(clippy::needless_range_loop)]
pub fn lmr_init_string() -> String {
    const LMR_BASE: f64 = 0.77;
    const LMR_DIVISOR: f64  = 2.36;

    let mut reduction_table: ReductionTable = [[0; MAX_MOVECOUNT as usize]; MAX_PLY as usize];
    for d in 0..MAX_PLY {
        for m in 0..(MAX_MOVECOUNT as u32) {
            let depth = f64::from(d.max(1));
            let move_count = f64::from(m.max(1));
            reduction_table[d as usize][m as usize] =
                (LMR_BASE + depth.ln() * move_count.ln() / LMR_DIVISOR) as Depth;
        }
    }

    let mut result = String::new();
    result.push_str("[\n");
    for move_counts in reduction_table {
        result.push('[');
        for r in move_counts {
            result.push_str(format!("{}, ", r).as_str());
        }
        result.push_str("],\n");
    }
    result.push(']');

    result
}
