use super::*;

from_examples!(DMN_3_1143);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  let invocable = "decision001";
  let expected = "6";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}
