use super::*;

from_examples!(DMN_3_0069);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &ctx, r#"[1, 2, 3]"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001_a", &ctx, r#"[]"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision002",
    &ctx,
    r#"null(index in filter is out of range [1..3], actual index is 0)"#,
  );
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision003",
    &ctx,
    r#"null(index in filter is out of range [1..3], actual index is 4)"#,
  );
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision004", &ctx, r#"1"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision005", &ctx, r#"3"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision006", &ctx, r#"3"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision007", &ctx, r#"1"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision008",
    &ctx,
    r#"null(index in filter is out of range [-3..-1], actual index is -4)"#,
  );
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision009", &ctx, r#"[1, 2, 3]"#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision010", &ctx, r#"[]"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision011", &ctx, r#"[2, 3]"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision012", &ctx, r#"[true]"#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision013", &ctx, r#"[]"#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision014", &ctx, r#"[100]"#);
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision015", &ctx, r#"[]"#);
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision016", &ctx, r#"["foo"]"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision017", &ctx, r#"[]"#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision018", &ctx, r#"true"#);
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision019", &ctx, r#"100"#);
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision020", &ctx, r#""foo""#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision021",
    &ctx,
    r#"null(for singletons, only filter index with value 1 or -1 is accepted)"#,
  );
}

#[test]
fn _0023() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision022",
    &ctx,
    r#"null(for singletons, only filter index with value 1 or -1 is accepted)"#,
  );
}

#[test]
fn _0024() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision023",
    &ctx,
    r#"null(for singletons, only filter index with value 1 or -1 is accepted)"#,
  );
}

#[test]
fn _0025() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision024", &ctx, r#"[{a: 2}, {a: 3}]"#);
}

#[test]
fn _0026() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision025", &ctx, r#"[{a: 2}, {a: 3}]"#);
}

#[test]
fn _0027() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision026", &ctx, r#"[{item: 2}, {item: 3}]"#);
}

#[test]
fn _0028() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision027", &ctx, r#"[3, 4]"#);
}

#[test]
fn _0029() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision028", &ctx, r#"[{x: 1, y: 2}]"#);
}

#[test]
fn _0030() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision029", &ctx, r#"[{x: 1, y: 2}]"#);
}

#[test]
fn _0031() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision030", &ctx, r#"[2, 3]"#);
}

#[test]
fn _0032() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision031", &ctx, r#"[]"#);
}

#[test]
fn _0033() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision032", &ctx, r#"[{y: 2}]"#);
}

#[test]
fn _0034() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision033", &ctx, r#"[null]"#);
}

#[test]
fn _0035() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision034", &ctx, r#"[null, 2]"#);
}
