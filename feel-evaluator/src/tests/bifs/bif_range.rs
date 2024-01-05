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
  te_be_value(false, &scope!(), r#"range("[1..[")"#, "[1..null)");
}

#[test]
fn _0010() {
  te_null(false, &scope!(), r#"range("[1..]")"#, "invalid range literal");
}

#[test]
fn _0011() {
  te_be_value(false, &scope!(), r#"range("(..1]")"#, "(null..1]");
}

#[test]
fn _0012() {
  te_be_value(false, &scope!(), r#"range("]..1]")"#, "(null..1]");
}

#[test]
fn _0013() {
  te_null(false, &scope!(), r#"range("[..1]")"#, "invalid range literal");
}

#[test]
fn _0014() {
  te_be_value(false, &scope!(), r#"range(from:"[1..2]")"#, "[1..2]");
}

#[test]
fn _0015() {
  te_be_value(false, &scope!(), r#"range(from:"(1..2]")"#, "(1..2]");
}

#[test]
fn _0016() {
  te_be_value(false, &scope!(), r#"range(from:"]1..2]")"#, "(1..2]");
}

#[test]
fn _0017() {
  te_be_value(false, &scope!(), r#"range(from:"[1..2)")"#, "[1..2)");
}

#[test]
fn _0018() {
  te_be_value(false, &scope!(), r#"range(from:"[1..2[")"#, "[1..2)");
}

#[test]
fn _0019() {
  te_be_value(false, &scope!(), r#"range(from:"(1..2)")"#, "(1..2)");
}

#[test]
fn _0020() {
  te_be_value(false, &scope!(), r#"range(from:"]1..2[")"#, "(1..2)");
}

#[test]
fn _0021() {
  te_be_value(false, &scope!(), r#"range(from:"[1..)")"#, "[1..null)");
}

#[test]
fn _0022() {
  te_be_value(false, &scope!(), r#"range(from:"[1..[")"#, "[1..null)");
}

#[test]
fn _0023() {
  te_null(false, &scope!(), r#"range(from:"[1..]")"#, "invalid range literal");
}

#[test]
fn _0024() {
  te_be_value(false, &scope!(), r#"range(from:"(..1]")"#, "(null..1]");
}

#[test]
fn _0025() {
  te_be_value(false, &scope!(), r#"range(from:"]..1]")"#, "(null..1]");
}

#[test]
fn _0026() {
  te_null(false, &scope!(), r#"range(from:"[..1]")"#, "invalid range literal");
}
