use super::super::*;
use crate::context::ParsingContext;
use crate::lalr::TokenType::StartExpression;
use dsntk_feel::Name;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"<2"#,
    r#"
       UnaryLt
       └─ Numeric
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
    r#" <= 12.465"#,
    r#"
       UnaryLe
       └─ Numeric
          └─ `12.465`
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
    r#" > 50"#,
    r#"
       UnaryGt
       └─ Numeric
          └─ `50.`
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
    r#" >= time("10:23")"#,
    r#"
       UnaryGe
       └─ FunctionInvocation
          ├─ Name
          │  └─ `time`
          └─ PositionalParameters
             └─ String
                └─ `10:23`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  let mut ctx = ParsingContext::default();
  let name_power = Name::from("power");
  ctx.set_name(name_power);
  scope.set_context("engine".into(), ctx);
  accept(
    &scope,
    StartExpression,
    r#" >= engine.power"#,
    r#"
       UnaryGe
       └─ QualifiedName
          ├─ Name
          │  └─ `engine`
          └─ Name
             └─ `power`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  scope.set_name("engine".into());
  scope.set_name("power".into());
  accept(
    &scope,
    StartExpression,
    r#" >= engine.power"#,
    r#"
       UnaryGe
       └─ QualifiedName
          ├─ Name
          │  └─ `engine`
          └─ Name
             └─ `power`
    "#,
    false,
  );
}

#[test]
#[should_panic]
fn _0007() {
  let scope = scope!();
  accept(&scope, StartExpression, r#" < null"#, r#""#, false);
}
