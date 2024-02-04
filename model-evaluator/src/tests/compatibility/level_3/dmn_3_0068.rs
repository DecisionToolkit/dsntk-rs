use super::*;

from_examples!(DMN_3_0068);
static_context!(CTX, "{}");
static_context!(CTXN, "{null_input: null}");

#[test]
fn _0001() {
  let invocable = "null_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0002() {
  let invocable = "null_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0003() {
  let invocable = "boolean_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0004() {
  let invocable = "boolean_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0005() {
  let invocable = "boolean_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0006() {
  let invocable = "boolean_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0007() {
  let invocable = "boolean_005";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0008() {
  let invocable = "boolean_006";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0009() {
  let invocable = "boolean_007";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0010() {
  let invocable = "boolean_008";
  let expected = "null(equal err 'false' =?= '0')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0011() {
  let invocable = "boolean_009";
  let expected = "null(equal err 'true' =?= '1')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0012() {
  let invocable = "number_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0013() {
  let invocable = "number_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0014() {
  let invocable = "number_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0015() {
  let invocable = "number_004";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0016() {
  let invocable = "number_005";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0017() {
  let invocable = "number_006";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0018() {
  let invocable = "number_007";
  let expected = r#"null(equal err '100' =?= '"100"')"#;
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0019() {
  let invocable = "string_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0020() {
  let invocable = "string_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0021() {
  let invocable = "string_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0022() {
  let invocable = "string_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0023() {
  let invocable = "string_005";
  let expected = r#"null(equal err '"foo"' =?= '100')"#;
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0024() {
  let invocable = "list_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0025() {
  let invocable = "list_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0026() {
  let invocable = "list_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0027() {
  let invocable = "list_004";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0028() {
  let invocable = "list_005";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0029() {
  let invocable = "list_006";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0030() {
  let invocable = "list_007";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0031() {
  let invocable = "list_008";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0032() {
  let invocable = "list_009";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0033() {
  let invocable = "list_010";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0034() {
  let invocable = "list_011";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0035() {
  let invocable = "list_012";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0036() {
  let invocable = "list_013";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0037() {
  let invocable = "list_014";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0038() {
  let invocable = "list_015";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0039() {
  let invocable = "list_016";
  let expected = "null(equal err '[]' =?= '0')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0040() {
  let invocable = "context_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0041() {
  let invocable = "context_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0042() {
  let invocable = "context_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0043() {
  let invocable = "context_004";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0044() {
  let invocable = "context_005";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0045() {
  let invocable = "context_006";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0046() {
  let invocable = "context_007";
  let expected = "null(equal err '{}' =?= '[]')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0047() {
  let invocable = "date_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0048() {
  let invocable = "date_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0049() {
  let invocable = "date_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0050() {
  let invocable = "date_004";
  let expected = "null(equal err '2018-12-07' =?= '100')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0051() {
  let invocable = "time_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0052() {
  let invocable = "time_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0053() {
  let invocable = "time_002_a";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0054() {
  let invocable = "time_002_b";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0055() {
  let invocable = "time_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0056() {
  let invocable = "time_004";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0057() {
  let invocable = "time_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0058() {
  let invocable = "time_006";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0059() {
  let invocable = "time_009";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0060() {
  let invocable = "time_010";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0061() {
  let invocable = "time_011";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0062() {
  let invocable = "time_012";
  let expected = "null(equal err '10:30:00' =?= '100')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0063() {
  let invocable = "datetime_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0064() {
  let invocable = "datetime_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0065() {
  let invocable = "datetime_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0066() {
  let invocable = "datetime_003_a";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0067() {
  let invocable = "datetime_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0068() {
  let invocable = "datetime_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0069() {
  let invocable = "datetime_006";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0070() {
  let invocable = "datetime_007";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0071() {
  let invocable = "datetime_008";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0072() {
  let invocable = "datetime_008_a";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0073() {
  let invocable = "datetime_008_b";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0074() {
  let invocable = "datetime_009";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0075() {
  let invocable = "datetime_010";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0076() {
  let invocable = "datetime_011";
  let expected = "null(equal err '2018-12-08T00:00:00' =?= '100')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0077() {
  let invocable = "datetime_012";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0078() {
  let invocable = "datetime_013";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0079() {
  let invocable = "dt_duration_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0080() {
  let invocable = "dt_duration_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0081() {
  let invocable = "dt_duration_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0082() {
  let invocable = "dt_duration_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0083() {
  let invocable = "dt_duration_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0084() {
  let invocable = "dt_duration_006";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0085() {
  let invocable = "dt_duration_007";
  let expected = "null(equal err 'PT0S' =?= '0')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0086() {
  let invocable = "ym_duration_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0087() {
  let invocable = "ym_duration_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0088() {
  let invocable = "ym_duration_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0089() {
  let invocable = "ym_duration_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0090() {
  let invocable = "ym_duration_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0091() {
  let invocable = "ym_duration_006";
  let expected = "null(equal err 'P1Y' =?= 'P365D')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0092() {
  let invocable = "ym_duration_007";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0093() {
  let invocable = "ym_duration_008";
  let expected = "null(equal err 'P0M' =?= '0')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0094() {
  let invocable = "deep_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0095() {
  let invocable = "deep_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0096() {
  let invocable = "deep_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0097() {
  let invocable = "deep_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0098() {
  let invocable = "deep_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0099() {
  let invocable = "deep_006";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0100() {
  let invocable = "deep_007";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0101() {
  let invocable = "range_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
}

#[test]
fn _0102() {
  let invocable = "range_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
}

#[test]
fn _0103() {
  let invocable = "range_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
}

#[test]
fn _0104() {
  let invocable = "range_004";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
}

#[test]
fn _0105() {
  let invocable = "range_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
}

#[test]
fn _0106() {
  let invocable = "range_006";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
}

#[test]
fn _0107() {
  let invocable = "range_007";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
}

#[test]
fn _0108() {
  let invocable = "range_008";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
}

#[test]
fn _0109() {
  let invocable = "range_009";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
}
