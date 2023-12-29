use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_number(false, &scope!(), "round down(5.0, 0)", 5, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), "round down(5.1, 0)", 5, 0);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), "round down(5.2, 0)", 5, 0);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), "round down(5.3, 0)", 5, 0);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), "round down(5.4, 0)", 5, 0);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), "round down(5.5, 0)", 5, 0);
}

#[test]
fn _0007() {
  te_number(false, &scope!(), "round down(5.6, 0)", 5, 0);
}

#[test]
fn _0008() {
  te_number(false, &scope!(), "round down(5.7, 0)", 5, 0);
}

#[test]
fn _0009() {
  te_number(false, &scope!(), "round down(5.8, 0)", 5, 0);
}

#[test]
fn _0010() {
  te_number(false, &scope!(), "round down(5.9, 0)", 5, 0);
}

#[test]
fn _0011() {
  te_number(false, &scope!(), "round down(-5.0, 0)", -5, 0);
}

#[test]
fn _0012() {
  te_number(false, &scope!(), "round down(-5.1, 0)", -5, 0);
}

#[test]
fn _0013() {
  te_number(false, &scope!(), "round down(-5.2, 0)", -5, 0);
}

#[test]
fn _0014() {
  te_number(false, &scope!(), "round down(-5.3, 0)", -5, 0);
}

#[test]
fn _0015() {
  te_number(false, &scope!(), "round down(-5.4, 0)", -5, 0);
}

#[test]
fn _0016() {
  te_number(false, &scope!(), "round down(-5.5, 0)", -5, 0);
}

#[test]
fn _0017() {
  te_number(false, &scope!(), "round down(-5.6, 0)", -5, 0);
}

#[test]
fn _0018() {
  te_number(false, &scope!(), "round down(-5.7, 0)", -5, 0);
}

#[test]
fn _0019() {
  te_number(false, &scope!(), "round down(-5.8, 0)", -5, 0);
}

#[test]
fn _0020() {
  te_number(false, &scope!(), "round down(-5.9, 0)", -5, 0);
}

#[test]
fn _0021() {
  te_number(false, &scope!(), "round down(1.120, 2)", 112, 2);
}

#[test]
fn _0022() {
  te_number(false, &scope!(), "round down(1.121, 2)", 112, 2);
}

#[test]
fn _0023() {
  te_number(false, &scope!(), "round down(1.122, 2)", 112, 2);
}

#[test]
fn _0024() {
  te_number(false, &scope!(), "round down(1.123, 2)", 112, 2);
}

#[test]
fn _0025() {
  te_number(false, &scope!(), "round down(1.124, 2)", 112, 2);
}

#[test]
fn _0026() {
  te_number(false, &scope!(), "round down(1.125, 2)", 112, 2);
}

#[test]
fn _0027() {
  te_number(false, &scope!(), "round down(1.126, 2)", 112, 2);
}

#[test]
fn _0028() {
  te_number(false, &scope!(), "round down(1.127, 2)", 112, 2);
}

#[test]
fn _0029() {
  te_number(false, &scope!(), "round down(1.128, 2)", 112, 2);
}

#[test]
fn _0030() {
  te_number(false, &scope!(), "round down(1.129, 2)", 112, 2);
}

#[test]
fn _0031() {
  te_number(false, &scope!(), "round down(-1.120, 2)", -112, 2);
}

#[test]
fn _0032() {
  te_number(false, &scope!(), "round down(-1.121, 2)", -112, 2);
}

#[test]
fn _0033() {
  te_number(false, &scope!(), "round down(-1.122, 2)", -112, 2);
}

#[test]
fn _0034() {
  te_number(false, &scope!(), "round down(-1.123, 2)", -112, 2);
}

#[test]
fn _0035() {
  te_number(false, &scope!(), "round down(-1.124, 2)", -112, 2);
}

#[test]
fn _0036() {
  te_number(false, &scope!(), "round down(-1.125, 2)", -112, 2);
}

#[test]
fn _0037() {
  te_number(false, &scope!(), "round down(-1.126, 2)", -112, 2);
}

#[test]
fn _0038() {
  te_number(false, &scope!(), "round down(-1.127, 2)", -112, 2);
}

#[test]
fn _0039() {
  te_number(false, &scope!(), "round down(-1.128, 2)", -112, 2);
}

#[test]
fn _0040() {
  te_number(false, &scope!(), "round down(-1.129, 2)", -112, 2);
}

#[test]
fn _0041() {
  //--------------------------------------------------------------------------------------------------------------------
  // Use maximum allowed scale.
  //--------------------------------------------------------------------------------------------------------------------
  te_number(false, &scope!(), "round down(5.5, 6144)", 55, 1);
}

#[test]
fn _0042() {
  //--------------------------------------------------------------------------------------------------------------------
  // Scale is out of range.
  //--------------------------------------------------------------------------------------------------------------------
  te_null(
    false,
    &scope!(),
    "round down(5.5, 6145)",
    "[core::round_down] <FeelNumberError> invalid scale, allowed range is -6111..6144, actual is 6145",
  );
}
