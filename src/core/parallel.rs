// Copyright 2019 Google LLC
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
//! Utilities for dealing with parallel programming in pbrt.
//!

use std::sync::atomic::Ordering;

use crate::core::pbrt::AtomicUsizeFloat;
use crate::core::pbrt::Float;
use crate::core::pbrt::UsizeFloat;

/// AtomicFloat allows atomic addition of a `Float` by treating its bits as an unsigned integer of
/// the same bit width.
pub struct AtomicFloat {
    bits: AtomicUsizeFloat,
}

impl AtomicFloat {
    pub fn new(f: Float) -> AtomicFloat {
        AtomicFloat {
            bits: AtomicUsizeFloat::new(f.to_bits()),
        }
    }

    pub fn get(&self) -> Float {
        Float::from_bits(self.bits.load(Ordering::Relaxed))
    }

    /// Adds `v` atomically to this `AtomicFloat`.  It is implemented by converting `Float` values
    /// to equivalently sized integer values, and using integer atomics.
    ///
    /// # Examples
    /// ```
    /// use rayon::prelude::*;
    /// use pbrt::core::parallel::AtomicFloat;
    ///
    /// let af = AtomicFloat::new(8.);
    /// (0..10000).into_par_iter().for_each(|_| {
    ///     af.add(0.5);
    /// });
    /// assert_eq!(5008., af.get());
    /// ```
    pub fn add(&self, v: Float) {
        let mut old_bits = self.bits.load(Ordering::Relaxed);
        loop {
            let new_bits: UsizeFloat = (Float::from_bits(old_bits) + v).to_bits();
            match self.bits.compare_exchange_weak(
                old_bits,
                new_bits,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(x) => old_bits = x,
            }
        }
    }
}
