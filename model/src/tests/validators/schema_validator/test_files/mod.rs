//! # Input files for testing validation against XML Schema

/// ```text
/// `xmllint` output for 1.3
///
/// 13_0001.dmn:2: element definitionen: Schemas validity error : Element 'definitionen': No matching global declaration available for the validation root.
/// 13_0001.dmn fails to validate
/// ```
pub const DMN_13_0001: &str = include_str!("13_0001.dmn");
