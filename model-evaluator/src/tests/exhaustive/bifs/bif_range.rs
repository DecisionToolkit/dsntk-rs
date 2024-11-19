use super::*;

from_examples!(EXHAUSTIVE_BIF_RANGE);
static_context!(CTX, "{}");

fn assert(invocable_name: &str, expected: &str) {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0001() {
  assert("decision_0001", "[[1..3], [1..3), (1..3], (1..3), [1..3), (1..3], (1..3), [1..), (..3]]");
}

#[test]
fn _0002() {
  assert("decision_0002", r#"[["a".."c"]]"#);
}

#[test]
fn _0003() {
  assert("decision_0100", "[true, true]");
}
