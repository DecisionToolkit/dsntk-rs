//! # DecisionToolkit
//!
//! **DecisionToolkit** is a set of tools designed to build, test and evaluate decision models,
//! constructed basing on the [Decision Model and Notation](https://www.omg.org/dmn) (DMN™) specification,
//! which is an industry standard governed by the [Object Management Group](https://www.omg.org) (OMG®).

mod actions;
mod built_in_examples;

/// Main entrypoint of **DecisionToolkit** application.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  actions::do_action().await
}
