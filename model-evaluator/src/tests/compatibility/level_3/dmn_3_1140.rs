use super::*;

from_examples!(DMN_3_1140);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &ctx, r#""abc""#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision002", &ctx, r#""a and b and c""#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision003", &ctx, r#""abc""#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision004", &ctx, r#""abc""#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision005_a", &ctx, r#""a""#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision005_b", &ctx, r#""a""#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision006_a", &ctx, r#""ac""#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision006_b", &ctx, r#""aXc""#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision007_a", &ctx, r#""""#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision007_b", &ctx, r#""""#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision008",
    &ctx,
    "null(expected 1,2 parameters, actual number of parameters is 0)",
  );
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision009",
    &ctx,
    "null(expected 1,2 parameters, actual number of parameters is 3)",
  );
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision010_a", &ctx, r#""ac""#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision010_b", &ctx, r#""aXc""#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision011_a",
    &ctx,
    "null(parameter 'delimiter' not found)",
  );
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision011_b", &ctx, "null(parameter 'list' not found)");
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision012_a",
    &ctx,
    "null(string join: expected list or string, actual value type is Null)",
  );
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision012_b",
    &ctx,
    "null(string join: expected list or string, actual value type is Null)",
  );
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision013",
    &ctx,
    "null([core::string join] invalid argument type, expected string, actual type is number)",
  );
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision014",
    &ctx,
    "null(string join: expected list or string, actual value type is number)",
  );
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision015", &ctx, r#""a""#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision016", &ctx, r#""a""#);
}
