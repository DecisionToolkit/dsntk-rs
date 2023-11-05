use super::*;

from_examples!(DMN_3_0021);

#[test]
fn _0001() {
  let ctx = context(r#"{Employees: ["Jack","John","Bob","Zack"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision1", &ctx, r#"["John"]"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{Employees: ["Jack","John","Bob","Zack"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision2", &ctx, r#""John""#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{Employees: ["Jack","John","Bob","Zack"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision3", &ctx, r#"["Bob"]"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{Employees: ["Jack","John","Bob","Zack"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision4", &ctx, r#""Bob""#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{Employees: ["Jack","John","Bob","Zack"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision5", &ctx, r#""BOB""#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{Employees: ["Jack","John","Bob","Zack"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision6", &ctx, r#""bob""#);
}
