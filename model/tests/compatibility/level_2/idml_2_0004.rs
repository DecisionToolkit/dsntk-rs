use dsntk_examples::*;
use dsntk_model::*;

#[test]
fn _0001() {
  let xml = from_xml(DMN_2_0004).unwrap();
  let idml = from_idml(IDML_2_0004).unwrap();
  assert_eq!(xml, idml);
}
