use super::*;

from_examples!(DMN_3_1101);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "feel-floor-function_001_75592d0dee", &ctx, r#"1"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "feel-floor-function_002_6fea586853", &ctx, r#"-2"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "feel-floor-function_003_cbae05445d", &ctx, r#"1"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "feel-floor-function_004_30f6d26798", &ctx, r#"-11"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "feel-floor-function_005_dd970ad275", &ctx, r#"5"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "feel-floor-function_006_1223620d9c", &ctx, r#"-1"#);
}
