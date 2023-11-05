use super::super::*;
use crate::lalr::TokenType::StartExpression;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "null",
    r#"
       Null
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "text" "#,
    r#"
       String
       └─ `text`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "line \n line" "#,
    r#"
       String
       └─ `line 
       line`
    "#,
    false,
  );
}

#[test]
fn _0004() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "before\u002Bafter" "#,
    r#"
       String
       └─ `before+after`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "before\U00002Bafter" "#,
    r#"
       String
       └─ `before+after`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "line \n line \t line \r line" "#,
    "\n       String\n       └─ `line \n       line \t line \r line`\n    ",
    false,
  );
}

#[test]
fn _0007() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "line \' line \" line \\ line" "#,
    r#"
       String
       └─ `line ' line " line \ line`
    "#,
    false,
  );
}

#[test]
fn _0008() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "a \u07EF b" "#,
    r#"
       String
       └─ `a ߯ b`
    "#,
    false,
  );
}

#[test]
fn _0009() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "a \u0801 b" "#,
    r#"
       String
       └─ `a ࠁ b`
    "#,
    false,
  );
}

#[test]
fn _0010() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "a \U010001 b" "#,
    r#"
       String
       └─ `a 𐀁 b`
    "#,
    false,
  );
}

#[test]
fn _0011() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "\"a \\U010437 b\"",
    r#"
       String
       └─ `a 𐐷 b`
    "#,
    false,
  );
}

#[test]
fn _0012() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "a \uD801\uDC12 b" "#,
    r#"
       String
       └─ `a 𐐒 b`
    "#,
    false,
  );
}

#[test]
fn _0013() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#" "a \u0056 \u00A9 b" "#,
    r#"
       String
       └─ `a V © b`
    "#,
    false,
  );
}

#[test]
fn _0014() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#""earth""#,
    r#"
       String
       └─ `earth`
    "#,
    false,
  );
}

#[test]
#[should_panic]
fn _0015() {
  let scope = scope!();
  accept(&scope, StartExpression, r#" "\u000G" "#, r#"  String `a V © b` "#, false);
}

#[test]
fn _0016() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "true",
    r#"
       Boolean
       └─ `true`
    "#,
    false,
  );
}

#[test]
fn _0017() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "false",
    r#"
       Boolean
       └─ `false`
    "#,
    false,
  );
}

#[test]
fn _0018() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"@"2021-09-23""#,
    r#"
       At
       └─ `2021-09-23`
    "#,
    false,
  );
}

#[test]
fn _0019() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"date("2021-09-23")"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date`
       └─ PositionalParameters
          └─ String
             └─ `2021-09-23`
    "#,
    false,
  );
}

#[test]
fn _0020() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"time("10:23:45")"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `time`
       └─ PositionalParameters
          └─ String
             └─ `10:23:45`
    "#,
    false,
  );
}

#[test]
fn _0021() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"date and time("2021-09-23 10:23:45")"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ PositionalParameters
          └─ String
             └─ `2021-09-23 10:23:45`
    "#,
    false,
  );
}

#[test]
fn _0022() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"duration("P1Y")"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `duration`
       └─ PositionalParameters
          └─ String
             └─ `P1Y`
    "#,
    false,
  );
}
