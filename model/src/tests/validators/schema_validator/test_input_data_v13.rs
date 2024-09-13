use super::test_files::*;
use super::*;

#[test]
fn _0030() {
  assert_eq!(
    "<ModelParserError> expected value for mandatory attribute 'name' in node 'inputData' at [5:5]",
    expect_err_str(v13::_0030)
  );
}

#[test]
fn _0031() {
  assert_eq!(
    "<ModelParserError> expected value for mandatory attribute 'name' in node 'inputData' at [6:5]",
    expect_err_str(v13::_0031)
  );
}

#[test]
fn _0032() {
  assert_eq!(
    "<ModelError> duplicated namespace URI: https://www.omg.org/spec/DMN/20191111/MODEL/",
    expect_err_str(v13::_0032)
  );
}
