use dsntk_examples::decision_tables::*;
use dsntk_recognizer::{recognize_from_markdown, BuiltinAggregator, DecisionTableOrientation, HitPolicy};

#[test]
fn test_h_000010() {
  let dt = recognize_from_markdown(H_000010, false).unwrap();
  assert_eq!(None, dt.information_item_name);
  assert_eq!(None, dt.output_label);
  assert_eq!(HitPolicy::Collect(BuiltinAggregator::List), dt.hit_policy);
  assert_eq!(BuiltinAggregator::List, dt.aggregation.unwrap());
  assert_eq!(DecisionTableOrientation::RulesAsRows, dt.preferred_orientation);
  assert!(dt.input_clauses.is_empty());
  assert_eq!(1, dt.output_clauses.len());
  assert_eq!(None, dt.output_clauses[0].name);
  assert!(dt.annotation_clauses.is_empty());
  assert_eq!(7, dt.rules.len());
  const OUTPUTS: [&str; 7] = [
    r#""Monday""#,
    r#""Tuesday""#,
    r#""Wednesday""#,
    r#""Thursday""#,
    r#""Friday""#,
    r#""Saturday""#,
    r#""Sunday""#,
  ];
  for (index, rule) in dt.rules.iter().enumerate() {
    assert!(rule.input_entries.is_empty());
    assert_eq!(OUTPUTS[index], rule.output_entries[0].text);
    assert!(rule.annotation_entries.is_empty());
  }
}

#[test]
fn test_h_111222() {
  let dt = recognize_from_markdown(H_111222, false).unwrap();
  assert_eq!("Offered order options", dt.information_item_name.unwrap());
  assert_eq!("Order options", dt.output_label.unwrap());
  assert_eq!(HitPolicy::Unique, dt.hit_policy);
  assert_eq!(None, dt.aggregation);
  assert_eq!(DecisionTableOrientation::RulesAsRows, dt.preferred_orientation);
  assert_eq!(2, dt.input_clauses.len());
  assert_eq!(2, dt.output_clauses.len());
  assert_eq!(2, dt.annotation_clauses.len());
}
