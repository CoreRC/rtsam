#![feature(alloc)]

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod core;
pub mod geometry;
pub mod inference;
pub mod linear;
