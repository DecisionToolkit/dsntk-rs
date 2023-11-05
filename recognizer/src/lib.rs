//! # Decision Table Recognizer
//!
//! Recognizes decision tables defined as plain Unicode text.

#![warn(missing_docs)]

#[macro_use]
extern crate dsntk_macros;

mod builder;
mod canvas;
mod errors;
mod plane;
mod point;
mod recognizer;
mod rect;

#[cfg(test)]
mod tests;

pub use builder::recognize_decision_table;
