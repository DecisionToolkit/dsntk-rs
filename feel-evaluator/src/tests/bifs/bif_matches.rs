use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_bool(false, &scope!(), r#"matches("foobar","^fo*b")"#, true);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"matches(input: "foobar", pattern: "^fo*b")"#, true);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"matches("abracadabra","bra")"#, true);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"matches("abracadabra","^a.*a$")"#, true);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"matches("abracadabra","^bra")"#, false);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"matches("hello\nworld","hello.*world")"#, false);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"matches("hello\nworld","hello.*world","s")"#, true);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"matches(input: "hello\nworld", pattern: "hello.*world", flags: "s")"#, true);
}

#[test]
fn _0009() {
  let scope = &te_scope(
    r#"{poem:"<poem author=\"Wilhelm Busch\">\nKaum hat dies der Hahn gesehen,\nFängt er auch schon an zu krähen:\nKikeriki! Kikikerikih!!\nTak, tak, tak! - da kommen sie.\n</poem>"}"#,
  );
  te_bool(false, scope, r#"matches(poem, "Kaum.*krähen")"#, false);
}

#[test]
fn _0010() {
  te_null(false, &scope!(), r#"matches()"#, r#"expected 2,3 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0011() {
  te_null(false, &scope!(), r#"matches("abc")"#, r#"expected 2,3 parameters, actual number of parameters is 1"#);
}

#[test]
fn _0012() {
  te_null(
    false,
    &scope!(),
    r#"matches("abc","a","b","c")"#,
    r#"expected 2,3 parameters, actual number of parameters is 4"#,
  );
}

#[test]
fn _0013() {
  te_null(false, &scope!(), r#"matches(i: "foobar", pattern: "^fo*b")"#, r#"parameter 'input' not found"#);
}

#[test]
fn _0014() {
  te_null(false, &scope!(), r#"matches(input: "foobar", p: "^fo*b")"#, r#"parameter 'pattern' not found"#);
}

#[test]
fn _0015() {
  te_null(false, &scope!(), r#"matches(input: 10, pattern: "^fo*b")"#, r#"matches"#);
}

#[test]
fn _0016() {
  te_null(false, &scope!(), r#"matches(input: "foobar", pattern: true)"#, r#"matches"#);
}
