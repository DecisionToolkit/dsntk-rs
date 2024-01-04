use super::*;

from_examples!(DMN_3_0084);

#[test]
fn _0001() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_001", &ctx, "[2, 3, 4]");
}

#[test]
fn _0002() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_002", &ctx, "[5, 6, 6, 7, 7, 8]");
}

#[test]
fn _0003() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_003", &ctx, "[]");
}

#[test]
fn _0007() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_007", &ctx, "[2, 3, 4]");
}

#[test]
fn _0008() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_008", &ctx, "[4, 3, 2]");
}

#[test]
fn _0009() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_009", &ctx, "[-1, 0, 1]");
}

#[test]
fn _0010() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_010", &ctx, "[1, 0, -1]");
}

#[test]
fn _0011() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_011", &ctx, "[1]");
}

#[test]
fn _0012() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_012", &ctx, "[2, 3, 4]");
}

#[test]
fn _0013() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_013", &ctx, "[1, 1, 2, 6, 24]");
}

#[test]
fn _0014() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_014", &ctx, "[false, true]");
}

#[test]
fn _0015() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_015", &ctx, "[1, 2, 3, 4]");
}

#[test]
fn _0016() {
  let ctx = context("{}");
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_016", &ctx, "[[1, 2], [3, 4]]");
}
