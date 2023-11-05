use super::super::*;
use crate::lalr::TokenType::StartExpression;

#[test]
fn _0001() {
  let scope = scope!();
  scope.set_name("Date".into());
  scope.set_name("fromString".into());
  accept(
    &scope,
    StartExpression,
    r#"Date.fromString.day"#,
    r#"
       Path
       ├─ Name
       │  └─ `Date`
       └─ Path
          ├─ Name
          │  └─ `fromString`
          └─ Name
             └─ `day`
    "#,
    false,
  );
}
