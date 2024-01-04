use super::super::*;
use crate::lalr::TokenType::StartExpression;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "exp(100000000000000000000000000000)",
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `exp`
       └─ PositionalParameters
          └─ Numeric
             └─ `100000000000000000000000000000`
    "#,
    false,
  );
}
