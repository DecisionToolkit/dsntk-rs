use super::*;
use dsntk_examples::decision_tables::*;
use dsntk_recognizer::{from_markdown, from_unicode, BuiltinAggregator, DecisionTableOrientation, HitPolicy};

#[test]
fn h_000010() {
  let expected = (
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
    // expected output clauses
    t_output_clauses(&[(None, None, None)]),
    // expected annotation clauses
    t_annotation_clauses(&[]),
    // expected rules
    t_rules(&[
      (&[], &[r#""Monday""#], &[]),
      (&[], &[r#""Tuesday""#], &[]),
      (&[], &[r#""Wednesday""#], &[]),
      (&[], &[r#""Thursday""#], &[]),
      (&[], &[r#""Friday""#], &[]),
      (&[], &[r#""Saturday""#], &[]),
      (&[], &[r#""Sunday""#], &[]),
    ]),
  );
  t_eq(&expected, from_unicode(H_000010, false).unwrap());
  t_eq(&expected, from_markdown(H_000010, false).unwrap());
}
