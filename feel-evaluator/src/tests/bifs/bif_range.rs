use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_be_value(false, &scope!(), r#"range("[1..2]")"#, r#"[1..2]"#);
}

#[test]
fn _0002() {
  te_be_value(false, &scope!(), r#"range(from: "[1..2]")"#, r#""[1..2]""#);
}
