use dsntk_recognizer::recognize_from_unicode;

#[test]
fn _0001() {
  assert_eq!(
    "<RecognizerError> expected characters not found: ┌",
    recognize_from_unicode("", false).unwrap_err().to_string()
  );
}

#[test]
fn _0002() {
  assert_eq!(
    "<RecognizerError> expected characters not found: ┌",
    recognize_from_unicode("        ", false).unwrap_err().to_string()
  );
}

#[test]
fn _0003() {
  assert_eq!(
    "<RecognizerError> expected characters not found: ┌",
    recognize_from_unicode("    \n          \n        ", false).unwrap_err().to_string()
  );
}

#[test]
fn _0004() {
  assert_eq!(
    "<RecognizerError> expected characters not found: ┌",
    recognize_from_unicode("  \t        \r    ", false).unwrap_err().to_string()
  );
}

#[test]
fn _0005() {
  let input = r#"

    There is some text,

    but no decision table.

  "#;
  assert_eq!(
    "<RecognizerError> expected characters not found: ┌",
    recognize_from_unicode(input, false).unwrap_err().to_string()
  );
}
