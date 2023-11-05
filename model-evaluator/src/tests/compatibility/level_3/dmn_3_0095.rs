use super::*;

from_examples!(DMN_3_0095);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_001", &CTX, r#"1"#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_002", &CTX, r#"365"#);
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_003", &CTX, r#"366"#);
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_004", &CTX, r#"260"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{ date_input_001: @"1970-01-01" }"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_005", &ctx, r#"1"#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_006", &CTX, r#"260"#);
}

#[test]
fn _0007() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_001", &CTX, r#"1"#);
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_002", &CTX, r#"365"#);
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_003", &CTX, r#"366"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{ date_input_001: @"1970-01-01" }"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_004", &ctx, r#"1"#);
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_005", &CTX, r#"260"#);
}

#[test]
fn _0012() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "null_001",
    &CTX,
    r#"null([core::day of year] invalid argument type, expected date, date and time, actual type is Null)"#,
  );
}

#[test]
fn _0013() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "null_002",
    &CTX,
    r#"null(expected 1 parameters, actual number of parameters is 0)"#,
  );
}

#[test]
fn _0014() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "null_003",
    &CTX,
    r#"null([core::day of year] invalid argument type, expected date, date and time, actual type is string)"#,
  );
}

#[test]
fn _0015() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "null_004",
    &CTX,
    r#"null([core::day of year] invalid argument type, expected date, date and time, actual type is Null)"#,
  );
}

#[test]
fn _0016() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "null_005",
    &CTX,
    r#"null([core::day of year] invalid argument type, expected date, date and time, actual type is string)"#,
  );
}

#[test]
fn _0017() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "null_006", &CTX, r#"null(parameter 'date' not found)"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{ date_input_001: "foo" }"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "null_007",
    &ctx,
    r#"null([core::day of year] invalid argument type, expected date, date and time, actual type is string)"#,
  );
}

#[test]
fn _0019() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "null_008",
    &CTX,
    r#"null(expected 1 parameters, actual number of parameters is 2)"#,
  );
}
