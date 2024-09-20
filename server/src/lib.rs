#[cfg(feature = "tck")]
#[macro_use]
extern crate dsntk_macros;

mod data;
mod server;
mod tck;
mod utils;

pub use server::start_server;
