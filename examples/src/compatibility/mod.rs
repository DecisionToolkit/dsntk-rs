//! # Decision models for compatibility tests
//!
//! Test file names begin with the compatibility level,
//! followed by the number of the test.
//!
//! Test decision models are based on examples contained in
//! [DMN™ Technology Compatibility Kit](https://dmn-tck.github.io/tck/) project.
//!
//! The content of each model is slightly adjusted and differs from the original.

mod level_2;
mod level_3;
mod non_compliant;

pub use level_2::*;
pub use level_3::*;
pub use non_compliant::*;
