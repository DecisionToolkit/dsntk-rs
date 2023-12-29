use super::*;

from_examples!(DMN_2_0003);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{Employment Status: "EMPLOYED"}"#);
  let invocable_name = "Employment Status Statement";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#""You are EMPLOYED""#);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{Employment Status: "RETIRED"}"#);
  let invocable_name = "Employment Status Statement";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    &ctx,
    r#"null(expected string as a second argument in addition)"#,
  );
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}
