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
  te_null(false, &scope!(), r#"floor(num: 1.5)"#, "parameter 'n' not found");
}

#[test]
fn _0007() {
  te_number(false, &scope!(), r#"floor(1.56, 1)"#, 15, 1);
}
#[test]
fn _0008() {
  te_number(false, &scope!(), r#"floor(-1.56, 1)"#, -16, 1);
}

#[test]
fn _0009() {
  te_number(false, &scope!(), r#"floor(n: -1.56, scale: 1)"#, -16, 1);
}

#[test]
fn _0010() {
  te_null(
    false,
    &scope!(),
    r#"floor(true)"#,
    r#"[core::floor] invalid argument type, expected number, actual type is boolean"#,
  );
}

#[test]
fn _0011() {
  te_null(false, &scope!(), r#"floor(number:2.5)"#, r#"parameter 'n' not found"#);
}

#[test]
fn _0012() {
  te_null(false, &scope!(), r#"floor()"#, r#"expected 1,2 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0013() {
  te_null(false, &scope!(), r#"floor(-2.45,2,6)"#, r#"expected 1,2 parameters, actual number of parameters is 3"#);
}

#[test]
fn _0014() {
  te_null(
    false,
    &scope!(),
    r#"floor(null,2)"#,
    r#"[core::floor] invalid argument type, expected number, actual type is Null"#,
  );
}

#[test]
fn _0015() {
  te_null(
    false,
    &scope!(),
    r#"floor(1.56,null)"#,
    r#"[core::floor] invalid argument type, expected number, actual type is Null"#,
  );
}

#[test]
fn _0016() {
  te_null(false, &scope!(), r#"floor(num: 1.56, scale: 1)"#, "parameter 'n' not found");
}

#[test]
fn _0017() {
  te_null(false, &scope!(), r#"floor(n: 1.56, s: 1)"#, "parameter 'scale' not found");
}

#[test]
fn _0018() {
  te_null(
    false,
    &scope!(),
    r#"floor(n: 1.56, scale: 1, foo: 5)"#,
    "expected 2 parameters, actual number of parameters is 3",
  );
}

#[test]
fn _0019() {
  te_null(false, &scope!(), r#"floor(n: 1.56, scale: 2147483648)"#, "invalid scale: 2147483648");
}

#[test]
fn _0020() {
  //--------------------------------------------------------------------------------------------------------------------
  // Now the scale is truncated to integer.
  // If non-integer scale should be reported as an error then this test should be modified.
  //--------------------------------------------------------------------------------------------------------------------
  te_number(false, &scope!(), "floor(n: 1.56, scale: 1.28)", 15, 1);
}
