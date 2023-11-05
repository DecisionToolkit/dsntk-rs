use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_be_value(false, &scope!(), r#"for n in [12,8,32] return n + 1"#, "[13,9,33]");
}

#[test]
fn _0002() {
  te_be_value(false, &scope!(), r#"for n in 1..10 return n - 1"#, "[0,1,2,3,4,5,6,7,8,9]");
}

#[test]
fn _0003() {
  let scope = &te_scope(r#"{N:4}"#);
  te_be_value(
    false,
    scope,
    r#"
      for i in 1..N return
        if i = 1 then
          1
        else
          i * partial[-1]
    "#,
    "[1,2,6,24]",
  );
}

#[test]
fn _0004() {
  let scope = &te_scope(r#"{N:5}"#);
  te_be_value(
    false,
    scope,
    r#"
      for i in 0..N return
        if i = 0 then
          1
        else if i = 1 then
          1 
        else 
          partial[-1] + partial[-2]
    "#,
    "[1,1,2,3,5,8]",
  );
}

#[test]
fn _0005() {
  te_be_value(
    false,
    &scope!(),
    r#"
      for x in [1,2,3],
          y in [6,7,8]
      return x + y
    "#,
    "[7,8,9,8,9,10,9,10,11]",
  );
}

#[test]
fn _0006() {
  te_be_value(
    false,
    &scope!(),
    r#"for x in [1,2,3], y in [6,7,8], z in [-1,-2,-3] return x + y + z"#,
    "[6,5,4,7,6,5,8,7,6,7,6,5,8,7,6,9,8,7,8,7,6,9,8,7,10,9,8]",
  );
}

#[test]
fn _0007() {
  te_be_value(false, &scope!(), r#"for n in 4..2 return n"#, "[4,3,2]");
}
