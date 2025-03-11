use dsntk_examples::decision_tables::*;
use dsntk_recognizer::from_unicode;

#[test]
fn test_h_000010() {
  from_unicode(H_000010, false).unwrap();
}
