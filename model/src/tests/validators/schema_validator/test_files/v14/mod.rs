//! # Input files for testing validation against XML Schema v1.3
//!
//! For each input file, there are validation errors provided,
//! these errors are reported by `xmllint` tool.

/// ```text
/// v14/_0030.dmn:5: element inputData: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20211108/MODEL/}inputData': The attribute 'name' is required but missing.
/// ```
pub const _0030: &str = include_str!("_0030.dmn");
