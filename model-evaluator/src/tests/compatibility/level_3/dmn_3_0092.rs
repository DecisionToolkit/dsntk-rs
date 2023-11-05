use super::*;

from_examples!(DMN_3_0092);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_001_1", &CTX, r#"3"#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_002_1", &CTX, r#"4"#);
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_003_1", &CTX, r#"5"#);
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_004_1", &CTX, r#"6"#);
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_005_1", &CTX, r#"20"#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_006_1", &CTX, r#"30"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{input_007_1: 20}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_007_1", &ctx, r#"100"#);
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_008_1", &CTX, r#"6"#);
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_009_1", &CTX, r#"200"#);
}

#[test]
fn _0010() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_010_1", &CTX, r#"120"#);
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_010_1_a", &CTX, r#"120"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{input_011_1: 10}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_011_1", &ctx, r#"5000"#);
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_012_1", &CTX, r#"5000"#);
}

#[test]
fn _0014() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_013_1", &CTX, r#"5000"#);
}

#[test]
fn _0015() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_014_1", &CTX, r#"25"#);
}

#[test]
fn _0016() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_015_1", &CTX, r#"18"#);
}

#[test]
fn _0017() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_016_1",
    &CTX,
    r#"null(expected 1 parameters, actual number of parameters is 2)"#,
  );
}

#[test]
fn _0018() {
  let ctx = context(r#"{input_017_1: "a"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_017_1", &ctx, r#"["a", "a", "z", "z"]"#);
}

#[test]
fn _0019() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_018", &CTX, r#"["a", "a", "z", "z"]"#);
}
