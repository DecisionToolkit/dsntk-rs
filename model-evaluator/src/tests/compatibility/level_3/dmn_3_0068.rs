use super::*;

from_examples!(DMN_3_0068);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "null_001", &ctx, r#"true"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "null_002", &ctx, r#"false"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_001", &ctx, r#"true"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_002", &ctx, r#"false"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_003", &ctx, r#"true"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_004", &ctx, r#"false"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_005", &ctx, r#"false"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_006", &ctx, r#"false"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_007", &ctx, r#"false"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_008", &ctx, r#"null(equal err 'false' =?= '0')"#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "boolean_009", &ctx, r#"null(equal err 'true' =?= '1')"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_001", &ctx, r#"true"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_002", &ctx, r#"true"#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_003", &ctx, r#"true"#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_004", &ctx, r#"true"#);
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_005", &ctx, r#"false"#);
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_006", &ctx, r#"false"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "number_007", &ctx, r#"null(equal err '100' =?= '"100"')"#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_001", &ctx, r#"true"#);
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_002", &ctx, r#"false"#);
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_003", &ctx, r#"true"#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_004", &ctx, r#"false"#);
}

#[test]
fn _0023() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "string_005", &ctx, r#"null(equal err '"foo"' =?= '100')"#);
}

#[test]
fn _0024() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_001", &ctx, r#"true"#);
}

#[test]
fn _0025() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_002", &ctx, r#"false"#);
}

#[test]
fn _0026() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_003", &ctx, r#"false"#);
}

#[test]
fn _0027() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_004", &ctx, r#"true"#);
}

#[test]
fn _0028() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_005", &ctx, r#"false"#);
}

#[test]
fn _0029() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_006", &ctx, r#"true"#);
}

#[test]
fn _0030() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_007", &ctx, r#"true"#);
}

#[test]
fn _0031() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_008", &ctx, r#"true"#);
}

#[test]
fn _0032() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_009", &ctx, r#"true"#);
}

#[test]
fn _0033() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_010", &ctx, r#"true"#);
}

#[test]
fn _0034() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_011", &ctx, r#"true"#);
}

#[test]
fn _0035() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_012", &ctx, r#"true"#);
}

#[test]
fn _0036() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_013", &ctx, r#"true"#);
}

#[test]
fn _0037() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_014", &ctx, r#"true"#);
}

#[test]
fn _0038() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_015", &ctx, r#"false"#);
}

#[test]
fn _0039() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "list_016", &ctx, r#"null(equal err '[]' =?= '0')"#);
}

#[test]
fn _0040() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "context_001", &ctx, r#"true"#);
}

#[test]
fn _0041() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "context_002", &ctx, r#"true"#);
}

#[test]
fn _0042() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "context_003", &ctx, r#"true"#);
}

#[test]
fn _0043() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "context_004", &ctx, r#"true"#);
}

#[test]
fn _0044() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "context_005", &ctx, r#"false"#);
}

#[test]
fn _0045() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "context_006", &ctx, r#"false"#);
}

#[test]
fn _0046() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "context_007", &ctx, r#"null(equal err '{}' =?= '[]')"#);
}

#[test]
fn _0047() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_001", &ctx, r#"true"#);
}

#[test]
fn _0048() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_002", &ctx, r#"false"#);
}

#[test]
fn _0049() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_003", &ctx, r#"false"#);
}

#[test]
fn _0050() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "date_004",
    &ctx,
    r#"null(equal err '2018-12-07' =?= '100')"#,
  );
}

#[test]
fn _0051() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_001", &ctx, r#"true"#);
}

#[test]
fn _0052() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_002", &ctx, r#"false"#);
}

#[test]
fn _0053() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_002_a", &ctx, r#"false"#);
}

#[test]
fn _0054() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_002_b", &ctx, r#"false"#);
}

#[test]
fn _0055() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_003", &ctx, r#"true"#);
}

#[test]
fn _0056() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_004", &ctx, r#"true"#);
}

#[test]
fn _0057() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_005", &ctx, r#"true"#);
}

#[test]
fn _0058() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_006", &ctx, r#"true"#);
}

#[test]
fn _0059() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_009", &ctx, r#"true"#);
}

#[test]
fn _0060() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_010", &ctx, r#"true"#);
}

#[test]
fn _0061() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_011", &ctx, r#"false"#);
}

#[test]
fn _0062() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_012", &ctx, r#"null(equal err '10:30:00' =?= '100')"#);
}

