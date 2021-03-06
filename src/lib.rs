#![warn(missing_docs)]
#![cfg_attr(feature="cargo-clippy", deny(clippy))]

//! Implementation of a selection of Jean Meeus' astronomy algorithms.
//!
//! Sources:
//! * [NREL Solar Position Algorithm](https://midcdmz.nrel.gov/solpos/spa.html)
//! * "Astronomical Algorithms, Second Edition" by Jean Meeus

extern crate chrono;

#[cfg(test)]
#[macro_use]
extern crate spectral;

pub mod time;
pub mod earth;
pub mod utils;
