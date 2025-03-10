mod horizontal;
mod horizontal_decision_tables;
mod minimal_decision_tables;
mod miscellaneous_decision_tables;
mod vertical_decision_tables;

use dsntk_recognizer::{BuiltinAggregator, DecisionTable, DecisionTableOrientation, HitPolicy, InputClause};

fn t_eq(
  actual: &DecisionTable,
  information_item_name: Option<&'static str>,
  output_label: Option<&'static str>,
  hit_policy: HitPolicy,
  aggregation: Option<BuiltinAggregator>,
  preferred_orientations: DecisionTableOrientation,
  input_clauses: Vec<InputClause>,
) {
  assert_eq!(actual.information_item_name, information_item_name.map(|s| s.to_string()));
  assert_eq!(actual.output_label, output_label.map(|s| s.to_string()));
  assert_eq!(actual.hit_policy, hit_policy);
  assert_eq!(actual.aggregation, aggregation);
  assert_eq!(actual.preferred_orientation, preferred_orientations);
  assert_eq!(actual.input_clauses.len(), input_clauses.len());
  for (i, actual_input_clause) in actual.input_clauses.iter().enumerate() {
    assert_eq!(*actual_input_clause, input_clauses[i]);
  }
}

fn t_input_clauses(list: &[(&str, Option<&'static str>)]) -> Vec<InputClause> {
  let mut input_clauses = vec![];
  for input_clause in list {
    input_clauses.push(InputClause {
      input_expression: input_clause.0.to_string(),
      allowed_input_values: input_clause.1.map(|s| s.to_string()),
    });
  }

  input_clauses
}
