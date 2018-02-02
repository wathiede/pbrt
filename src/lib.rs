#![feature(trace_macros)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;

pub mod core;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
