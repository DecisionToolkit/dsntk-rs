use dsntk_recognizer::recognize_from_markdown;

#[test]
fn _0001() {
  let markdown = r#"
    | C |           |
    |:-:|:---------:|
    |   |    Out    |
    | 1 | "Monday"  |
    | 2 | "Tuesday" | 2 |
  "#;
  assert_eq!(
    r#"<RecognizerError> invalid number of columns, expected 2, actual is 3 in row '| 2 | "Tuesday" | 2 |'"#,
    recognize_from_markdown(markdown).unwrap_err().to_string()
  );
}
