//! # FEEL regex
//!
//! Functions that extensively operate on regular expressions.

mod errors;
mod impl_regex;
mod unicode_blocks;
mod utils;

pub use impl_regex::{is_match, is_match_with_flags};
