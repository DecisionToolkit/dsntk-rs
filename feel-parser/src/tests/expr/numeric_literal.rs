use super::super::*;
use crate::lalr::TokenType::StartExpression;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "2",
    r#"
       Numeric
       └─ `2.`
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
    "2.3",
    r#"
       Numeric
       └─ `2.3`
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
    ".2",
    r#"
       Numeric
       └─ `0.2`
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
    "1",
    r#"
       Numeric
       └─ `1.`
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
    "(1.32983740938573405329458372450983275)",
    r#"
       Numeric
       └─ `1.32983740938573405329458372450983275`
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
    "-14",
    r#"
       Neg
       └─ Numeric
          └─ `14.`
    "#,
    false,
  );
}
