use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_number(false, &scope!(), r#"floor(1.5)"#, 1, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), r#"floor(-1.5)"#, -2, 0);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), r#"floor(--1)"#, 1, 0);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), r#"floor(-5/2.3*5)"#, -11, 0);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), r#"floor(n: 1.5)"#, 1, 0);
}

#[test]
fn _0006() {
  te_null(
    false,
    &scope!(),
    r#"floor(true)"#,
    r#"[core::floor] invalid argument type, expected number, actual type is boolean"#,
  );
}

#[test]
fn _0007() {
  te_null(false, &scope!(), r#"floor(number:2.5)"#, r#"parameter 'n' not found"#);
}

#[test]
fn _0008() {
  te_null(false, &scope!(), r#"floor()"#, r#"expected 1 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0009() {
  te_null(false, &scope!(), r#"floor(-2.45,2)"#, r#"expected 1 parameters, actual number of parameters is 2"#);
}
