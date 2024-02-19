use super::super::*;

#[test]
fn date_1() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"date("2012-12-25")"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date`
       └─ PositionalParameters
          └─ String
             └─ `2012-12-25`
    "#,
    false,
  );
}
