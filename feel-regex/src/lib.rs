//! # FEEL regex
//!
//! Functions that extensively operate on regular expressions.

mod errors;
#[cfg(feature = "onig")]
mod impl_onig;
#[cfg(not(feature = "onig"))]
mod impl_regex;
mod unicode_blocks;
mod utils;

#[cfg(not(feature = "onig"))]
pub use impl_regex::{is_match, is_match_with_flags};

#[cfg(feature = "onig")]
pub use impl_onig::{is_match, is_match_with_flags};
