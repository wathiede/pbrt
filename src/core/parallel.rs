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

use std::sync::atomic::Ordering;

use crate::Float;

#[cfg(feature = "float-as-double")]
mod float {
    /// UsizeFloat is an integer type with the same number of bits as Float
    pub(super) type UsizeFloat = u64;
    /// AtomicUsizeFloat is an alias to the integer atomic type with enough bits to hold the currently
    /// configured `Float` type.
    pub(super) type AtomicUsizeFloat = std::sync::atomic::AtomicU64;
}

#[cfg(not(feature = "float-as-double"))]
mod float {
    /// UsizeFloat is an integer type with the same number of bits as Float
    pub(super) type UsizeFloat = u32;
    /// AtomicUsizeFloat is an alias to the integer atomic type with enough bits to hold the currently
    /// configured `Float` type.
    pub(super) type AtomicUsizeFloat = std::sync::atomic::AtomicU32;
}

use float::{AtomicUsizeFloat, UsizeFloat};

/// AtomicFloat allows atomic addition of a `Float` by treating its bits as an unsigned integer of
/// the same bit width.
#[derive(Debug)]
pub struct AtomicFloat {
    bits: AtomicUsizeFloat,
}

impl From<Float> for AtomicFloat {
    fn from(f: Float) -> Self {
        AtomicFloat {
            bits: AtomicUsizeFloat::new(f.to_bits()),
        }
    }
}

impl AtomicFloat {
    /// Get the current value stored in the atomic.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::parallel::AtomicFloat;
    ///
    /// let af = AtomicFloat::from(8.);
    /// af.add(4.);
    /// assert_eq!(12., af.get());
    /// ```
    pub fn get(&self) -> Float {
        Float::from_bits(self.bits.load(Ordering::Relaxed))
    }

    /// Adds `v` atomically to this `AtomicFloat`.  It is implemented by converting `Float` values
    /// to equivalently sized integer values, and using integer atomics.
    ///
    /// # Examples
    /// ```
    /// use pbrt::{core::parallel::AtomicFloat, Float};
    /// use rayon::prelude::*;
    ///
    /// let af = AtomicFloat::from(8.);
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

impl Into<Float> for AtomicFloat {
    fn into(self) -> Float {
        Float::from_bits(self.bits.load(Ordering::Relaxed))
    }
}
