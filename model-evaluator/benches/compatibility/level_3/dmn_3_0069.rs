use super::*;

from_examples!(DMN_3_0069);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[1, 2, 3]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision001_a";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision002";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    &ctx,
    r#"null(index in filter is out of range [1..3], actual index is 0)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision003";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    &ctx,
    r#"null(index in filter is out of range [1..3], actual index is 4)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"3"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"3"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision008";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    &ctx,
    r#"null(index in filter is out of range [-3..-1], actual index is -4)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[1, 2, 3]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision011";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[2, 3]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision012";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[true]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision013";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision014";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[100]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision015";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision016";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"["foo"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision017";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision018";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision019";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"100"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision020";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision021";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    &ctx,
    r#"null(for singletons, only filter index with value 1 or -1 is accepted)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision022";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    &ctx,
    r#"null(for singletons, only filter index with value 1 or -1 is accepted)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision023";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    &ctx,
    r#"null(for singletons, only filter index with value 1 or -1 is accepted)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision024";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[{a: 2}, {a: 3}]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision025";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[{a: 2}, {a: 3}]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision026";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"[{item: 2}, {item: 3}]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}
