use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_context(false, &scope!(), r#"context put({}, "a", 1)"#, "{a: 1}");
}
