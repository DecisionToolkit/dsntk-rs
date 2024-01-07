use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_bool(false, &scope!(), r#"0 in range("[1..3]")"#, false);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"1 in range("[1..3]")"#, true);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"2 in range("[1..3]")"#, true);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"3 in range("[1..3]")"#, true);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"4 in range("[1..3]")"#, false);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"1 in range("(..1]")"#, true);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"-2024 in range("(..1]")"#, true);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"1 in range("[1..)")"#, true);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#"2024 in range("[1..)")"#, true);
}
