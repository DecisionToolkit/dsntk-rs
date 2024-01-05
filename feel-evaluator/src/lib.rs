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
mod macros;

#[cfg(test)]
mod tests;

pub use crate::evaluators::{evaluate, evaluate_context, evaluate_context_node, evaluate_equals, evaluate_max, evaluate_min, evaluate_sum, prepare};
pub use crate::filters::FilterExpressionEvaluator;
pub use crate::iterations::{EveryExpressionEvaluator, ForExpressionEvaluator, SomeExpressionEvaluator};
