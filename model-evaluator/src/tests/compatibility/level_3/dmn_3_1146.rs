use super::*;

from_examples!(DMN_3_1146);
static_context!(CTX, "{}");

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision001", &CTX, "{a: 1}");
}

#[test]
fn _0002() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision002",
    &CTX,
    r#"[{key: "a", value: 1}, {key: "b", value: 2}]"#,
  );
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision003", &CTX, "{a: 2}");
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision003_a", &CTX, "{a: 1, b: 3, c: 3}");
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision004", &CTX, r#"{"": 1}"#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision005", &CTX, r#"null"#);
}

#[test]
fn _0007() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision006", &CTX, r#"null"#);
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision007", &CTX, "{a: null}");
}

#[test]
fn _0009() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision008",
    &CTX,
    "null(expected 3 parameters, actual number of parameters is 2)",
  );
}

#[test]
fn _0010() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision009",
    &CTX,
    "null(expected 3 parameters, actual number of parameters is 4)",
  );
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision010", &CTX, "{a: 1}");
}

#[test]
fn _0012() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision011",
    &CTX,
    "null(parameter 'key' or 'keys' not found)",
  );
}

#[test]
fn _0013() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision012",
    &CTX,
    "null([core::context put] invalid argument type, expected context, actual type is list<Null>)",
  );
}

#[test]
fn _0014() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "decision013",
    &CTX,
    "null([core::context put] invalid argument type, expected string or list<string>, actual type is number)",
  );
}

#[test]
fn _0015() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision014", &CTX, "{a: 2}");
}

#[test]
fn _0016() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision015", &CTX, "{context01: {a: 1}, copied: {a: 2}}");
}

#[test]
fn _0017() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "decision016", &CTX, "{copied: {a: 2}, original: {a: 1}}");
}

#[test]
fn _0018() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "nested001", &CTX, "{x: 1, y: {a: 2}}");
}

#[test]
fn _0019() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "nested001_a", &CTX, "{x: 1, y: {a: 0, b: 2}}");
}

#[test]
fn _0020() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "nested002", &CTX, "null");
}

#[test]
fn _0021() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "nested003", &CTX, "null");
}

#[test]
fn _0023() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "nested004", &CTX, "null");
}

#[test]
fn _0024() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "nested005", &CTX, "null");
}

#[test]
fn _0025() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "nested006", &CTX, "{x: 1, y: {a: {b: {c: 2}}}}");
}

#[test]
fn _0026() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "nested007", &CTX, "{x: 1, y: {a: 2}}");
}

#[test]
fn _0027() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "nested008", &CTX, "null");
}

#[test]
fn _0028() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "nested009",
    &CTX,
    "null(parameter 'key' or 'keys' not found)",
  );
}

#[test]
fn _0029() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "nested010", &CTX, "{a: {b: 2}}");
}

#[test]
fn _0030() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "nested011",
    &CTX,
    "{copied: {a: {b: 2}}, nestedContext01: {a: {b: 1}}}",
  );
}

#[test]
fn _0031() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "nested012",
    &CTX,
    "{copied: {a: {b: 2}}, original: {a: {b: 1}}}",
  );
}
