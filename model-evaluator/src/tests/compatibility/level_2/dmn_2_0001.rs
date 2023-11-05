use super::*;

from_examples!(DMN_2_0001);

#[test]
fn _0001() {
  let ctx = context(r#"{Full Name: "John Doe"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "Greeting Message", &ctx, r#""Hello John Doe""#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{"Full Name": "George Gershwin"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "Greeting Message", &ctx, r#""Hello George Gershwin""#);
}
