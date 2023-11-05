mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_floor_001() {
  eqs!("1", num!(1.5).floor());
}

#[test]
fn test_floor_002() {
  eqs!("-2", num!(-1.5).floor());
}

#[test]
fn test_floor_003() {
  eqs!("0", num!(0.333).floor());
}

#[test]
fn test_floor_004() {
  eqs!("-1", num!(-0.3333).floor());
}
