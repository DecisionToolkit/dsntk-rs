use super::*;

from_examples!(DMN_3_1151);
static_context!(CTX, "{}");

#[test]
#[ignore]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &CTX, "[3, 4, 5]");
}
