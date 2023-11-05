use super::*;

from_examples!(DMN_3_0071);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_001", &ctx, r#"false"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_002", &ctx, r#"true"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_003", &ctx, r#"true"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_004", &ctx, r#"true"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_005", &ctx, r#"false"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_001", &ctx, r#"false"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_002", &ctx, r#"true"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_003", &ctx, r#"true"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_004", &ctx, r#"true"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_005", &ctx, r#"false"#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_001", &ctx, r#"false"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_002", &ctx, r#"true"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_003", &ctx, r#"true"#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_004", &ctx, r#"true"#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_005", &ctx, r#"false"#);
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_001", &ctx, r#"false"#);
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_002", &ctx, r#"true"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_003", &ctx, r#"true"#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_004", &ctx, r#"true"#);
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_005", &ctx, r#"false"#);
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_001", &ctx, r#"false"#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_002", &ctx, r#"true"#);
}

#[test]
fn _0023() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_003", &ctx, r#"true"#);
}

#[test]
fn _0024() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_004", &ctx, r#"true"#);
}

#[test]
fn _0025() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_005", &ctx, r#"false"#);
}

#[test]
fn _0026() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_001", &ctx, r#"false"#);
}

#[test]
fn _0027() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_002", &ctx, r#"true"#);
}

#[test]
fn _0028() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_003", &ctx, r#"true"#);
}

#[test]
fn _0029() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_004", &ctx, r#"true"#);
}

#[test]
fn _0030() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_005", &ctx, r#"false"#);
}

#[test]
fn _0031() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_001", &ctx, r#"false"#);
}

#[test]
fn _0032() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_002", &ctx, r#"true"#);
}

#[test]
fn _0033() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_003", &ctx, r#"true"#);
}

#[test]
fn _0034() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_004", &ctx, r#"true"#);
}

#[test]
fn _0035() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_005", &ctx, r#"false"#);
}

#[test]
fn _0036() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "null_001", &ctx, r#"null"#);
}

#[test]
fn _0037() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "null_002",
    &ctx,
    r#"null(expected number as a second argument in 'between' operator, actual value type is Null)"#,
  );
}

#[test]
fn _0038() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "null_003",
    &ctx,
    r#"null(expected number as a third argument in 'between' operator, actual value type is Null)"#,
  );
}
