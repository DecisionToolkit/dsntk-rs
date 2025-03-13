use super::*;

#[test]
fn _0001() {
  let expected = (
    // expected information item name
    r#"Days of week"#.into(),
    // expected output label
    r#"Days"#.into(),
    // expected hit policy
    HitPolicy::Collect(BuiltinAggregator::List),
    // expected aggregation
    BuiltinAggregator::List.into(),
    // expected preferred orientation
    DecisionTableOrientation::RulesAsRows,
    // expected input clauses
    t_input_clauses(&[(r#"Day"#, None)]),
    // expected output clauses
    t_output_clauses(&[(r#"Weekday"#.into(), None, None), (r#"Short name"#.into(), None, None)]),
    // expected annotation clauses
    t_annotation_clauses(&[r#"Description"#, r#"Reference"#]),
    // expected rules
    t_rules(&[
      (&[r#"1"#], &[r#""Monday""#, r#""Mon""#], &["Desc 1", "Ref 1"]),
      (&[r#"1"#], &[r#""Tuesday""#, r#""Tue""#], &["Desc 2", "Ref 2"]),
      (&[r#"2"#], &[r#""Wednesday""#, r#""Wed""#], &["Desc 3", "Ref 3"]),
      (&[r#"2"#], &[r#""Thursday""#, r#""Thu""#], &["Desc 4", "Ref 4"]),
      (&[r#"3"#], &[r#""Friday""#, r#""Fri""#], &["Desc 5", "Ref 5"]),
      (&[r#"3"#], &[r#""Saturday""#, r#""Sat""#], &["Desc 6", "Ref 6"]),
      (&[r#"0"#], &[r#""Sunday""#, r#""Sun""#], &["Desc 7", "Ref 7"]),
    ]),
  );
  t_eq(&expected, from_unicode(H_110122, false).unwrap());
  t_eq(&expected, from_markdown(H_110122, false).unwrap());
}
