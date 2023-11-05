use super::*;

from_examples!(DMN_3_0093);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "test_001", &CTX, r#"null(invalid @ literal: foo)"#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_001", &CTX, r#"true"#);
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_002", &CTX, r#"2019-03-31"#);
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_001", &CTX, r#"true"#);
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_002", &CTX, r#"2018-12-08T10:30:11"#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_003", &CTX, r#"true"#);
}

#[test]
fn _0007() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "datetime_004",
    &CTX,
    r#""2018-12-08T10:30:11@Australia/Melbourne""#,
  );
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_005", &CTX, r#"true"#);
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_006", &CTX, r#"2018-12-08T10:30:11+11:00"#);
}

#[test]
fn _0010() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_001", &CTX, r#"true"#);
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_002", &CTX, r#"10:30:11"#);
}

#[test]
fn _0012() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_003", &CTX, r#"true"#);
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_004", &CTX, r#"10:30:11+11:00"#);
}

#[test]
fn _0014() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_005", &CTX, r#"true"#);
}

#[test]
fn _0015() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_006", &CTX, r#""10:30:11@Australia/Melbourne""#);
}

#[test]
fn _0016() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_001", &CTX, r#"true"#);
}

#[test]
fn _0017() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_002", &CTX, r#"P10D"#);
}

#[test]
fn _0018() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_001", &CTX, r#"true"#);
}

#[test]
fn _0019() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_002", &CTX, r#"P10Y"#);
}
