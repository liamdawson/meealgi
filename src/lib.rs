#![warn(missing_docs)]

//! Implementation of a selection of Jean Meeus' astronomy algorithms.
//! 
//! Sources:
//! * [NREL Solar Position Algorithm](https://midcdmz.nrel.gov/solpos/spa.html)
//! * "Astronomical Algorithms, Second Edition" by Jean Meeus

extern crate chrono;
extern crate num;

pub mod time;
pub mod earth;
pub mod utils;