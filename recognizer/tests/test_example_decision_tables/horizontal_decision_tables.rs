use dsntk_examples::decision_tables::*;
use dsntk_recognizer::recognize_decision_table;

#[test]
fn test_h_000010() {
  let _ = recognize_decision_table(H_000010, false);
}
