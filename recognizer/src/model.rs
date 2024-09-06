//! # Decision table model

/// A struct representing a decision table.
pub struct DecisionTable {
  /// Information item name.
  pub information_item_name: Option<String>,
  /// List of instances of input clause that compose this decision table.
  pub input_clauses: Vec<InputClause>,
  /// List of instances of output clause that compose this decision table.
  pub output_clauses: Vec<OutputClause>,
  /// List of instances of rule annotation clause that compose this decision table.
  pub annotations: Vec<RuleAnnotationClause>,
  /// List of instances of decision rule that compose this decision table.
  pub rules: Vec<DecisionRule>,
  /// Hit policy associated with the instance of the decision table.
  pub hit_policy: HitPolicy,
  /// Optional aggregation type when the hit policy is `COLLECT`.
  pub aggregation: Option<Aggregator>,
  /// Preferred representation of the instance of the decision table.
  pub orientation: Orientation,
  /// Optional output label for the description of the decision table output.
  pub output_label: Option<String>,
}

impl DecisionTable {
  /// Creates a new decision table.
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    information_item_name: Option<String>,
    input_clauses: Vec<InputClause>,
    output_clauses: Vec<OutputClause>,
    annotations: Vec<RuleAnnotationClause>,
    rules: Vec<DecisionRule>,
    hit_policy: HitPolicy,
    aggregation: Option<Aggregator>,
    orientation: Orientation,
    output_label: Option<String>,
  ) -> Self {
    Self {
      information_item_name,
      input_clauses,
      output_clauses,
      annotations,
      rules,
      hit_policy,
      aggregation,
      orientation,
      output_label,
    }
  }
}

/// A struct representing an input clause.
pub struct InputClause {
  /// The subject of this input clause, text representation of unary tests.
  pub input_expression: String,
  /// Optional unary tests that constrain the result of input expression of this input clause.
  pub allowed_input_values: Option<String>,
}

/// A struct representing an output clause.
pub struct OutputClause {
  /// The name of the output component when the decision table contains more than one output clause.
  pub name: Option<String>,
  /// Unary tests that constrain the result of output entries corresponding to output clause.
  pub allowed_output_values: Option<String>,
  /// Default output expression, selected in incomplete table when no rules match for the decision table.
  pub default_output_entry: Option<String>,
}

/// A struct representing an annotation clause.
pub struct RuleAnnotationClause {
  /// Name that is used as the name of the rule annotation column of the containing decision table.
  pub name: String,
}

/// A struct representing a decision rule.
pub struct DecisionRule {
  /// Ordered list of input entries that compose decision rule.
  pub input_entries: Vec<InputEntry>,
  /// Ordered list of output entries that compose decision rule.
  pub output_entries: Vec<OutputEntry>,
  /// Ordered list of rule annotations that compose decision rule.
  pub annotation_entries: Vec<AnnotationEntry>,
}

/// A struct representing an input entry.
pub struct InputEntry {
  /// Text representation of unary test that composes recognized input entry.
  pub text: String,
}

/// A struct representing an output entry.
pub struct OutputEntry {
  /// Text representation of literal expression that composes recognized output entry.
  pub text: String,
}

/// A struct representing an annotation entry.
pub struct AnnotationEntry {
  /// Text representing rule annotation.
  pub text: String,
}

/// An enumeration of hit policies.
pub enum HitPolicy {
  /// `UNIQUE` hit policy.
  Unique,
  /// `ANY` hit policy.
  Any,
  /// `PRIORITY` hit policy.
  Priority,
  /// `FIRST` hit policy.
  First,
  /// `COLLECT` hit policy.
  Collect(Aggregator),
  /// `OUTPUT ORDER` hit policy.
  OutputOrder,
  /// `RULE ORDER` hit policy.
  RuleOrder,
}

/// An enumeration representing the built-in aggregator.
pub enum Aggregator {
  /// The result is a list of matching output entries.
  List,
  /// The result is the number of matching outputs.
  Count,
  /// The result is the sum of all matching outputs.
  Sum,
  /// The result is the smallest value of matching outputs.
  Min,
  /// The result is the largest value of matching outputs.
  Max,
}

/// An enumeration representing the orientation of the decision table.
pub enum Orientation {
  /// Decision table is presented horizontally, rules are presented as rows.
  RuleAsRow,
  /// Decision table is presented vertically, rules are presented as columns.
  RuleAsColumn,
  /// Decision table is presented as crosstab, rules are composed of two dimensions.
  CrossTable,
}
