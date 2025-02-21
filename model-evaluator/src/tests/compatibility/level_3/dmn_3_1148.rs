use super::*;

from_examples!(DMN_3_1148);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &CTX, "true");
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision002", &CTX, "null(expected 0 parameters, actual number of parameters is 1)");
}
