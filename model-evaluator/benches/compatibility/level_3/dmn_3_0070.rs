use super::*;

from_examples!(DMN_3_0070);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "null_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "null_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "null_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "null_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "null_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "null_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "null_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "null_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "null_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "number_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "number_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "number_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "number_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "number_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "number_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "number_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "number_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "number_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "string_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "string_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "string_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "string_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "string_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "string_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "string_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "string_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "string_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0028(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0029(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0030(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0031(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0032(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0033(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0034(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0035(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0036(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0037(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0038(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0039(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0040(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0041(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0042(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0043(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0044(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0045(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0046(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0047(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0048(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0049(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0050(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0051(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0052(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0053(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0054(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0055(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0056(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0057(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0058(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0059(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0060(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0061(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0062(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0063(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0064(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_016";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0065(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0066(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0067(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0068(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0069(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0070(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0071(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0072(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0073(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0074(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0075(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0076(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0077(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0078(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0079(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0080(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0081(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0082(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0083(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0084(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0085(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0086(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0087(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0088(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0089(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0090(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0091(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0092(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "function_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0093(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "function_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0094(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "function_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0095(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "function_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0096(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "function_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0097(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "function_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0098(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "function_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0099(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "function_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0100(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "function_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}
