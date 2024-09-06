//! # Decision table recognizer
//!
//! Recognizes decision tables defined as Unicode text.

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]

#[macro_use]
extern crate dsntk_macros;

mod builder;
mod canvas;
mod errors;
mod model;
mod plane;
mod point;
mod recognizer;
mod rect;
mod tests;

pub use builder::recognize_decision_table;

pub use model::DecisionTable;
