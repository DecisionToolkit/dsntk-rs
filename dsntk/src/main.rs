//! # DecisionToolkit
//!
//! **DecisionToolkit** is a set of tools designed to build, test and evaluate decision models,
//! constructed basing on the [Decision Model and Notation](https://www.omg.org/dmn) (DMN™) specification,
//! which is an industry standard governed by the [Object Management Group](https://www.omg.org) (OMG®).

#[macro_use]
extern crate dsntk_macros;

mod actions;
mod built_in_examples;
mod errors;

/// Main entrypoint of application.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  actions::do_action().await
}
