use super::*;

from_examples!(DMN_3_0012);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{list1: ["a","b","c"],list2: ["x","y","z"]}"#);
  let invocable_name = "listContainsList";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx));
}
