use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_be_value(false, &scope!(), r#"range("[1..2]")"#, "[1..2]");
}

#[test]
fn _0002() {
  te_be_value(false, &scope!(), r#"range("(1..2]")"#, "(1..2]");
}

#[test]
fn _0003() {
  te_be_value(false, &scope!(), r#"range("]1..2]")"#, "(1..2]");
}

#[test]
fn _0004() {
  te_be_value(false, &scope!(), r#"range("[1..2)")"#, "[1..2)");
}

#[test]
fn _0005() {
  te_be_value(false, &scope!(), r#"range("[1..2[")"#, "[1..2)");
}

#[test]
fn _0006() {
  te_be_value(false, &scope!(), r#"range("(1..2)")"#, "(1..2)");
}

#[test]
fn _0007() {
  te_be_value(false, &scope!(), r#"range("]1..2[")"#, "(1..2)");
}

#[test]
fn _0008() {
  te_be_value(false, &scope!(), r#"range("[1..)")"#, "[1..null)");
}

#[test]
fn _0009() {
  te_be_value(false, &scope!(), r#"range(from: "[1..2]")"#, "[1..2]");
}
