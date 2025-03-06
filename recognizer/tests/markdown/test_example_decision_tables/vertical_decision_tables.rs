use dsntk_examples::decision_tables::*;
use dsntk_recognizer::{recognize_from_markdown, DecisionTableOrientation, HitPolicy};

#[test]
fn test_v_111222() {
  let dt = recognize_from_markdown(V_111222, false).unwrap();
  assert_eq!("Offered selling options", dt.information_item_name.unwrap());
  assert_eq!("Sell options", dt.output_label.unwrap());
  assert_eq!(HitPolicy::Unique, dt.hit_policy);
  assert_eq!(None, dt.aggregation);
  assert_eq!(DecisionTableOrientation::RuleAsColumn, dt.preferred_orientation);
}
