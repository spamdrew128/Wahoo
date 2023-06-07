// *Really* minimal PCG32 code / (c) 2014 M.E. O'Neill / pcg-random.org
// Licensed under Apache License 2.0 (NO WARRANTY, etc. see website)

#[derive(Debug)]
struct Pcg32State {
    state: u64,
    inc: u64,
}

impl Pcg32State {
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
    const DEFAULT_SEED: Pcg32State = Pcg32State {
        state: 0x853c49e6748fea9b,
        inc: 0xda3e39cb94b95bdb,
    };

    pub const fn new() -> Self {
        Self {
            state: Self::DEFAULT_SEED,
        }
    }

    pub fn rand_u64(&mut self) -> u64 {
        (u64::from(self.state.next()) << 32) | u64::from(self.state.next())
    }
}
