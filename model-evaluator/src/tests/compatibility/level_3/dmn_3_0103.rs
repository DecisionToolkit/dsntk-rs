use super::*;

from_examples!(DMN_3_0103);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_001", &CTX, r#"true"#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_002", &CTX, r#"false"#);
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_001", &CTX, r#"true"#);
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_002", &CTX, r#"false"#);
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_001", &CTX, r#"true"#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_002", &CTX, r#"false"#);
}

#[test]
fn _0007() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_001", &CTX, r#"true"#);
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_002", &CTX, r#"false"#);
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_003", &CTX, r#"false"#);
}

#[test]
fn _0010() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_004", &CTX, r#"false"#);
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_005", &CTX, r#"false"#);
}

#[test]
fn _0012() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_001", &CTX, r#"true"#);
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_002", &CTX, r#"true"#);
}

#[test]
fn _0014() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_003", &CTX, r#"true"#);
}

#[test]
fn _0015() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_004", &CTX, r#"true"#);
}

#[test]
fn _0016() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_005", &CTX, r#"false"#);
}

#[test]
fn _0017() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_006", &CTX, r#"false"#);
}

#[test]
fn _0018() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_007", &CTX, r#"false"#);
}

#[test]
fn _0019() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_008", &CTX, r#"false"#);
}

#[test]
fn _0020() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_009", &CTX, r#"false"#);
}

#[test]
fn _0021() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_010", &CTX, r#"false"#);
}

#[test]
fn _0022() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_011", &CTX, r#"false"#);
}

#[test]
fn _0023() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_012", &CTX, r#"false"#);
}

#[test]
fn _0024() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_013", &CTX, r#"false"#);
}

#[test]
fn _0025() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_001", &CTX, r#"true"#);
}

#[test]
fn _0026() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_002", &CTX, r#"true"#);
}

#[test]
fn _0027() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_003", &CTX, r#"true"#);
}

#[test]
fn _0028() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_004", &CTX, r#"false"#);
}

#[test]
fn _0029() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_005", &CTX, r#"false"#);
}

#[test]
fn _0030() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_006", &CTX, r#"false"#);
}

#[test]
fn _0031() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_007", &CTX, r#"false"#);
}

#[test]
fn _0032() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_008", &CTX, r#"false"#);
}

#[test]
fn _0033() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_009", &CTX, r#"false"#);
}

#[test]
fn _0034() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_010", &CTX, r#"false"#);
}

#[test]
fn _0035() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_011", &CTX, r#"false"#);
}

#[test]
fn _0036() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_012", &CTX, r#"false"#);
}

#[test]
fn _0037() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_013", &CTX, r#"false"#);
}

#[test]
fn _0038() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_014", &CTX, r#"false"#);
}

#[test]
fn _0039() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_015", &CTX, r#"true"#);
}

#[test]
fn _0040() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_016", &CTX, r#"false"#);
}

#[test]
fn _0041() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_017", &CTX, r#"false"#);
}

#[test]
fn _0042() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_001", &CTX, r#"true"#);
}

#[test]
fn _0043() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_002", &CTX, r#"true"#);
}

#[test]
fn _0044() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_001", &CTX, r#"true"#);
}

#[test]
fn _0045() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_002", &CTX, r#"true"#);
}

#[test]
fn _0046() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "zero_duration_001", &CTX, r#"false"#);
}

#[test]
fn _0047() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "named_params_001", &CTX, r#"true"#);
}

#[test]
fn _0048() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "named_params_002", &CTX, r#"false"#);
}

#[test]
fn _0049() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "named_params_003", &CTX, r#"false"#);
}

#[test]
fn _0050() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "named_params_004",
    &CTX,
    r#"null(expected 2 parameters, actual number of parameters is 3)"#,
  );
}
