use super::super::*;
use crate::lalr::TokenType::StartExpression;

#[test]
fn _0001() {
  let input = "[1..10]";
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10`
    "#;
  accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0002() {
  let input = r#"date("2012-12-25") in [date("2012-01-01")..date("2021-12-31")]"#;
  let expected = r#"
       In
       ├─ FunctionInvocation
       │  ├─ Name
       │  │  └─ `date`
       │  └─ PositionalParameters
       │     └─ String
       │        └─ `2012-12-25`
       └─ Range
          ├─ IntervalStart (closed)
          │  └─ FunctionInvocation
          │     ├─ Name
          │     │  └─ `date`
          │     └─ PositionalParameters
          │        └─ String
          │           └─ `2012-01-01`
          └─ IntervalEnd (closed)
             └─ FunctionInvocation
                ├─ Name
                │  └─ `date`
                └─ PositionalParameters
                   └─ String
                      └─ `2021-12-31`
    "#;
  accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0003() {
  let input = "(<=10) = [1..10]";
  let expected = r#"
       Eq
       ├─ UnaryLe
       │  └─ Numeric
       │     └─ `10`
       └─ Range
          ├─ IntervalStart (closed)
          │  └─ Numeric
          │     └─ `1`
          └─ IntervalEnd (closed)
             └─ Numeric
                └─ `10`
    "#;
  accept(&scope!(), StartExpression, input, expected, false);
}
