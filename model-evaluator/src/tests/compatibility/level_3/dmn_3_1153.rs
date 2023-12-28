use super::*;

from_examples!(DMN_3_1153);
static_context!(CTX, "{}");

#[test]
#[ignore]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &CTX, r#""then""#);
}
