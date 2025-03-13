use super::*;

#[test]
fn _0001() {
  let expected = (
    // expected information item name
    r#"Offered order options"#.into(),
    // expected output label
    r#"Order options"#.into(),
    // expected hit policy
    HitPolicy::Unique,
    // expected aggregation
    None,
    // expected preferred orientation
    DecisionTableOrientation::RulesAsRows,
    // expected input clauses
    t_input_clauses(&[(r#"Customer"#, r#""Business","Private""#.into()), (r#"Order"#, None)]),
    // expected output clauses
    t_output_clauses(&[
      (r#"Discount"#.into(), None, None),
      (r#"Priority"#.into(), r#""Normal", "High", "Low""#.into(), r#""High""#.into()),
    ]),
    // expected annotation clauses
    t_annotation_clauses(&[r#"Description"#, r#"Reference"#]),
    // expected rules
    t_rules(&[
      (&[r#""Business""#, "<10"], &[r#"0.10"#, r#""Normal""#], &[r#"Desc 1"#, r#"Ref 1"#]),
      (&[r#""Business""#, ">=10"], &[r#"0.15"#, r#""High""#], &[r#"Desc 2"#, r#"Ref 2"#]),
      (&[r#""Private""#, "-"], &[r#"0.05"#, r#""Low""#], &[r#"Desc 3"#, r#"Ref 3"#]),
    ]),
  );
  t_eq(&expected, from_unicode(H_111222, false).unwrap());
  t_eq(&expected, from_markdown(H_111222, false).unwrap());
}
