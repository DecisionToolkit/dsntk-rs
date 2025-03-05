use dsntk_examples::decision_tables::*;
use dsntk_recognizer::recognize_from_markdown;

#[test]
fn test_h_000010() {
  recognize_from_markdown(H_000010, false).unwrap();
}
