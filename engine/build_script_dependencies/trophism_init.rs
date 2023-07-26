use super::dummy_types::{NUM_SQUARES, Square};

fn trophism_table() -> [[usize; NUM_SQUARES as usize]; NUM_SQUARES as usize] {
    let mut result = [[0; NUM_SQUARES as usize]; NUM_SQUARES as usize];

    for i in 0..NUM_SQUARES {
        let sq_1 = Square::new(i);
        for j in 0..NUM_SQUARES {
            let sq_2 = Square::new(j);
            let l1 = (sq_1.rank() as i16 - sq_2.rank() as i16).pow(2);
            let l2 = (sq_1.file() as i16 - sq_2.file() as i16).pow(2);
            let dist = ((l1 + l2) as f64).sqrt() as usize;
            result[i as usize][j as usize] = dist.min(8); 
        }
    }

    result
}

pub fn trophism_table_init_string() -> String {
    let t = trophism_table();

    let mut result = String::new();
    result.push_str("[\n");
    for i in 0..NUM_SQUARES {
        result.push('[');
        for j in 0..NUM_SQUARES {
            if j % 8 == 0 {
                result.push_str("\n  ");
            }
            result.push_str(format!("{}, ", t[i as usize][j as usize]).as_str());
        }
        result.push_str("\n],\n");
    }
    result.push_str("]\n");

    result
}