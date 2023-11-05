use super::*;

from_examples!(DMN_2_0002);

#[test]
fn _0001() {
  let ctx = context(r#"{Monthly Salary: 10000}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "Yearly Salary", &ctx, r#"120000"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{Monthly Salary: 8375.00}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "Yearly Salary", &ctx, r#"100500"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{Monthly Salary: 8375.13}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "Yearly Salary", &ctx, r#"100501.56"#);
}
