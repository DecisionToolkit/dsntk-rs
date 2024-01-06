use super::*;

from_examples!(DMN_3_1149);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  let invocable = "decision001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0002() {
  let invocable = "decision002";
  let expected = "null(expected 0 parameters, actual number of parameters is 1)";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}
