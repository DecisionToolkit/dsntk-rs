use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_number(false, &scope!(), "round up(5.0, 0)", 5, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), "round up(5.1, 0)", 6, 0);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), "round up(5.5, 0)", 6, 0);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), "round up(5.6, 0)", 6, 0);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), "round up(5.9, 0)", 6, 0);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), "round up(-5.0, 0)", -5, 0);
}

#[test]
fn _0007() {
  te_number(false, &scope!(), "round up(-5.1, 0)", -6, 0);
}

#[test]
fn _0008() {
  te_number(false, &scope!(), "round up(-5.5, 0)", -6, 0);
}

#[test]
fn _0009() {
  te_number(false, &scope!(), "round up(-5.6, 0)", -6, 0);
}

#[test]
fn _0010() {
  te_number(false, &scope!(), "round up(-5.9, 0)", -6, 0);
}

#[test]
fn _0011() {
  te_number(false, &scope!(), "round up(1.121, 2)", 113, 2);
}

#[test]
fn _0012() {
  te_number(false, &scope!(), "round up(1.126, 2)", 113, 2);
}

#[test]
fn _0013() {
  te_number(false, &scope!(), "round up(-1.121, 2)", -113, 2);
}

#[test]
fn _0014() {
  te_number(false, &scope!(), "round up(-1.126, 2)", -113, 2);
}
