use super::*;

from_examples!(DMN_3_1154);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  //TODO Implement the full functionality of DMN 1.5.
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &CTX, "null(boxed 'every' not implemented)");
}
