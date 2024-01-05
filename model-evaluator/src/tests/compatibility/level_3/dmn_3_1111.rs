use super::*;

from_examples!(DMN_3_1111);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  let invocable = "decision001";
  let expected = "null([core::matches] invalid argument type, expected string, actual type is Null)";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0002() {
  let invocable = "decision002";
  let expected = "null([core::matches] invalid argument type, expected string, actual type is Null)";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0003() {
  let invocable = "decision003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0004() {
  let invocable = "K2-MatchesFunc-5";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0005() {
  let invocable = "K2-MatchesFunc-6";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0006() {
  let invocable = "K2-MatchesFunc-17";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}
