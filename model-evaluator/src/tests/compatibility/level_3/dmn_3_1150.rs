use super::*;

from_examples!(DMN_3_1150);
static_context!(CTX, "{}");

#[ignore]
#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &CTX, "{a: 1}");
}
