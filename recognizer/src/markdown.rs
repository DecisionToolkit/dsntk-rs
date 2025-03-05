//! # Markdown decision table recognizer

use crate::errors::{err_md_invalid_decision_table, err_md_invalid_number_of_column, err_md_no_decision_table, err_md_no_hit_policy};
use crate::{DecisionTable, DecisionTableOrientation, HitPolicy};
use dsntk_common::Result;

/// Character used in Markdown as a vertical line that forms table columns.
const TABLE_VERT_LINE: &str = "|";

/// Characters used to denote the information item name of the decision table.
const INFORMATION_ITEM_NAME_START: &str = "> #";

/// Characters used to denote the output label of the decision table.
const OUTPUT_LABEL_START: &str = ">";

const MIN_ROWS: usize = 2;
const MIN_COLUMNS: usize = 2;

type Table = Vec<Vec<Option<String>>>;

/// Recognizes a decision table defined as plain Markdown text.
pub fn recognize_from_markdown(markdown: &str) -> Result<DecisionTable> {
  // Locate and retrieve lines containing the decision table from provided Markdown content.
  let (information_item_name, output_label, lines) = markdown_lines(markdown);

  // Convert lines into table.
  let table = markdown_table(lines)?;

  for row in &table {
    println!("DDD: {:?}", row);
  }

  // Get the hit policy.
  let hit_policy = get_hit_policy(&table)?;

  // Get preferred orientation.
  let (preferred_orientation, _table) = get_preferred_orientation(table)?;

  //
  let input_clauses = vec![];
  let output_clauses = vec![];
  let annotations = vec![];
  let rules = vec![];
  let aggregation = None;

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

/// Returns the lines of the Markdown table, with optional information item name and output label.
fn markdown_lines(text: &str) -> (Option<String>, Option<String>, Vec<String>) {
  enum State {
    BeforeTable,
    BlockQuoteFirst,
    BlockQuoteSecond,
    TableLines,
  }
  let mut buffer = vec![];
  let mut information_item_name = None;
  let mut output_label = None;
  let mut state = State::BeforeTable;
  for line in text.lines().filter_map(|line| {
    let trimmed_line = line.trim();
    if !trimmed_line.is_empty() {
      Some(trimmed_line)
    } else {
      None
    }
  }) {
    match state {
      State::BeforeTable => {
        if line.starts_with(INFORMATION_ITEM_NAME_START) {
          buffer.push(line.trim_start_matches(INFORMATION_ITEM_NAME_START).trim().to_string());
          state = State::BlockQuoteFirst;
        } else if line.starts_with(OUTPUT_LABEL_START) {
          buffer.push(line.trim_start_matches(OUTPUT_LABEL_START).trim().to_string());
          state = State::BlockQuoteSecond;
        } else if line.starts_with(TABLE_VERT_LINE) {
          buffer.push(line.to_string());
          state = State::TableLines;
        }
      }
      State::BlockQuoteFirst => {
        if line.starts_with(OUTPUT_LABEL_START) {
          buffer.push(line.trim_start_matches(OUTPUT_LABEL_START).trim().to_string());
          state = State::BlockQuoteSecond;
        } else if line.starts_with(TABLE_VERT_LINE) {
          information_item_name = buffer.pop();
          buffer.push(line.to_string());
          state = State::TableLines;
        } else {
          buffer.pop();
          state = State::BeforeTable;
        }
      }
      State::BlockQuoteSecond => {
        if line.starts_with(TABLE_VERT_LINE) {
          output_label = buffer.pop();
          information_item_name = buffer.pop();
          buffer.push(line.to_string());
          state = State::TableLines;
        } else {
          buffer.pop();
          buffer.pop();
          state = State::BeforeTable;
        }
      }
      State::TableLines => {
        if line.starts_with(TABLE_VERT_LINE) {
          buffer.push(line.to_string());
        }
      }
    }
  }
  (information_item_name, output_label, buffer)
}

fn markdown_table(lines: Vec<String>) -> Result<Table> {
  let mut table: Table = vec![];
  let Some(first_line) = lines.iter().take(1).next() else {
    return Err(err_md_no_decision_table());
  };
  let columns = markdown_columns(first_line);
  if columns.is_empty() {
    return Err(err_md_no_decision_table());
  }
  let column_count = columns.len();
  table.push(columns);
  for line in lines.iter().skip(2) {
    let columns = markdown_columns(line);
    if columns.len() != column_count {
      return Err(err_md_invalid_number_of_column(column_count, columns.len(), line));
    }
    if columns.iter().any(|column| column.is_some()) {
      table.push(columns);
    }
  }
  let filtered_table = remove_empty_columns(table);
  if filtered_table.len() < MIN_ROWS {
    return Err(err_md_invalid_decision_table(&format!("number of rows is less than {MIN_ROWS}")));
  }
  if filtered_table[0].len() < MIN_COLUMNS {
    return Err(err_md_invalid_decision_table(&format!("number of columns is less than {MIN_COLUMNS}")));
  }
  Ok(filtered_table)
}

/// Converts Markdown line into a vector of columns containing optional text from decision table.
fn markdown_columns(line: &str) -> Vec<Option<String>> {
  let mut row = line
    .split(TABLE_VERT_LINE)
    .skip(1)
    .map(|text| {
      let trimmed_text = text.trim();
      if trimmed_text.is_empty() {
        None
      } else {
        Some(trimmed_text.to_string())
      }
    })
    .collect::<Vec<Option<String>>>();
  row.pop();
  row
}

/// Removes column from each row, for which all columns in all rows are empty.
fn remove_empty_columns(table: Table) -> Table {
  let column_count = table[0].len(); // Safe, table has at least MIN_ROWS
  let mut keep_column = vec![false; column_count];
  for row in &table {
    for (index, column) in row.iter().enumerate() {
      keep_column[index] |= column.is_some();
    }
  }
  table
    .iter()
    .map(|row| {
      row
        .iter()
        .enumerate()
        .filter_map(|(index, column)| if keep_column[index] { Some(column.clone()) } else { None })
        .collect()
    })
    .collect()
}

/// Returns the hit policy.
fn get_hit_policy(table: &Table) -> Result<HitPolicy> {
  let top_row = &table[0]; // Safe, table has at least MIN_ROWS
  let top_left_column = &top_row[0]; // Safe, table has at least MIN_COLUMNS
  top_left_column.as_ref().ok_or(err_md_no_hit_policy())?.try_into()
}

fn get_preferred_orientation(table: Table) -> Result<(DecisionTableOrientation, Table)> {
  let rule_numbers = get_rule_numbers(table[0].iter().skip(1));
  match is_monotonic(&rule_numbers) {
    (true, _empty_count, _rule_count) => Ok((DecisionTableOrientation::RuleAsColumn, table)),
    (false, _, _) => {
      let table = pivot(table);
      let rule_numbers = get_rule_numbers(table[0].iter().skip(1));
      match is_monotonic(&rule_numbers) {
        (true, _empty_count, _rule_count) => Ok((DecisionTableOrientation::RuleAsRow, table)),
        (false, _, _) => Err(err_md_invalid_decision_table("can not recognize the preferred orientation")),
      }
    }
  }
}

/// Returns a vector of numbers converted from optional texts.
fn get_rule_numbers<'a>(columns: impl Iterator<Item = &'a Option<String>>) -> Vec<usize> {
  let zero = "0".to_string();
  columns.map(|text| text.as_ref().unwrap_or(&zero).parse::<usize>().unwrap_or(0)).collect()
}

