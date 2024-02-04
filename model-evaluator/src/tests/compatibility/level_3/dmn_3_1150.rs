use super::*;

from_examples!(DMN_3_1150);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  let invocable_name = "decision001";
  let expected = r#""then""#;
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0002() {
  let invocable_name = "decision002";
  let expected = r#""else""#;
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0003() {
  let invocable_name = "decision003";
  let expected = "null(condition is not a boolean expression)";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}
