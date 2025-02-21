use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_be_value(false, &scope!(), r#"range("[1..2]")"#, "[1..2]");
}

#[test]
fn _0003() {
  te_be_value(false, &scope!(), r#"range("(1..2]")"#, "(1..2]");
}

#[test]
fn _0004() {
  te_be_value(false, &scope!(), r#"range("]1..2]")"#, "(1..2]");
}

#[test]
fn _0005() {
  te_be_value(false, &scope!(), r#"range("[1..2)")"#, "[1..2)");
}

#[test]
fn _0006() {
  te_be_value(false, &scope!(), r#"range("[1..2[")"#, "[1..2)");
}

#[test]
fn _0007() {
  te_be_value(false, &scope!(), r#"range("(1..2)")"#, "(1..2)");
}

#[test]
fn _0008() {
  te_be_value(false, &scope!(), r#"range("]1..2[")"#, "(1..2)");
}

#[test]
fn _0009() {
  te_be_value(false, &scope!(), r#"range("[1..)")"#, "[1..null)");
}

#[test]
fn _0010() {
  te_be_value(false, &scope!(), r#"range("[1..[")"#, "[1..null)");
}

#[test]
fn _0011() {
  te_null(false, &scope!(), r#"range("[1..]")"#, "invalid range literal");
}

#[test]
fn _0012() {
  te_be_value(false, &scope!(), r#"range("(..1]")"#, "(null..1]");
}

#[test]
fn _0014() {
  te_be_value(false, &scope!(), r#"range("]..1]")"#, "(null..1]");
}

#[test]
fn _0015() {
  te_null(false, &scope!(), r#"range("[..1]")"#, "invalid range literal");
}

#[test]
fn _0016() {
  te_null(false, &scope!(), r#"range()"#, "expected 1 parameters, actual number of parameters is 0");
}

#[test]
fn _0017() {
  te_null(false, &scope!(), r#"range("[1..2]","[3..4]")"#, "expected 1 parameters, actual number of parameters is 2");
}

#[test]
fn _0018() {
  te_null(false, &scope!(), r#"range(1)"#, "[core::range] invalid argument type, expected string, actual type is number");
}

#[test]
fn _0019() {
  te_be_value(false, &scope!(), r#"range(from:"[1..2]")"#, "[1..2]");
}

#[test]
fn _0020() {
  te_be_value(false, &scope!(), r#"range(from:"(1..2]")"#, "(1..2]");
}

#[test]
fn _0021() {
  te_be_value(false, &scope!(), r#"range(from:"]1..2]")"#, "(1..2]");
}

#[test]
fn _0022() {
  te_be_value(false, &scope!(), r#"range(from:"[1..2)")"#, "[1..2)");
}

#[test]
fn _0023() {
  te_be_value(false, &scope!(), r#"range(from:"[1..2[")"#, "[1..2)");
}

#[test]
fn _0024() {
  te_be_value(false, &scope!(), r#"range(from:"(1..2)")"#, "(1..2)");
}

#[test]
fn _0025() {
  te_be_value(false, &scope!(), r#"range(from:"]1..2[")"#, "(1..2)");
}

#[test]
fn _0026() {
  te_be_value(false, &scope!(), r#"range(from:"[1..)")"#, "[1..null)");
}

#[test]
fn _0029() {
  te_be_value(false, &scope!(), r#"range(from:"[1..[")"#, "[1..null)");
}

#[test]
fn _0030() {
  te_null(false, &scope!(), r#"range(from:"[1..]")"#, "invalid range literal");
}

#[test]
fn _0031() {
  te_be_value(false, &scope!(), r#"range(from:"(..1]")"#, "(null..1]");
}

#[test]
fn _0032() {
  te_be_value(false, &scope!(), r#"range(from:"]..1]")"#, "(null..1]");
}

#[test]
fn _0033() {
  te_null(false, &scope!(), r#"range(from:"[..1]")"#, "invalid range literal");
}

#[test]
fn _0034() {
  te_null(false, &scope!(), r#"range(to:"[1..2]")"#, "parameter 'from' not found");
}

#[test]
fn _0035() {
  te_null(false, &scope!(), r#"range(from:"[1..2]",to:"[3..4]")"#, "expected 1 parameters, actual number of parameters is 2");
}

#[test]
fn _0036() {
  te_null(false, &scope!(), r#"range(from:1)"#, "[core::range] invalid argument type, expected string, actual type is number");
}
