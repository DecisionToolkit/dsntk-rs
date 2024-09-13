//! # Input files for testing validation against XML Schema v1.3
//!
//! For each input file, there are validation errors provided,
//! these errors are reported by `xmllint` tool.

/// ```text
/// _0001.dmn:2: element definitionen: Schemas validity error : Element 'definitionen': No matching global declaration available for the validation root.
/// ```
pub const _0001: &str = include_str!("_0001.dmn");

/// ```text
/// _0002.dmn:2: element definitions: Schemas validity error : Element 'definitions': No matching global declaration available for the validation root.
/// ```
pub const _0002: &str = include_str!("_0002.dmn");

/// ```text
/// _0003.dmn:2: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/invalid}definitions': No matching global declaration available for the validation root.
/// ```
pub const _0003: &str = include_str!("_0003.dmn");

/// ```text
/// _0004.dmn:2: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions': The attribute 'namespace' is required but missing.
/// _0004.dmn:2: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions': The attribute 'name' is required but missing.
/// ```
pub const _0004: &str = include_str!("_0004.dmn");

/// ```text
/// _0005.dmn:2: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions': The attribute 'namespace' is required but missing.
/// _0005.dmn:2: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions': The attribute 'name' is required but missing.
/// ```
pub const _0005: &str = include_str!("_0005.dmn");

/// ```text
/// _0006.dmn:5: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions', attribute 'revision': The attribute 'revision' is not allowed.
/// ```
pub const _0006: &str = include_str!("_0006.dmn");

/// ```text
/// _0007.dmn:5: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions', attribute 'revision': The attribute 'revision' is not allowed.
/// ```
pub const _0007: &str = include_str!("_0007.dmn");
