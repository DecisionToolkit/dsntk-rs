//! # FEEL evaluator errors

use dsntk_common::{DsntkError, ToErrorMessage};

/// `FEEL` expressions evaluator errors.
#[derive(ToErrorMessage)]
struct FeelEvaluatorError(String);

pub fn err_not_a_context() -> DsntkError {
  FeelEvaluatorError("expected FEEL context as an input".to_string()).into()
}
