use super::*;

from_examples!(DMN_3_0068);
static_context!(CTX, "{}");
static_context!(CTXN, "{null_input: null}");

#[bench]
fn _0001(b: &mut Bencher) {
  let invocable = "null_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let invocable = "null_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let invocable = "boolean_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let invocable = "boolean_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let invocable = "boolean_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let invocable = "boolean_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let invocable = "boolean_005";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let invocable = "boolean_006";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let invocable = "boolean_007";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let invocable = "boolean_008";
  let expected = "null(equal err 'false' =?= '0')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0011(b: &mut Bencher) {
  let invocable = "boolean_009";
  let expected = "null(equal err 'true' =?= '1')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0012(b: &mut Bencher) {
  let invocable = "number_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0013(b: &mut Bencher) {
  let invocable = "number_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0014(b: &mut Bencher) {
  let invocable = "number_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0015(b: &mut Bencher) {
  let invocable = "number_004";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0016(b: &mut Bencher) {
  let invocable = "number_005";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0017(b: &mut Bencher) {
  let invocable = "number_006";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0018(b: &mut Bencher) {
  let invocable = "number_007";
  let expected = r#"null(equal err '100' =?= '"100"')"#;
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0019(b: &mut Bencher) {
  let invocable = "string_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0020(b: &mut Bencher) {
  let invocable = "string_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0021(b: &mut Bencher) {
  let invocable = "string_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0022(b: &mut Bencher) {
  let invocable = "string_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0023(b: &mut Bencher) {
  let invocable = "string_005";
  let expected = r#"null(equal err '"foo"' =?= '100')"#;
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0024(b: &mut Bencher) {
  let invocable = "list_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0025(b: &mut Bencher) {
  let invocable = "list_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0026(b: &mut Bencher) {
  let invocable = "list_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0027(b: &mut Bencher) {
  let invocable = "list_004";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0028(b: &mut Bencher) {
  let invocable = "list_005";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0029(b: &mut Bencher) {
  let invocable = "list_006";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0030(b: &mut Bencher) {
  let invocable = "list_007";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0031(b: &mut Bencher) {
  let invocable = "list_008";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0032(b: &mut Bencher) {
  let invocable = "list_009";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0033(b: &mut Bencher) {
  let invocable = "list_010";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0034(b: &mut Bencher) {
  let invocable = "list_011";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0035(b: &mut Bencher) {
  let invocable = "list_012";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0036(b: &mut Bencher) {
  let invocable = "list_013";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0037(b: &mut Bencher) {
  let invocable = "list_014";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0038(b: &mut Bencher) {
  let invocable = "list_015";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0039(b: &mut Bencher) {
  let invocable = "list_016";
  let expected = "null(equal err '[]' =?= '0')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0040(b: &mut Bencher) {
  let invocable = "context_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0041(b: &mut Bencher) {
  let invocable = "context_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0042(b: &mut Bencher) {
  let invocable = "context_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0043(b: &mut Bencher) {
  let invocable = "context_004";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0044(b: &mut Bencher) {
  let invocable = "context_005";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0045(b: &mut Bencher) {
  let invocable = "context_006";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0046(b: &mut Bencher) {
  let invocable = "context_007";
  let expected = "null(equal err '{}' =?= '[]')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0047(b: &mut Bencher) {
  let invocable = "date_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0048(b: &mut Bencher) {
  let invocable = "date_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0049(b: &mut Bencher) {
  let invocable = "date_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0050(b: &mut Bencher) {
  let invocable = "date_004";
  let expected = "null(equal err '2018-12-07' =?= '100')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0051(b: &mut Bencher) {
  let invocable = "time_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0052(b: &mut Bencher) {
  let invocable = "time_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0053(b: &mut Bencher) {
  let invocable = "time_002_a";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0054(b: &mut Bencher) {
  let invocable = "time_002_b";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0055(b: &mut Bencher) {
  let invocable = "time_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0056(b: &mut Bencher) {
  let invocable = "time_004";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0057(b: &mut Bencher) {
  let invocable = "time_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0058(b: &mut Bencher) {
  let invocable = "time_006";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0059(b: &mut Bencher) {
  let invocable = "time_009";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0060(b: &mut Bencher) {
  let invocable = "time_010";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0061(b: &mut Bencher) {
  let invocable = "time_011";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0062(b: &mut Bencher) {
  let invocable = "time_012";
  let expected = "null(equal err '10:30:00' =?= '100')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0063(b: &mut Bencher) {
  let invocable = "datetime_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0064(b: &mut Bencher) {
  let invocable = "datetime_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0065(b: &mut Bencher) {
  let invocable = "datetime_003";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0066(b: &mut Bencher) {
  let invocable = "datetime_003_a";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0067(b: &mut Bencher) {
  let invocable = "datetime_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0068(b: &mut Bencher) {
  let invocable = "datetime_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0069(b: &mut Bencher) {
  let invocable = "datetime_006";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0070(b: &mut Bencher) {
  let invocable = "datetime_007";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0071(b: &mut Bencher) {
  let invocable = "datetime_008";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0072(b: &mut Bencher) {
  let invocable = "datetime_008_a";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0073(b: &mut Bencher) {
  let invocable = "datetime_008_b";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0074(b: &mut Bencher) {
  let invocable = "datetime_009";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0075(b: &mut Bencher) {
  let invocable = "datetime_010";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0076(b: &mut Bencher) {
  let invocable = "datetime_011";
  let expected = "null(equal err '2018-12-08T00:00:00' =?= '100')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0077(b: &mut Bencher) {
  let invocable = "datetime_012";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0078(b: &mut Bencher) {
  let invocable = "datetime_013";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0079(b: &mut Bencher) {
  let invocable = "dt_duration_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0080(b: &mut Bencher) {
  let invocable = "dt_duration_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0081(b: &mut Bencher) {
  let invocable = "dt_duration_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0082(b: &mut Bencher) {
  let invocable = "dt_duration_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0083(b: &mut Bencher) {
  let invocable = "dt_duration_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0084(b: &mut Bencher) {
  let invocable = "dt_duration_006";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0085(b: &mut Bencher) {
  let invocable = "dt_duration_007";
  let expected = "null(equal err 'PT0S' =?= '0')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0086(b: &mut Bencher) {
  let invocable = "ym_duration_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0087(b: &mut Bencher) {
  let invocable = "ym_duration_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0088(b: &mut Bencher) {
  let invocable = "ym_duration_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0089(b: &mut Bencher) {
  let invocable = "ym_duration_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0090(b: &mut Bencher) {
  let invocable = "ym_duration_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0091(b: &mut Bencher) {
  let invocable = "ym_duration_006";
  let expected = "null(equal err 'P1Y' =?= 'P365D')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0092(b: &mut Bencher) {
  let invocable = "ym_duration_007";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0093(b: &mut Bencher) {
  let invocable = "ym_duration_008";
  let expected = "null(equal err 'P0M' =?= '0')";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0094(b: &mut Bencher) {
  let invocable = "deep_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0095(b: &mut Bencher) {
  let invocable = "deep_002";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0096(b: &mut Bencher) {
  let invocable = "deep_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0097(b: &mut Bencher) {
  let invocable = "deep_004";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0098(b: &mut Bencher) {
  let invocable = "deep_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0099(b: &mut Bencher) {
  let invocable = "deep_006";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0100(b: &mut Bencher) {
  let invocable = "deep_007";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX));
}

#[bench]
fn _0101(b: &mut Bencher) {
  let invocable = "range_001";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN));
}

#[bench]
fn _0102(b: &mut Bencher) {
  let invocable = "range_002";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN));
}

#[bench]
fn _0103(b: &mut Bencher) {
  let invocable = "range_003";
  let expected = "false";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN));
}

#[bench]
fn _0104(b: &mut Bencher) {
  let invocable = "range_004";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN));
}

#[bench]
fn _0105(b: &mut Bencher) {
  let invocable = "range_005";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN));
}

#[bench]
fn _0106(b: &mut Bencher) {
  let invocable = "range_006";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN));
}

#[bench]
fn _0107(b: &mut Bencher) {
  let invocable = "range_007";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN));
}

#[bench]
fn _0108(b: &mut Bencher) {
  let invocable = "range_008";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN));
}

#[bench]
fn _0109(b: &mut Bencher) {
  let invocable = "range_009";
  let expected = "true";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN, expected);
  iter!(b, MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN));
}
