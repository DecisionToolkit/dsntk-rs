use super::super::*;
use crate::lalr::TokenType::StartRangeLiteral;

#[test]
fn _0001() {
  let input = "[1..2]";
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0002() {
  let input = "(1..2]";
  let expected = r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0003() {
  let input = "]1..2]";
  let expected = r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0004() {
  let input = "[1..2)";
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0005() {
  let input = "[1..2[";
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0006() {
  let input = "(1..2)";
  let expected = r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0007() {
  let input = "]1..2[";
  let expected = r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0008() {
  let input = r#"["a".."z"]"#;
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ String
       │     └─ `a`
       └─ IntervalEnd (closed)
          └─ String
             └─ `z`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0009() {
  let input = r#"[date("2012-01-01")..date("2021-12-31")]"#;
  let expected = r#"
       Range
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
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0010() {
  let input = r#"[@"2012-01-01"..@"2021-12-31"]"#;
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ At
       │     └─ `2012-01-01`
       └─ IntervalEnd (closed)
          └─ At
             └─ `2021-12-31`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0011() {
  let input = r#"[false..true]"#;
  Parser::new(&scope!(), StartRangeLiteral, input, false).parse().unwrap_err();
}
