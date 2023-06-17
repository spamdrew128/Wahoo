// *Really* minimal PCG32 code / (c) 2014 M.E. O'Neill / pcg-random.org
// Licensed under Apache License 2.0 (NO WARRANTY, etc. see website)

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Pcg32State {
    state: u64,
    inc: u64,
}

impl Pcg32State {
    fn random_seed() -> Self {
        let state = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let inc = state ^ 0xE5CB_1AFE_6353_7DBF;

        Self {
            state: state as u64,
            inc: inc as u64,
        }
    }

    fn next(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        // Advance internal state
        self.state = oldstate
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc | 1);
        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted: u32 = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot: u32 = (oldstate >> 59) as u32;
        (xorshifted >> rot) | (xorshifted << ((rot.wrapping_neg()) & 31))
    }
}

#[derive(Debug)]
pub struct Rng {
    state: Pcg32State,
}

impl Rng {
    pub fn new() -> Self {
        Self {
            state: Pcg32State::random_seed(),
        }
    }

    pub fn rand_u64(&mut self) -> u64 {
        (u64::from(self.state.next()) << 32) | u64::from(self.state.next())
    }
}
