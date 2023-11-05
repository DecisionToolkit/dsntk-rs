use super::*;

from_examples!(DMN_2_0003);

#[test]
fn _0001() {
  let ctx = context(r#"{Employment Status: "EMPLOYED"}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "Employment Status Statement",
    &ctx,
    r#""You are EMPLOYED""#,
  );
}

#[test]
fn _0002() {
  let ctx = context(r#"{Employment Status: "RETIRED"}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "Employment Status Statement",
    &ctx,
    r#"null(expected string as a second argument in addition)"#,
  );
}
