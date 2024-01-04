use super::*;

from_examples!(DMN_3_1152);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  let invocable_name = "decision001";
  let expected = "[2, 4, 6, 8, 10]";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0002() {
  let invocable_name = "decision002";
  let expected = "[]";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}