#[test]
fn _0063() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_001", &ctx, r#"true"#);
}

#[test]
fn _0064() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_002", &ctx, r#"true"#);
}

#[test]
fn _0065() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_003", &ctx, r#"true"#);
}

#[test]
fn _0066() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_003_a", &ctx, r#"true"#);
}

#[test]
fn _0067() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_004", &ctx, r#"false"#);
}

#[test]
fn _0068() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_005", &ctx, r#"true"#);
}

#[test]
fn _0069() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_006", &ctx, r#"false"#);
}

#[test]
fn _0070() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_007", &ctx, r#"false"#);
}

#[test]
fn _0071() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_008", &ctx, r#"false"#);
}

#[test]
fn _0072() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_008_a", &ctx, r#"false"#);
}

#[test]
fn _0073() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_008_b", &ctx, r#"true"#);
}

#[test]
fn _0074() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_009", &ctx, r#"true"#);
}

#[test]
fn _0075() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_010", &ctx, r#"false"#);
}

#[test]
fn _0076() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "datetime_011",
    &ctx,
    r#"null(equal err '2018-12-08T00:00:00' =?= '100')"#,
  );
}

#[test]
fn _0077() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_012", &ctx, r#"true"#);
}

#[test]
fn _0078() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "datetime_013", &ctx, r#"true"#);
}

#[test]
fn _0079() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_001", &ctx, r#"true"#);
}

#[test]
fn _0080() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_002", &ctx, r#"true"#);
}

#[test]
fn _0081() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_003", &ctx, r#"false"#);
}

#[test]
fn _0082() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_004", &ctx, r#"false"#);
}

#[test]
fn _0083() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_005", &ctx, r#"true"#);
}

#[test]
fn _0084() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_006", &ctx, r#"false"#);
}

#[test]
fn _0085() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "dt_duration_007",
    &ctx,
    r#"null(equal err 'PT0S' =?= '0')"#,
  );
}

#[test]
fn _0086() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_001", &ctx, r#"true"#);
}

#[test]
fn _0087() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_002", &ctx, r#"true"#);
}

#[test]
fn _0088() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_003", &ctx, r#"false"#);
}

#[test]
fn _0089() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_004", &ctx, r#"false"#);
}

#[test]
fn _0090() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_005", &ctx, r#"true"#);
}

#[test]
fn _0091() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "ym_duration_006",
    &ctx,
    r#"null(equal err 'P1Y' =?= 'P365D')"#,
  );
}

#[test]
fn _0092() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_007", &ctx, r#"false"#);
}

#[test]
fn _0093() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_008", &ctx, r#"null(equal err 'P0M' =?= '0')"#);
}

#[test]
fn _0094() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "deep_001", &ctx, r#"true"#);
}

#[test]
fn _0095() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "deep_002", &ctx, r#"true"#);
}

#[test]
fn _0096() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "deep_003", &ctx, r#"false"#);
}

#[test]
fn _0097() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "deep_004", &ctx, r#"false"#);
}

#[test]
fn _0098() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "deep_005", &ctx, r#"true"#);
}

#[test]
fn _0099() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "deep_006", &ctx, r#"true"#);
}

#[test]
fn _0100() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "deep_007", &ctx, r#"false"#);
}

#[test]
fn _0101() {
  let ctx = context(r#"{null_input: null}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_001", &ctx, r#"true"#);
}

#[test]
fn _0102() {
  let ctx = context(r#"{null_input: null}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_002", &ctx, r#"false"#);
}

#[test]
fn _0103() {
  let ctx = context(r#"{null_input: null}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_003", &ctx, r#"false"#);
}

#[test]
fn _0104() {
  let ctx = context(r#"{null_input: null}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_004", &ctx, r#"true"#);
}

#[test]
fn _0105() {
  let ctx = context(r#"{null_input: null}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_005", &ctx, r#"true"#);
}

#[test]
fn _0106() {
  let ctx = context(r#"{null_input: null}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_006", &ctx, r#"true"#);
}

#[test]
fn _0107() {
  let ctx = context(r#"{null_input: null}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_007", &ctx, r#"true"#);
}

#[test]
fn _0108() {
  let ctx = context(r#"{null_input: null}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_008", &ctx, r#"true"#);
}

#[test]
fn _0109() {
  let ctx = context(r#"{null_input: null}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_009", &ctx, r#"true"#);
}
