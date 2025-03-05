use dsntk_examples::decision_tables::*;
use dsntk_recognizer::recognize_from_markdown;

#[test]
fn test_v_111222() {
  recognize_from_markdown(V_111222, false).unwrap();
}
