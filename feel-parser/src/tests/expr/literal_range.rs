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
