use super::*;

#[test]
fn _0001() {
  let expected = (
    // expected information item name
    None,
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
    t_output_clauses(&[(None, "0.05, 0.10, 0.15".into(), None)]),
    // expected annotation clauses
    t_annotation_clauses(&[]),
    // expected rules
    t_rules(&[
      (&[r#""Business""#, "<10"], &["0.10"], &[]),
      (&[r#""Business""#, ">=10"], &["0.15"], &[]),
      (&[r#""Private""#, "-"], &["0.05"], &[]),
    ]),
  );
  t_eq(&expected, from_unicode(H_001210, false).unwrap());
  t_eq(&expected, from_markdown(H_001210, false).unwrap());
}
