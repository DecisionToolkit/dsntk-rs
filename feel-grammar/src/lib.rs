//! `FEEL` grammar definition and `LALR` parsing tables generator.

mod extractor;
mod generator;

pub use generator::lalr_rust_tables;
