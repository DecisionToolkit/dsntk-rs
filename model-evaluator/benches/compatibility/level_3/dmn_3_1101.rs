use super::*;

from_examples!(DMN_3_1101);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-floor-function_001_75592d0dee";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-floor-function_002_6fea586853";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"-2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-floor-function_003_cbae05445d";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-floor-function_004_30f6d26798";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"-11"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-floor-function_005_dd970ad275";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"5"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-floor-function_006_1223620d9c";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"-1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}
