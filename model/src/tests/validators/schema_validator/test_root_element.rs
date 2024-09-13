use super::test_files::*;
use super::*;

#[test]
fn _0001() {
  assert_eq!(
    "<ModelParserError> unexpected XML node, expected: definitions, actual: definitionen",
    expect_err_str(DMN_0001)
  );
}
