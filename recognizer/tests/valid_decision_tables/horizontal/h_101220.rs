use super::*;

#[test]
fn _0001() {
  let expected = (
    // expected information item name
    "Offered order options".into(),
    // expected output label
    None,
    // expected hit policy
    HitPolicy::Unique,
    // expected aggregation
    None,
    // expected preferred orientation
    DecisionTableOrientation::RulesAsRows,
    // expected input clauses
    t_input_clauses(&[("Customer", r#""Business", "Private""#.into()), ("Order", "<10, >=10".into())]),
    // expected output clauses
    t_output_clauses(&[
      ("Discount".into(), "0.05, 0.10, 0.15".into(), None),
      ("Priority".into(), r#""Normal", "High", "Low""#.into(), None),
    ]),
    // expected annotation clauses
    t_annotation_clauses(&[]),
    // expected rules
    t_rules(&[
      (&[r#""Business""#, "<10"], &["0.10", r#""Normal""#], &[]),
      (&[r#""Business""#, ">=10"], &["0.15", r#""High""#], &[]),
      (&[r#""Private""#, "-"], &["0.05", r#""Low""#], &[]),
    ]),
  );
  t_eq(&expected, from_unicode(H_101220, false).unwrap());
  t_eq(&expected, from_markdown(H_101220, false).unwrap());
}
