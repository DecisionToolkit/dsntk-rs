use super::*;

from_examples!(DMN_3_1153);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  let invocable_name = "decision001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0002() {
  let invocable_name = "decision002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0005() {
  let invocable_name = "decision005";
  let expected = "null";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0006() {
  let invocable_name = "decision006";
  let expected = "null";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0007() {
  let invocable_name = "decision007";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}
