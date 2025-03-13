use super::*;

#[test]
fn _0001() {
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
    t_input_clauses(&[(r#"Day"#, r#"0, 1, 2, 3, 4, 5, 6"#.into())]),
    // expected output clauses
    t_output_clauses(&[(None, r#""Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday""#.into(), None)]),
    // expected annotation clauses
    t_annotation_clauses(&[]),
    // expected rules
    t_rules(&[
      (&[r#"1"#], &[r#""Monday""#], &[]),
      (&[r#"2"#], &[r#""Tuesday""#], &[]),
      (&[r#"3"#], &[r#""Wednesday""#], &[]),
      (&[r#"4"#], &[r#""Thursday""#], &[]),
      (&[r#"5"#], &[r#""Friday""#], &[]),
      (&[r#"6"#], &[r#""Saturday""#], &[]),
      (&[r#"0"#], &[r#""Sunday""#], &[]),
    ]),
  );
  t_eq(&expected, from_unicode(H_001110, false).unwrap());
  t_eq(&expected, from_markdown(H_001110, false).unwrap());
}
