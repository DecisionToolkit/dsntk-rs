//! # FEEL expressions evaluator

#[macro_use]
extern crate dsntk_macros;

mod bifs;
mod builders;
mod errors;
mod evaluator_java;
mod evaluator_pmml;
mod evaluators;
mod filters;
mod iterations;
mod iterations2;
mod macros;
mod tests;

pub use crate::evaluators::*;
pub use crate::filters::FilterExpressionEvaluator;
pub use crate::iterations::{EveryExpressionEvaluator, ForExpressionEvaluator, SomeExpressionEvaluator};
