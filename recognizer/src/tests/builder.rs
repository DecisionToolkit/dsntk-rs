use crate::builder;
use crate::tests::{EX_01, EX_07};
use dsntk_examples::decision_tables::H_000010;

#[test]
fn test_dt_0001() {
  let _decision_table = builder::recognize_decision_table(H_000010, false).unwrap();
  // FIXME change to assert
  // println!("{}", decision_table);
}

#[test]
fn ex_0001() {
  let _decision_table = builder::recognize_decision_table(EX_01, false).unwrap();
  // FIXME change to assert
  // println!("{}", decision_table);
}

#[test]
fn ex_0064() {
  let _decision_table = builder::recognize_decision_table(EX_07, false).unwrap();
  // FIXME change to assert
  // println!("{}", decision_table);
}
