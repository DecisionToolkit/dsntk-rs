use super::*;

from_examples!(DMN_3_0085);

#[bench]
fn _0001(b: &mut Bencher) {
  let input_data = r#"{}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_001";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, input_data, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let input_data = r#"{decision_002_input: "baz"}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_002";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, input_data, r#""foo baz""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0002_a(b: &mut Bencher) {
  let input_data = r#"{}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_002";
  assert_decision_service(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    input_data,
    r#"null(expected string as a second argument in addition)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0002_b(b: &mut Bencher) {
  let input_data = r#"{decision_002_input: null}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_002";
  assert_decision_service(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    input_data,
    r#"null(expected string as a second argument in addition)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0002_c(b: &mut Bencher) {
  let input_data = r#"{decision_002_input: 1234}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_002";
  assert_decision_service(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    input_data,
    r#"null(expected string as a second argument in addition)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let input_data = r#"{decision_003_input_1: "B", decision_003_input_2: "C", inputData_003: "D"}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_003";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, input_data, r#""A B C D""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision_004_1";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let input_data = r#"{}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_005";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, input_data, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let input_data = r#"{}"#;
  let ctx = context(input_data);
  let invocable_name = "decision_005_1";
  assert_decision_service(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    input_data,
    r#"null(invalid number of arguments)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let input_data = r#"{}"#;
  let ctx = context(input_data);
  let invocable_name = "decision_005_2";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, input_data, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}
