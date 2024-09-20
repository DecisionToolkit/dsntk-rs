//! # TCK definitions

#![cfg(feature = "tck")]

mod errors;
mod handlers;

pub use handlers::post_tck_evaluate;
