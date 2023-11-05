use super::*;

from_examples!(DMN_2_0001);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{Full Name: "John Doe"}"#);
  let invocable_name = "Greeting Message";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#""Hello John Doe""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{Full Name: "George Gerschwin!"}"#);
  let invocable_name = "Greeting Message";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#""Hello George Gerschwin!""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}
