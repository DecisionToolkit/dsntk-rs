use dsntk_feel_regex::is_match;

#[test]
fn _0001() {
  assert!(is_match(r#"Mr\. Obama"#, ".+Obama").unwrap());
}

#[test]
#[cfg(feature = "onig")]
fn _0002() {
  assert!(is_match(r#"ababb"#, r#"(a(b))\1\2"#).unwrap());
}
