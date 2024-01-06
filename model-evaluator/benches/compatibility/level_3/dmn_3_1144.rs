use super::*;

from_examples!(DMN_3_1144);
static_context!(CTX, "{}");

#[bench]
fn _0001(b: &mut Bencher) {
  let invocable = "decision001";
  let expected = "5";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}
