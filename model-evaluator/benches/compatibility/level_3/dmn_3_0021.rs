use super::*;

from_examples!(DMN_3_0021);
static CTX: Lazy<FeelContext> = Lazy::new(|| context(r#"{Employees: ["Jack","John","Bob","Zack"]}"#));

#[bench]
fn _0001(b: &mut Bencher) {
  let invocable_name = "decision1";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, r#"["John"]"#);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let invocable_name = "decision2";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, r#""John""#);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let invocable_name = "decision3";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, r#"["Bob"]"#);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let invocable_name = "decision4";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, r#""Bob""#);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let invocable_name = "decision5";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, r#""BOB""#);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX));
}