/// Checks if rule numbers are monotonic, skipping leading zeros.
/// Returns (`true`, `zeros_count`, `rule_count`) when rules are monotonic, otherwise (`false`, 0, 0).
fn is_monotonic(list: &[usize]) -> (bool, usize, usize) {
  let position = list.iter().position(|&x| x != 0);
  match position {
    Some(index) => {
      if list[index..].iter().enumerate().all(|(i, &value)| value == i + 1) {
        (true, index, list[index..].len())
      } else {
        (false, 0, 0)
      }
    }
    None => (false, 0, 0),
  }
}

/// Pivots the table.
fn pivot(table: Table) -> Table {
  let column_count = table[0].len();
  let mut pivot_table: Table = vec![vec![]; column_count];
  for row in table {
    for (index, column) in row.iter().enumerate() {
      pivot_table[index].push(column.clone());
    }
  }
  pivot_table
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_monotonic() {
    assert_eq!((false, 0, 0), is_monotonic(&[0, 0, 0, 0, 0, 0, 0]));
    assert_eq!((false, 0, 0), is_monotonic(&[1, 0, 0, 0, 0, 0, 0]));
    assert_eq!((true, 0, 5), is_monotonic(&[1, 2, 3, 4, 5]));
    assert_eq!((true, 1, 6), is_monotonic(&[0, 1, 2, 3, 4, 5, 6]));
    assert_eq!((true, 2, 7), is_monotonic(&[0, 0, 1, 2, 3, 4, 5, 6, 7]));
    assert_eq!((true, 3, 4), is_monotonic(&[0, 0, 0, 1, 2, 3, 4]));
  }
}
