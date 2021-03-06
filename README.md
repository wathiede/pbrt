# PBRT
[![Crate](https://img.shields.io/crates/v/pbrt.svg)](https://crates.io/crates/pbrt)
[![API](https://docs.rs/pbrt/badge.svg)](https://docs.rs/pbrt)
[![Actions Status](https://github.com/wathiede/pbrt/workflows/Rust/badge.svg)](https://github.com/wathiede/pbrt/actions)

# What is this?
This is not an officially supported Google product.  This is a personal
project, it serves as a learning experience for both: programming in rust, and
writing a ray-tracer.  If either of these things are interesting to you, then
you may like this project.

This is a rust implementation of the physics based ray tracer documented in
'Physically Based Rendering, Third Edition' http://www.pbrt.org/

If you're looking for a more fully-formed implementation, check out
  https://github.com/wahn/rs_pbrt

# Differences from C++ version
 * Output parameters rewritten as multiple return values.
 * Functions that use bool return type with out parameter are rewritten to use
   Option<>.
 * Constructors: zero-parameter constructors should implement
   [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html), or
   helpfully named constructors like `identity`.  Type changing constructors
   should implement
   [`From`](https://doc.rust-lang.org/std/convert/trait.From.html).
 * `pbrt.h`'s functionality has been put in `lib.rs`.  This is a different
   location from the C++ implementation.  It allows usage like `use pbrt::Float;`
   instead of the more stuttery `use pbrt::core::pbrt::Float;`
 * `Inside` and `InsideExclusive` are members on `Bounds[23]<T>` to overcome
   differences in parameter overloading abilities between C++ and rust.
