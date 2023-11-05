use super::*;

from_examples!(DMN_3_0080);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_001",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 0)"#,
  );
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_002",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 1)"#,
  );
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_003",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 3)"#,
  );
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_004", &ctx, r#""foo""#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_005",
    &ctx,
    r#"null([core::get value] invalid argument type, expected context, actual type is string)"#,
  );
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_006",
    &ctx,
    r#"null([core::get value] invalid argument type, expected string, actual type is number)"#,
  );
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_007", &ctx, r#""foo""#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_008", &ctx, r#"null(parameter 'key' not found)"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_009",
    &ctx,
    r#"null([core::get value] invalid argument type, expected context, actual type is Null)"#,
  );
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_010",
    &ctx,
    r#"null([core::get value] invalid argument type, expected string, actual type is Null)"#,
  );
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_011",
    &ctx,
    r#"null([core::get value] invalid argument type, expected context, actual type is Null)"#,
  );
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_012", &ctx, r#"null"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{ input_001: "a" }"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_013", &ctx, r#""foo""#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{ input_001: 12 }"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_013",
    &ctx,
    r#"null([core::get value] invalid argument type, expected string, actual type is number)"#,
  );
}
