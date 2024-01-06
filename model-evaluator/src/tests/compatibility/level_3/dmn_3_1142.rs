use super::*;

from_examples!(DMN_3_1142);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  let invocable = "decision001";
  let expected = "5";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}
