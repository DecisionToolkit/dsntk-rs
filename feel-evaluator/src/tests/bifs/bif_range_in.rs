use super::super::*;
use dsntk_feel::scope;

//----------------------------------------------------------------------------------------------------------------------
// NUMBER
//----------------------------------------------------------------------------------------------------------------------

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

//----------------------------------------------------------------------------------------------------------------------
// STRING
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#" "a" in range("[\"b\"..\"d\"]") "#, false);
}

#[test]
fn _0011() {
  te_bool(false, &scope!(), r#" "b" in range("[\"b\"..\"d\"]") "#, true);
}

#[test]
fn _0012() {
  te_bool(false, &scope!(), r#" "c" in range("[\"b\"..\"d\"]") "#, true);
}

#[test]
fn _0013() {
  te_bool(false, &scope!(), r#" "d" in range("[\"b\"..\"d\"]") "#, true);
}

#[test]
fn _0014() {
  te_bool(false, &scope!(), r#" "e" in range("[\"b\"..\"d\"]") "#, false);
}

#[test]
fn _0015() {
  te_bool(false, &scope!(), r#" "d" in range("(..\"d\"]") "#, true);
}

#[test]
fn _0016() {
  te_bool(false, &scope!(), r#" "a" in range("(..\"d\"]") "#, true);
}

#[test]
fn _0017() {
  te_bool(false, &scope!(), r#" "b" in range("[\"b\"..)") "#, true);
}

#[test]
fn _0018() {
  te_bool(false, &scope!(), r#" "z" in range("[\"b\"..)") "#, true);
}
