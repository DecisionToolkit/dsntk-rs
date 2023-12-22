use super::*;

from_examples!(DMN_3_1101);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &CTX, "1");
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision002", &CTX, "-2");
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision003", &CTX, "0");
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision004", &CTX, "1.5");
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision005", &CTX, "-1.6");
}

#[test]
fn _0006() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision006",
    &CTX,
    "null([core::floor] invalid argument type, expected number, actual type is Null)",
  );
}

#[test]
fn _0007() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision007",
    &CTX,
    "null([core::floor] invalid argument type, expected number, actual type is Null)",
  );
}

#[test]
fn _0008() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision008",
    &CTX,
    "null([core::floor] invalid argument type, expected number, actual type is Null)",
  );
}

#[test]
fn _0009() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision009",
    &CTX,
    "null([core::floor] invalid argument type, expected number, actual type is Null)",
  );
}

#[test]
fn _0010() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision010",
    &CTX,
    "null(expected 1,2 parameters, actual number of parameters is 0)",
  );
}

#[test]
fn _0011() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision011",
    &CTX,
    "null(expected 1,2 parameters, actual number of parameters is 3)",
  );
}

#[test]
fn _0012() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision012",
    &CTX,
    "null(expected 2 parameters, actual number of parameters is 3)",
  );
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision013", &CTX, "null(parameter 'scale' not found)");
}

#[test]
fn _0014() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision014",
    &CTX,
    "null([core::floor] invalid argument type, expected number, actual type is string)",
  );
}

#[test]
fn _0015() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision015",
    &CTX,
    "null([core::floor] invalid argument type, expected number, actual type is string)",
  );
}

#[test]
fn _0016() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision016",
    &CTX,
    "null([core::floor] invalid argument type, expected number, actual type is string)",
  );
}

#[test]
fn _0017() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision017",
    &CTX,
    "null([core::floor] invalid argument type, expected number, actual type is string)",
  );
}
