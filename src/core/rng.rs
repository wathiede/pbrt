// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! rng crate implements the PCG pseudo-random number generator (O’Neill 2014)
use std::ops::Sub;

use crate::Float;

const ONE_MINUS_EPSILON: Float = 1. - Float::EPSILON;

const PCG32_DEFAULT_STATE: u64 = 0x853c49e6748fea9b;
const PCG32_DEFAULT_STREAM: u64 = 0xda3e39cb94b95bdb;
const PCG32_MULT: u64 = 0x5851f42d4c957f2d;

/// Rng maintains the state for a PCG pseudo-random number generator based on O’Neill 2014.
/// It differs from the C++ version by excluding the following methods, which don't appear to be
/// called anywhere in the C++ source tree:
/// * Shuffle
/// * Advance
struct Rng {
    state: u64,
    inc: u64,
}

impl Default for Rng {
    fn default() -> Self {
        Rng {
            state: PCG32_DEFAULT_STATE,
            inc: PCG32_DEFAULT_STREAM,
        }
    }
}

impl Rng {
    /// Creates new random number generator seeded with `sequence_index`.
    pub fn new(sequence_index: u64) -> Rng {
        let mut rng: Rng = Default::default();
        rng.set_sequence(sequence_index);
        rng
    }

    /// Resets RNG state and sets the sequence_index.
    pub fn set_sequence(&mut self, sequence_index: u64) {
        self.state = 0;
        self.inc = (sequence_index << 1) | 1;
        self.uniform_u32();
        self.state += PCG32_DEFAULT_STATE;
        self.uniform_u32();
    }

    /// Returns pseudo-random value uniformly distributed in the range [0, (2^32)-1].
    pub fn uniform_u32(&mut self) -> u32 {
        let oldstate = self.state;
        // C code is:
        // state = oldstate * PCG32_MULT + inc;
        // TODO(wathiede): check on C++ wrapping behavior.
        self.state = oldstate.wrapping_mul(PCG32_MULT).wrapping_add(self.inc);
        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59) as u32;
        // Promote to 64 bits for final operation.
        let xorshifted = xorshifted as u64;
        let rot_inverse = (!rot) as u64;
        let rot = rot as u64;
        // Do math in 64-bits and truncate result back to 32-bit.
        ((xorshifted >> rot) | (xorshifted << ((rot_inverse + 1) & 31))) as u32
    }

    /// Returns pseudo-random value uniformly distributed in the range [0, b − 1].
    pub fn uniform_u32_threshold(&mut self, b: u32) -> u32 {
        let threshold = (!b).wrapping_add(1) % b;
        loop {
            let r = self.uniform_u32();
            if r >= threshold {
                return r % b;
            }
        }
    }

    /// Returns pseudo-random number uniform over in the half-open interval [0, 1).
    #[allow(clippy::excessive_precision)]
    pub fn uniform_float(&mut self) -> Float {
        ONE_MINUS_EPSILON.min((self.uniform_u32() as Float) * 2.3283064365386963e-10)
    }
}

impl Sub for Rng {
    type Output = i64;

    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(self.inc, other.inc);
        let mut cur_mult: u64 = PCG32_MULT;
        let mut cur_plus: u64 = self.inc;
        let mut cur_state: u64 = other.state;
        let mut the_bit: u64 = 1;
        let mut distance: u64 = 0;
        while self.state != cur_state {
            if (self.state & the_bit) != (cur_state & the_bit) {
                cur_state = cur_state * cur_mult + cur_plus;
                distance |= the_bit;
            }
            assert_eq!(self.state & the_bit, cur_state & the_bit);
            the_bit <<= 1;
            cur_plus *= cur_mult + 1;
            cur_mult *= cur_mult;
        }
        distance as i64
    }
}

#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::core::rng::{Rng, PCG32_DEFAULT_STATE, PCG32_DEFAULT_STREAM};

    #[test]
    fn default() {
        let mut rng: Rng = Default::default();
        assert_eq!(rng.state, PCG32_DEFAULT_STATE);
        assert_eq!(rng.inc, PCG32_DEFAULT_STREAM);
        // From C++ verison
        let got: Vec<_> = (0..10).map(|_| rng.uniform_u32()).collect();
        let want = [
            355248013, 41705475, 3406281715, 4186697710, 483882979, 2766312848, 1713261421,
            154902030, 3085534493, 3877580365,
        ];
        assert_eq!(got, want);
    }

    #[test]
    fn uniform_u32_threshold() {
        let mut rng: Rng = Default::default();
        assert_eq!(rng.state, PCG32_DEFAULT_STATE);
        assert_eq!(rng.inc, PCG32_DEFAULT_STREAM);
        // From C++ verison
        let got: Vec<_> = (0..10).map(|_| rng.uniform_u32_threshold(4095)).collect();
        let want = [2668, 1995, 3385, 2470, 1399, 1118, 3511, 465, 1133, 295];
        assert_eq!(got, want);
    }

    #[test]
    fn new() {
        let mut rng = Rng::new(0);
        // From C++ verison
        assert_eq!(rng.uniform_u32(), 1774745655);
    }

    #[test]
    fn threshold() {
        let mut rng: Rng = Default::default();
        rng.uniform_u32_threshold(u32::MAX / 2);
        // If this completes it's a success.
    }

    #[test]
    fn uniform_float() {
        let mut rng: Rng = Default::default();
        let got: Vec<_> = (0..10).map(|_| rng.uniform_float()).collect();
        let want = [
            0.0827126, 0.00971031, 0.793087, 0.974792, 0.112663, 0.644082, 0.3989, 0.0360659,
            0.718407, 0.90282,
        ];
        for (l, r) in got.iter().zip(want) {
            assert_approx_eq!(l, r);
        }
    }

    #[test]
    fn sub() {
        let r1: Rng = Default::default();
        let r2: Rng = Default::default();

        assert_eq!(r1 - r2, 0);
    }
}
