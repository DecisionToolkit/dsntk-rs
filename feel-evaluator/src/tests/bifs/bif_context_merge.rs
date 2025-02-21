use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_bool(false, &scope!(), r#"contains("foobar","ob")"#, true);
}

#[test]
fn _0002() {
  te_null(false, &scope!(), r#"context merge([1,{}])"#, "[core::context merge] invalid argument type, expected context, actual type is number");
}

#[test]
fn _0003() {
  te_null(false, &scope!(), r#"context merge(1)"#, "[core::context merge] invalid argument type, expected list<context>, actual type is number");
}
