use dsntk_examples::decision_tables::*;
use dsntk_recognizer::{recognize_from_markdown, BuiltinAggregator, DecisionTableOrientation, HitPolicy};

#[test]
fn test_h_000010() {
  let dt = recognize_from_markdown(H_000010).unwrap();
  assert_eq!(None, dt.information_item_name);
  assert_eq!(None, dt.output_label);
  assert_eq!(HitPolicy::Collect(BuiltinAggregator::List), dt.hit_policy);
  assert_eq!(DecisionTableOrientation::RuleAsRow, dt.preferred_orientation);
}
