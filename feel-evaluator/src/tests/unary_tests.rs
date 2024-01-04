use super::*;
use dsntk_feel::scope;

#[test]
fn test_0001() {
  valid_unary_tests(false, &scope!(), "-");
}

#[test]
fn test_0002() {
  valid_unary_tests(false, &scope!(), "1");
}

#[test]
fn test_0003() {
  valid_unary_tests(false, &scope!(), "1,2");
}

#[test]
fn test_0004() {
  valid_unary_tests(false, &scope!(), "1,2,3");
}

#[test]
fn test_0005() {
  valid_unary_tests(false, &scope!(), "[]");
}

#[test]
fn test_0006() {
  valid_unary_tests(false, &scope!(), "[1]");
}

#[test]
fn test_0007() {
  valid_unary_tests(false, &scope!(), "[1,2]");
}

#[test]
fn test_0008() {
  valid_unary_tests(false, &scope!(), "[1,2,3]");
}

#[test]
fn test_0009() {
  valid_unary_tests(false, &scope!(), "[1..2]");
}

#[test]
fn test_0010() {
  valid_unary_tests(false, &scope!(), "not(1,2,3)");
}

#[test]
fn test_0011() {
  valid_unary_tests(false, &scope!(), "<1");
}

#[test]
fn test_0012() {
  valid_unary_tests(false, &scope!(), "<=1");
}

#[test]
fn test_0013() {
  valid_unary_tests(false, &scope!(), ">1");
}

#[test]
fn test_0014() {
  valid_unary_tests(false, &scope!(), ">=1");
}

#[test]
fn test_0015() {
  valid_unary_tests(false, &scope!(), "=1");
}

#[test]
fn test_0016() {
  valid_unary_tests(false, &scope!(), "!=1");
}
