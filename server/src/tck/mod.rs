//! # TCK definitions

#![cfg(feature = "tck")]

mod errors;
mod handlers;

pub use handlers::evaluate_tck_post;
