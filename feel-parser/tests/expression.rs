use dsntk_feel::{scope, FeelScope};

#[test]
fn _0001() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "[1,2,3]", false).unwrap();
  let expected = r#"
       List
       ├─ Numeric
       │  └─ `1`
       ├─ Numeric
       │  └─ `2`
       └─ Numeric
          └─ `3`
    "#;
  assert_eq!(expected, node.to_string());
}
