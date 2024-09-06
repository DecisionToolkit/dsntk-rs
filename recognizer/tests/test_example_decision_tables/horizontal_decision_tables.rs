use dsntk_examples::decision_tables::*;
use dsntk_recognizer::recognize;

#[test]
fn test_h_000010() {
  let _ = recognize(H_000010, false);
}
