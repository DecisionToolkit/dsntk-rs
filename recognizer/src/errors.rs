//! # Error definitions for recognizer

use crate::point::Point;
use crate::rect::Rect;
use dsntk_common::{DsntkError, ToErrorMessage};

/// Recognizer errors.
#[derive(ToErrorMessage)]
struct RecognizerError(String);

pub fn err_canvas_expected_characters_not_found(searched: &[char]) -> DsntkError {
  RecognizerError(format!("expected characters not found: {searched:?}")).into()
}

pub fn err_canvas_character_is_not_allowed(ch: char, allowed: &[char]) -> DsntkError {
  RecognizerError(format!("character '{ch}' is not allowed in {allowed:?}")).into()
}

pub fn err_canvas_rectangle_not_closed(p1: Point, p2: Point) -> DsntkError {
  RecognizerError(format!("rectangle is not closed, start point: {p1}, end point: {p2}")).into()
}

pub fn err_canvas_region_not_found(r: Rect) -> DsntkError {
  RecognizerError(format!("region not found, rect: {r}")).into()
}

pub fn err_plane_is_empty() -> DsntkError {
  RecognizerError("plane is empty".to_string()).into()
}

pub fn err_plane_cell_is_not_region(details: &str) -> DsntkError {
  RecognizerError(format!("not a region cell in plane: {details}")).into()
}

pub fn err_plane_row_is_out_of_range() -> DsntkError {
  RecognizerError("plane row is out of range".to_string()).into()
}

pub fn err_plane_no_main_double_crossing() -> DsntkError {
  RecognizerError("plane no main double crossing".to_string()).into()
}

pub fn err_plane_column_is_out_of_range() -> DsntkError {
  RecognizerError("plane column is out of range".to_string()).into()
}

pub fn err_plane_invalid_rule_number(num: usize) -> DsntkError {
  RecognizerError(format!("plane invalid rule number: {num}")).into()
}

pub fn err_expected_no_rule_numbers_present() -> DsntkError {
  RecognizerError("expected no rule numbers present".to_string()).into()
}

pub fn err_invalid_input_expressions() -> DsntkError {
  RecognizerError("invalid input expressions".to_string()).into()
}

pub fn err_invalid_output_expressions() -> DsntkError {
  RecognizerError("invalid output expressions".to_string()).into()
}

pub fn err_no_output_clause() -> DsntkError {
  RecognizerError("no output clause".to_string()).into()
}

pub fn err_expected_right_after_rule_numbers_placement() -> DsntkError {
  RecognizerError("expected right-after rule numbers placement".to_string()).into()
}

pub fn err_expected_left_below_rule_numbers_placement() -> DsntkError {
  RecognizerError("expected left-below rule numbers placement".to_string()).into()
}

pub fn err_expected_bottom_left_hit_policy_placement() -> DsntkError {
  RecognizerError("expected bottom-left hit policy placement".to_string()).into()
}

pub fn err_expected_top_left_hit_policy_placement() -> DsntkError {
  RecognizerError("expected top-left hit policy placement".to_string()).into()
}

pub fn err_recognizing_cross_tab_not_supported_yet() -> DsntkError {
  RecognizerError("recognizing cross-tab decision tables is not yet implemented".to_string()).into()
}

pub fn err_too_many_rows_in_input_clause() -> DsntkError {
  RecognizerError("too many rows in input clause".to_string()).into()
}

pub fn err_too_many_rows_in_output_clause() -> DsntkError {
  RecognizerError("too many rows in output clause".to_string()).into()
}

pub fn err_invalid_size(details: &str) -> DsntkError {
  RecognizerError(format!("invalid size: {details}")).into()
}
