use super::*;

#[test]
fn _0001() {
  let expected = (
    // expected information item name
    r#"Days of week"#.into(),
    // expected output label
    r#"Weekday"#.into(),
    // expected hit policy
    HitPolicy::Collect(BuiltinAggregator::List),
    // expected aggregation
    BuiltinAggregator::List.into(),
    // expected preferred orientation
    DecisionTableOrientation::RulesAsRows,
    // expected input clauses
    t_input_clauses(&[]),
    // expected output clauses
    t_output_clauses(&[(r#"Weekday"#.into(), None, None)]),
    // expected annotation clauses
    t_annotation_clauses(&[r#"Description"#]),
    // expected rules
    t_rules(&[
      (&[], &[r#""Monday""#], &["Desc 1"]),
      (&[], &[r#""Tuesday""#], &["Desc 2"]),
      (&[], &[r#""Wednesday""#], &["Desc 3"]),
      (&[], &[r#""Thursday""#], &["Desc 4"]),
      (&[], &[r#""Friday""#], &["Desc 5"]),
      (&[], &[r#""Saturday""#], &["Desc 6"]),
      (&[], &[r#""Sunday""#], &["Desc 7"]),
    ]),
  );
  t_eq(&expected, from_unicode(H_110011, false).unwrap());
  t_eq(&expected, from_markdown(H_110011, false).unwrap());
}
