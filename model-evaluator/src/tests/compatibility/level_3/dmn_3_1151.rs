use super::*;

from_examples!(DMN_3_1151);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  let invocable_name = "decision001";
  let expected = "[3, 4, 5]";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}
