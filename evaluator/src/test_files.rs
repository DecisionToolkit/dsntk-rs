//! # Implementation of the evaluator for test files
//!
//! Test files (textual files containing test cases) are provided for black-box automated testing.
//!
//! Test files are used in evaluation tests for:
//! - `FEEL` expressions (see `tfe` subcommand),
//! - `DMN` models (see `tmd` subcommand),
//! - decision tables (see `tdt` subcommand).
//!
//! Single test case has the following structure:
//! ```text
//! separator input_data , expected_result
//! ```
//! where:
//! - **separator** is one or more separator characters: `~`, `!`, `@`, `#`, `$`, `%`, `^`, `&`, `|`,
//! - **input_data** is a valid `FEEL` context containing input data for a test case,
//! - **,** is literally the comma character,
//! - **expected_result** is a valid `FEEL` value that is expected as a result in test case.
//!
//! Test file may contain one or more test cases.
//!
//! # Example
//!
//! An example of a test file may look like this:
//! ```text
//! % { Customer:"Business",   Order:  -3.23 }, 0.10
//! % { Customer:"Business",   Order:   9.00 }, 0.10
//! % { Customer:"Business",   Order:  10.00 }, 0.15
//! % { Customer:"Business",   Order: 120.00 }, 0.15
//! % { Customer:"Private",    Order:  -2.34 }, 0.05
//! % { Customer:"Private",    Order:  10.00 }, 0.05
//! % { Customer:"Private",    Order: 101.00 }, 0.05
//! % { Customer:"Government", Order:  10.00 }, null
//! ```

use dsntk_common::{DsntkError, Result};
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::FeelScope;
use dsntk_feel_parser::AstNode;

/// Evaluates test cases loaded from input test file.
pub fn evaluate_test_cases(input: &str) -> Result<Vec<(FeelContext, Value)>> {
  let mut test_cases = vec![];
  if let Some(separator) = detect_separator(input) {
    let scope = FeelScope::default();
    for unary_tests in split_test_cases(input, &separator) {
      match dsntk_feel_parser::parse_unary_tests(&scope, unary_tests, false) {
        Ok(ast_node) => match ast_node {
          AstNode::ExpressionList(nodes) => {
            if nodes.len() == 2 {
              if let Ok(input_data) = dsntk_feel_evaluator::evaluate_context_node(&scope, &nodes[0]) {
                let expected_result = dsntk_feel_evaluator::evaluate(&scope, &nodes[1]);
                test_cases.push((input_data, expected_result));
              }
            }
          }
          other => return Err(DsntkError::new("Evaluator", &format!("expected expression list, but found '{other}'"))),
        },
        Err(reason) => return Err(reason),
      }
    }
  }
  Ok(test_cases)
}

/// Splits test cases from input test file using specified separator.
fn split_test_cases<'a>(input: &'a str, separator: &'a str) -> Vec<&'a str> {
  let split = input.split(&separator);
  split
    .filter_map(|s| {
      let trimmed = s.trim();
      if !trimmed.is_empty() {
        Some(trimmed)
      } else {
        None
      }
    })
    .collect()
}

/// Detects the separator used in test file.
fn detect_separator(input: &str) -> Option<String> {
  let mut separator = String::with_capacity(80);
  let mut found = false;
  for ch in input.chars() {
    if found {
      if is_separator_character(ch) {
        separator.push(ch);
      } else {
        break;
      }
    } else if is_separator_character(ch) {
      separator.push(ch);
      found = true;
    }
  }
  if found {
    Some(separator)
  } else {
    None
  }
}

/// Returns `true` when specified character is a test case separator character.
///
/// Valid separator characters are: `~` , `!` , `@` , `#` , `$` , `%` , `^` , `&` , `|`.
///
fn is_separator_character(ch: char) -> bool {
  matches!(ch, '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '|')
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_evaluate_empty_test_file() {
    assert_eq!(0, evaluate_test_cases("").unwrap().len())
  }

  #[test]
  fn test_evaluate_test_file() {
    let input = r#"
      % { Customer:"Business",   Order:  -3.23 }, 0.10
      % { Customer:"Business",   Order:   9.00 }, 0.10
      % { Customer:"Business",   Order:  10.00 }, 0.15
      % { Customer:"Business",   Order: 120.00 }, 0.15
      % { Customer:"Private",    Order:  -2.34 }, 0.05
      % { Customer:"Private",    Order:  10.00 }, 0.05
      % { Customer:"Private",    Order: 101.00 }, 0.05
      % { Customer:"Government", Order:  10.00 }, null
    "#;
    println!("{:?}", evaluate_test_cases(input));
    assert_eq!(8, evaluate_test_cases(input).unwrap().len())
  }

  #[test]
  #[should_panic(expected = r#"DsntkError("<Evaluator> expected expression list, but found '\n       Irrelevant\n    '"#)]
  fn test_evaluate_invalid_test_file() {
    let input = r#"
      % -
    "#;
    evaluate_test_cases(input).unwrap();
  }

  #[test]
  fn test_detect_separator() {
    assert_eq!(None, detect_separator(""));
    assert_eq!("~", detect_separator("~").unwrap());
    assert_eq!("!", detect_separator("!").unwrap());
    assert_eq!("@", detect_separator("@").unwrap());
    assert_eq!("#", detect_separator("#").unwrap());
    assert_eq!("$", detect_separator("$").unwrap());
    assert_eq!("%", detect_separator("%").unwrap());
    assert_eq!("^", detect_separator("^").unwrap());
    assert_eq!("&", detect_separator("&").unwrap());
    assert_eq!("|", detect_separator("|").unwrap());
    assert_eq!("%%", detect_separator("   %%   ").unwrap());
    assert_eq!("%%", detect_separator(" \n\n\n \t  %%{   ").unwrap());
    assert_eq!("%%", detect_separator(" \n %%\n").unwrap());
    assert_eq!("~!@#$%^&|", detect_separator(" ~!@#$%^&| ").unwrap());
  }
}
