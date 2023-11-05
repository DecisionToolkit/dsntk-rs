use super::*;

from_examples!(DMN_2_0119);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{Age: 17,RiskCategory: "High",isAffordable: true}"#);
  let invocable_name = "Approval Status";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    &ctx,
    r#"[{Approved/Declined: "Declined", Rate: "Standard"}, {Approved/Declined: "Approved", Rate: "Standard"}]"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{Age: 19,RiskCategory: "Low",isAffordable: true}"#);
  let invocable_name = "Approval Status";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    &ctx,
    r#"[{Approved/Declined: "Approved", Rate: "Basic"}]"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{Age: 10,RiskCategory: "Low",isAffordable: true}"#);
  let invocable_name = "Approval Status";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    invocable_name,
    &ctx,
    r#"[{Approved/Declined: "Declined", Rate: "Standard"}]"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}
