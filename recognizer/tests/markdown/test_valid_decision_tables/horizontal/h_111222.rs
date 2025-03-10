use super::*;
use dsntk_examples::decision_tables::*;
use dsntk_recognizer::{recognize_from_markdown, DecisionTableOrientation, HitPolicy};

#[test]
fn h_111222() {
  let dt = recognize_from_markdown(H_111222, false).unwrap();
  t_eq(
    &dt,
    // expected information item name
    "Offered order options".into(),
    // expected output label
    "Order options".into(),
    // expected hit policy
    HitPolicy::Unique,
    // expected aggregation
    None,
    // expected preferred orientation
    DecisionTableOrientation::RulesAsRows,
    // expected input clauses
    t_input_clauses(&[("Customer type", r#""Business","Private""#.into()), ("Order size", None)]),
  );

  assert_eq!(2, dt.input_clauses.len());
  assert_eq!(2, dt.output_clauses.len());
  assert_eq!(2, dt.annotation_clauses.len());
}
