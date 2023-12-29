use super::*;

from_examples!(DMN_3_1111);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision001",
    &CTX,
    "null([core::matches] invalid argument type, expected string, actual type is Null)",
  );
}

#[test]
fn _0002() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision002",
    &CTX,
    "null([core::matches] invalid argument type, expected string, actual type is Null)",
  );
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision003", &CTX, "true");
}
