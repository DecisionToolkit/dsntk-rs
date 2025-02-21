use dsntk_common::HRef;

#[test]
fn retrieving_namespace_should_work() {
  let href = HRef::try_from("documents#_b51ac78b-fd76-42fc-a12d-aad7150c9278").unwrap();
  assert_eq!("documents", href.namespace().unwrap());
}

#[test]
fn retrieving_id_should_work() {
  let href = HRef::try_from("documents#_b51ac78b-fd76-42fc-a12d-aad7150c9278").unwrap();
  assert_eq!("_b51ac78b-fd76-42fc-a12d-aad7150c9278", href.id());
}

#[test]
fn test_relative_references() {
  assert_eq!(r#"Err(DsntkError("<HRefError> no fragment in reference: ''"))"#, format!("{:?}", HRef::try_from("")));
  assert_eq!(
    r#"Err(DsntkError("<HRefError> no fragment in reference: 'documents'"))"#,
    format!("{:?}", HRef::try_from("documents"))
  );
  assert_eq!(
    r#"Ok(HRef { namespace: Some("documents"), id: "_b51ac78b-fd76-42fc-a12d-aad7150c9278" })"#,
    format!("{:?}", HRef::try_from("documents#_b51ac78b-fd76-42fc-a12d-aad7150c9278"))
  );
  assert_eq!(
    r#"Ok(HRef { namespace: None, id: "_b51ac78b-fd76-42fc-a12d-aad7150c9278" })"#,
    format!("{:?}", HRef::try_from("#_b51ac78b-fd76-42fc-a12d-aad7150c9278"))
  );
  assert_eq!(
    r#"Err(DsntkError("<HRefError> invalid reference: 'documents?name=Introduction#_b51ac78b-fd76-42fc-a12d-aad7150c9278'"))"#,
    format!("{:?}", HRef::try_from("documents?name=Introduction#_b51ac78b-fd76-42fc-a12d-aad7150c9278"))
  );

  assert_eq!(
    r#"Err(DsntkError("<HRefError> invalid reference: 'documents?name=Introduction#_b51ac78b-fd76-42fc-a12d-aad7150c9278?name=john'"))"#,
    format!("{:?}", HRef::try_from("documents?name=Introduction#_b51ac78b-fd76-42fc-a12d-aad7150c9278?name=john"))
  );
}

#[test]
fn test_absolute_references() {
  assert_eq!(r#"Err(DsntkError("<HRefError> no fragment in reference: ''"))"#, format!("{:?}", HRef::try_from("")));
  assert_eq!(
    r#"Err(DsntkError("<HRefError> invalid reference: '                                               '"))"#,
    format!("{:?}", HRef::try_from("                                               "))
  );
  assert_eq!(
    r#"Ok(HRef { namespace: Some("https://dsntk.io/documents"), id: "_b51ac78b-fd76-42fc-a12d-aad7150c9278" })"#,
    format!("{:?}", HRef::try_from("https://dsntk.io/documents#_b51ac78b-fd76-42fc-a12d-aad7150c9278"))
  );
  assert_eq!(
    r#"Err(DsntkError("<HRefError> no fragment in reference: 'https://dsntk.io/documents'"))"#,
    format!("{:?}", HRef::try_from("https://dsntk.io/documents"))
  );
  assert_eq!(
    r#"<HRefError> invalid reference: 'https::\/dsntk.io/documents#id'"#,
    HRef::try_from(r#"https::\/dsntk.io/documents#id"#).unwrap_err().to_string()
  );
  assert_eq!(
    r#"<HRefError> invalid reference: 'https://dsntk.io/documents?name=john'"#,
    HRef::try_from(r#"https://dsntk.io/documents?name=john"#).unwrap_err().to_string()
  );
}
