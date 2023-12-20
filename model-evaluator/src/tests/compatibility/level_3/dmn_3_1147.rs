use super::*;

from_examples!(DMN_3_1147);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &CTX, "{a: 1}");
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001_a", &CTX, "true");
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision002", &CTX, "{a: 1, b: 2}");
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision003", &CTX, "{a: 2}");
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision004", &CTX, "{a: {bb: 2}}");
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision005", &CTX, "null");
}

#[test]
fn _0007() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision006",
    &CTX,
    "null(expected 1 parameters, actual number of parameters is 0)",
  );
}

#[test]
fn _0008() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision007",
    &CTX,
    "null(expected 1 parameters, actual number of parameters is 2)",
  );
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision008", &CTX, "{a: 1}");
}

#[test]
fn _0010() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision009", &CTX, "null(parameter 'contexts' not found)");
}

#[test]
fn _0011() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision010",
    &CTX,
    "null([core::context merge] invalid argument type, expected context, actual type is number)",
  );
}

#[test]
fn _0012() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision011",
    &CTX,
    "null([core::context merge] invalid argument type, expected context, actual type is number)",
  );
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision012", &CTX, "{a: 1}");
}

#[test]
fn _0014() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision013", &CTX, "{a: 1}");
}
