use super::*;
use dsntk_examples::decision_tables::*;
use dsntk_recognizer::{recognize_from_markdown, BuiltinAggregator, DecisionTableOrientation, HitPolicy};

#[test]
fn h_000010() {
  let dt = recognize_from_markdown(H_000010, false).unwrap();
  t_eq(
    &dt,
    // expected information item name
    None,
    // expected output label
    None,
    // expected hit policy
    HitPolicy::Collect(BuiltinAggregator::List),
    // expected aggregation
    BuiltinAggregator::List.into(),
    // expected preferred orientation
    DecisionTableOrientation::RulesAsRows,
    // expected input clauses
    t_input_clauses(&[]),
  );

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
