use crate::validators::validate_schema;
use dsntk_common::DsntkError;
use roxmltree::Document;

mod test_files;
mod test_root_element;

fn document(input: &str) -> Document {
  Document::parse(input).unwrap()
}

fn expect_err(input: &str) -> DsntkError {
  validate_schema(&document(input)).unwrap_err()
}

fn expect_err_str(input: &str) -> String {
  expect_err(input).to_string()
}
