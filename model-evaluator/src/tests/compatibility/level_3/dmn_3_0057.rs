use super::*;

from_examples!(DMN_3_0057);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &ctx, r#"{a: "foo", b: "bar"}"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision002",
    &ctx,
    r#"{a: "foo", b: {c: "bar", d: {e: "baz"}}}"#,
  );
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision003", &ctx, r#"{a: 3, b: 6}"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision003_a", &ctx, r#"{a: 3, b: 3, c: {d: 6}}"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision004", &ctx, r#"{foo bar: "foo"}"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision005", &ctx, r#"{foo+bar: "foo"}"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision006", &ctx, r#"{foo+bar((!!],foo: "foo"}"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision007", &ctx, r#"{: "foo"}"#);
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
    r#"null(duplicated context entry key: foo)"#,
  );
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision009",
    &ctx,
    r#"null(build_path: no entry b in context: {a: 1})"#,
  );
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision010", &ctx, r#"null"#);
}
