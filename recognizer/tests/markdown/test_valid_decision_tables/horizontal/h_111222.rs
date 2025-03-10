use super::*;
use dsntk_examples::decision_tables::*;
use dsntk_recognizer::{recognize_from_markdown, DecisionTableOrientation, HitPolicy};

#[test]
fn h_111222() {
  let expected = (
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
    // expected output clauses
    t_output_clauses(&[("Discount".into(), None, None), ("Priority".into(), r#""Normal","High","Low""#.into(), r#""High""#.into())]),
    // expected annotation clauses
    t_annotation_clauses(&["Description", "Reference"]),
    // expected rules
    t_rules(&[
      (&[r#""Business""#, "<10"], &["0.11", r#""Normal""#], &["Small order", "Ref 1"]),
      (&[r#""Business""#, ">=10"], &["0.16", r#""High""#], &["Large order", "Ref 2"]),
      (&[r#""Private""#, "-"], &["0.06", r#""Low""#], &["All orders", "Ref 3"]),
    ]),
  );
  let dt = recognize_from_markdown(H_111222, false).unwrap();
  t_eq(&dt, expected);
}
