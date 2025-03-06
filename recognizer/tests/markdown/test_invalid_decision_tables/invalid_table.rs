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

#[test]
fn _0002() {
  let markdown = r#"
    | C |           |
    |:-:|:---------:|
    | 1 | "Monday"  |
    | 2 | "Tuesday" |
  "#;
  assert_eq!(
    r#"<RecognizerError> invalid decision table, reason: no markers row before the first rule"#,
    recognize_from_markdown(markdown).unwrap_err().to_string()
  );
}

#[test]
fn _0003() {
  let markdown = r#"
    | C |           |
    |:-:|:---------:|
    |   | 1,2       |
    |   | Out       |
    |   | invalid   |
    | 1 | "Monday"  |
    | 2 | "Tuesday" |
  "#;
  assert_eq!(
    r#"<RecognizerError> invalid decision table, reason: too many rows before the first rule"#,
    recognize_from_markdown(markdown).unwrap_err().to_string()
  );
}

#[test]
fn _0004() {
  let markdown = r#"
    | U |    1     |
    |:--|:--------:|
    |   | "Monday" |
  "#;
  assert_eq!(
    r#"<RecognizerError> invalid decision table, reason: no markers column before the first rule"#,
    recognize_from_markdown(markdown).unwrap_err().to_string()
  );
}

#[test]
fn _0005() {
  let markdown = r#"
    | U |     |     |         |    1     |     |
    |:--|:---:|:---:|:-------:|:--------:|:---:|
    |   | A   | Out | Invalid | "Monday" |     |
  "#;
  assert_eq!(
    r#"<RecognizerError> invalid decision table, reason: too many columns before the first rule"#,
    recognize_from_markdown(markdown).unwrap_err().to_string()
  );
}
