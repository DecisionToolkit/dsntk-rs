use super::*;

from_examples!(DMN_3_1151);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  let invocable_name = "decision001";
  let expected = "[3, 4, 5]";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0002() {
  let invocable_name = "decision002";
  let expected = "[]";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0003() {
  let invocable_name = "decision003";
  let expected = r#"["not a list"]"#;
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0004() {
  let invocable_name = "decision004";
  let expected = "null(only number or boolean indexes are allowed in filters)";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0005() {
  let invocable_name = "decision005";
  let expected = "null(only number or boolean indexes are allowed in filters)";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0006() {
  let invocable_name = "decision006";
  let expected = "null(only number or boolean indexes are allowed in filters)";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}
