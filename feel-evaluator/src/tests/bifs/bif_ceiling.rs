use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_number(false, &scope!(), "ceiling(1.5)", 2, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), "ceiling(-1.5)", -1, 0);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), "ceiling(--1)", 1, 0);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), "ceiling(-5/2.3*5)", -10, 0);
}
#[test]
fn _0005() {
  te_number(false, &scope!(), "ceiling(n:5.777)", 6, 0);
}
#[test]
fn _0006() {
  te_number(false, &scope!(), "ceiling(n:-.33333)", 0, 0);
}
#[test]
fn _0007() {
  te_number(false, &scope!(), "ceiling(n:.33333)", 1, 0);
}

#[test]
fn _0008() {
  let scope = &te_scope("{ Order size: 23.27 }");
  te_number(false, scope, "ceiling(n:Order size)", 24, 0);
}

#[test]
fn _0009() {
  te_null(false, &scope!(), "ceiling(number:5.777)", r#"parameter 'n' not found"#);
}

#[test]
fn _0010() {
  te_null(
    false,
    &scope!(),
    r#"ceiling(true)"#,
    r#"[core::ceiling] invalid argument type, expected number, actual type is boolean"#,
  );
}

#[test]
fn _0011() {
  te_null(false, &scope!(), r#"ceiling()"#, r#"expected 1,2 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0012() {
  te_null(false, &scope!(), r#"ceiling(1,2,3)"#, r#"expected 1,2 parameters, actual number of parameters is 3"#);
}
