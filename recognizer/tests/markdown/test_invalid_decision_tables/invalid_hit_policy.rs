use dsntk_recognizer::recognize_from_markdown;

#[test]
fn _0001() {
  let markdown = r#"
    |   |             |
    |:-:|:-----------:|
    |   |    Out      |
    | 1 |  "Monday"   |
  "#;
  assert_eq!("<RecognizerError> no hit policy", recognize_from_markdown(markdown).unwrap_err().to_string());
}

#[test]
fn _0002() {
  let markdown = r#"
    | X |             |
    |:-:|:-----------:|
    |   |    Out      |
    | 1 |  "Monday"   |
  "#;
  assert_eq!("<RecognizerError> invalid hit policy: X", recognize_from_markdown(markdown).unwrap_err().to_string());
}

#[test]
fn _0003() {
  let markdown = r#"
    | C@ |             |
    |:--:|:-----------:|
    |    |    Out      |
    | 1  |  "Monday"   |
  "#;
  assert_eq!("<RecognizerError> invalid hit policy: C@", recognize_from_markdown(markdown).unwrap_err().to_string());
}

#[test]
fn _0004() {
  let markdown = r#"
    | C # |             |
    |:---:|:-----------:|
    |     |    Out      |
    | 1   |  "Monday"   |
  "#;
  assert_eq!("<RecognizerError> invalid hit policy: C #", recognize_from_markdown(markdown).unwrap_err().to_string());
}
