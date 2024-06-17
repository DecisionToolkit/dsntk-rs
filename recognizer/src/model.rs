//! # Recognized decision table model

/// Recognized decision table.
pub struct RecognizedDecisionTable {
  /// Information item name.
  pub information_item_name: Option<String>,
  /// List of instances of input clause that compose this decision table.
  pub input_clauses: Vec<RecognizedInputClause>,
  /// List of instances of output clause that compose this decision table.
  pub output_clauses: Vec<RecognizedOutputClause>,
  /// List of instances of rule annotation clause that compose this decision table.
  pub annotations: Vec<RecognizedAnnotationClause>,
  /// List of instances of decision rule that compose this decision table.
  pub rules: Vec<RecognizedDecisionRule>,
  /// Hit policy associated with the instance of the decision table.
  pub hit_policy: RecognizedHitPolicy,
  /// Optional aggregation type when the hit policy is `COLLECT`.
  pub aggregation: Option<RecognizedAggregation>,
  /// Preferred representation of the instance of the decision table.
  pub preferred_orientation: RecognizedOrientation,
  /// Optional output label for the description of the decision table output.
  pub output_label: Option<String>,
}

/// Recognized input clause.
pub struct RecognizedInputClause {
  /// The subject of this input clause, text representation of unary tests.
  pub input_expression: String,
  /// Optional unary tests that constrain the result of input expression of this input clause.
  pub allowed_input_values: Option<String>,
}

/// Recognized output clause.
pub struct RecognizedOutputClause {
  /// The name of the output component when the decision table contains more than one output clause.
  pub name: Option<String>,
  /// Unary tests that constrain the result of output entries corresponding to recognized output clause.
  pub allowed_output_values: Option<String>,
  /// Default output expression, selected in incomplete table when no rules match for the decision table.
  pub default_output_entry: Option<String>,
}

/// Recognized annotation clause.
pub struct RecognizedAnnotationClause {
  /// Name that is used as the name of the rule annotation column of the containing decision table.
  pub name: String,
}

/// Recognized decision rule.
pub struct RecognizedDecisionRule {
  /// Ordered list of input entries that compose recognized decision rule.
  pub input_entries: Vec<RecognizedInputEntry>,
  /// Ordered list of output entries that compose recognized decision rule.
  pub output_entries: Vec<RecognizedOutputEntry>,
  /// Ordered list of rule annotations that compose recognized decision rule.
  pub annotation_entries: Vec<RecognizedAnnotationEntry>,
}

/// Recognized input entry.
pub struct RecognizedInputEntry {
  /// Text representation of unary test that composes recognized input entry.
  pub text: String,
}

/// Recognized output entry.
pub struct RecognizedOutputEntry {
  /// Text representation of literal expression that composes recognized output entry.
  pub text: String,
}

/// Recognized annotation entry.
pub struct RecognizedAnnotationEntry {
  /// Text representing recognized rule annotation.
  pub text: String,
}

/// Recognized hit policy.
pub enum RecognizedHitPolicy {
  /// `UNIQUE` hit policy.
  Unique,
  /// `ANY` hit policy.
  Any,
  /// `PRIORITY` hit policy.
  Priority,
  /// `FIRST` hit policy.
  First,
  /// `COLLECT` hit policy.
  Collect(RecognizedAggregation),
  /// `OUTPUT ORDER` hit policy.
  OutputOrder,
  /// `RULE ORDER` hit policy.
  RuleOrder,
}

/// Recognized aggregation.
pub enum RecognizedAggregation {
  /// The result of the decision table is a list of output entries.
  List,
  /// The result of the decision table is the number of outputs.
  Count,
  /// The result of the decision table is the sum of all the outputs.
  Sum,
  /// The result of the decision table is the smallest value of all the outputs.
  Min,
  /// The result of the decision table is the largest value of all the outputs.
  Max,
}

/// Recognized orientation.
pub enum RecognizedOrientation {
  /// Decision table is presented horizontally, rules are presented as rows.
  RuleAsRow,
  /// Decision table is presented vertically, rules are presented as columns.
  RuleAsColumn,
  /// Decision table is presented as crosstab, rules are composed of two input dimensions.
  CrossTable,
}
