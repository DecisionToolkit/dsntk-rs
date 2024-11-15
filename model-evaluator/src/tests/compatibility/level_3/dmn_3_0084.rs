use super::*;

from_examples!(DMN_3_0084);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_001", &CTX, "[2, 3, 4]");
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_002", &CTX, "[5, 6, 6, 7, 7, 8]");
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_003", &CTX, "[]");
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_007", &CTX, "[2, 3, 4]");
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_008", &CTX, "[4, 3, 2]");
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_009", &CTX, "[-1, 0, 1]");
}

#[test]
fn _0007() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_010", &CTX, "[1, 0, -1]");
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_011", &CTX, "[1]");
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_012", &CTX, "[2, 3, 4]");
}

#[test]
fn _0010() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_013", &CTX, "[1, 1, 2, 6, 24]");
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_014", &CTX, "[false, true]");
}

#[test]
fn _0012() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_015", &CTX, "[1, 2, 3, 4]");
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_016", &CTX, "[[1, 2], [3, 4]]");
}

#[test]
fn _0014() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_017",
    &CTX,
    "[1980-01-01, 1980-01-02, 1980-01-03]",
  );
}

#[test]
fn _0015() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_018",
    &CTX,
    "[1980-01-03, 1980-01-02, 1980-01-01]",
  );
}

#[test]
fn _0016() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_019",
    &CTX,
    "null(<FeelEvaluatorError> invalid type of interval start: string)",
  );
}

#[test]
fn _0017() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_019_a",
    &CTX,
    "null(<FeelEvaluatorError> invalid type of interval start: string)",
  );
}

#[test]
fn _0018() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_020",
    &CTX,
    "null(<FeelEvaluatorError> invalid type of interval start: date and time)",
  );
}

#[test]
fn _0019() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_020_a",
    &CTX,
    "null(<FeelEvaluatorError> invalid type of interval start: date and time)",
  );
}

#[test]
fn _0020() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_021",
    &CTX,
    "null(<FeelEvaluatorError> invalid type of interval start: time)",
  );
}

#[test]
fn _0021() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_021_a",
    &CTX,
    "null(<FeelEvaluatorError> invalid type of interval start: time)",
  );
}

#[test]
fn _0022() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_022",
    &CTX,
    "null(<FeelEvaluatorError> invalid type of interval start: days and time duration)",
  );
}

#[test]
fn _0023() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_022_a",
    &CTX,
    "null(<FeelEvaluatorError> invalid type of interval start: days and time duration)",
  );
}

#[test]
fn _0024() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_023", &CTX, "[1, 2]");
}

#[test]
fn _0025() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_024",
    &CTX,
    "[1980-01-01, 1980-01-02, 1980-01-03]",
  );
}

#[test]
fn _0026() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_025",
    &CTX,
    "null(<FeelEvaluatorError> range start must be less or equal then range end)",
  );
}
