//! # Common definitions

#[macro_use]
extern crate dsntk_macros;

mod errors;
mod href;
mod idents;
mod jsonify;
mod namespace;
mod uri;

pub use errors::{DsntkError, Result, ToErrorMessage};
pub use href::HRef;
pub use idents::gen_id;
pub use jsonify::Jsonify;
pub use namespace::to_rdnn;
pub use uri::{encode_segments, to_uri, Uri};
