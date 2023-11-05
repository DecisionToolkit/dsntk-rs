use super::*;

from_examples!(DMN_3_0099);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_001", &CTX, r#"-10"#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_002", &CTX, r#"10"#);
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_003", &CTX, r#"-P1D"#);
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_003_a", &CTX, r#"P1D"#);
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_004", &CTX, r#"-P1Y"#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_004_a", &CTX, r#"P1Y"#);
}

#[test]
fn _0007() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_005",
    &CTX,
    r#"null(unexpected type in arithmetic negation: date)"#,
  );
}

#[test]
fn _0008() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_006",
    &CTX,
    r#"null(unexpected type in arithmetic negation: date and time)"#,
  );
}

#[test]
fn _0009() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_007",
    &CTX,
    r#"null(unexpected type in arithmetic negation: time)"#,
  );
}

#[test]
fn _0010() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_008",
    &CTX,
    r#"null(unexpected type in arithmetic negation: context<a: number>)"#,
  );
}

#[test]
fn _0011() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_009",
    &CTX,
    r#"null(unexpected type in arithmetic negation: string)"#,
  );
}

#[test]
fn _0012() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_010",
    &CTX,
    r#"null(unexpected type in arithmetic negation: list<number>)"#,
  );
}

#[test]
fn _0013() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision_011",
    &CTX,
    r#"null(unexpected type in arithmetic negation: range<number>)"#,
  );
}

#[test]
fn _0014() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision_012", &CTX, r#"-10"#);
}
