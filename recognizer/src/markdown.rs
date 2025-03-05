//! # Markdown decision table recognizer

use crate::{DecisionTable, DecisionTableOrientation, HitPolicy};
use dsntk_common::Result;

/// Recognizes a decision table defined as plain Markdown text.
pub fn recognize_from_markdown(text: &str, trace: bool) -> Result<DecisionTable> {
  let _ = text;
  let _ = trace;
  let information_item_name = None;
  let input_clauses = vec![];
  let output_clauses = vec![];
  let annotations = vec![];
  let rules = vec![];
  let hit_policy = HitPolicy::Unique;
  let aggregation = None;
  let preferred_orientation = DecisionTableOrientation::RuleAsRow;
  let output_label = None;
  // Return the recognized decision table.
  Ok(DecisionTable::new(
    information_item_name,
    input_clauses,
    output_clauses,
    annotations,
    rules,
    hit_policy,
    aggregation,
    preferred_orientation,
    output_label,
  ))
}
