//! # URI reference
//!
//! This [HRef] struct utilizes an **href** attribute whose value must be a valid URI reference
//! [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986), where the path components
//! may be absolute or relative, the reference has no query component,
//! and the fragment consists of the value of the **id** of the referenced DMN element.

use self::errors::*;
use crate::DsntkError;
use uriparse::{RelativeReference, URIReference, URI};

/// URI reference used for utilizing `href` attribute.
#[derive(Debug, Clone)]
pub struct HRef {
  /// Namespace built from URI's path components.
  namespace: Option<String>,
  /// DMN element's identifier built from URI's fragment.
  id: String,
}

impl HRef {
  /// Returns the optional namespace.
  pub fn namespace(&self) -> Option<&String> {
    self.namespace.as_ref()
  }

  /// Returns the identifier.
  pub fn id(&self) -> &str {
    &self.id
  }
}

impl TryFrom<&str> for HRef {
  type Error = DsntkError;
  /// Converts [HRef] from string.
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Ok(reference) = RelativeReference::try_from(value) {
      if reference.has_query() {
        return Err(err_invalid_reference(value));
      }
      let id = reference.fragment().ok_or_else(|| err_invalid_reference_no_fragment(value))?.to_string();
      let path = reference.path().to_string().trim().trim_end_matches('/').to_string();
      let namespace = if path.is_empty() { None } else { Some(path) };
      return Ok(Self { namespace, id });
    }
    if let Ok(uri_reference) = URIReference::try_from(value) {
      if let Ok(uri) = URI::try_from(uri_reference) {
        if uri.has_query() {
          return Err(err_invalid_reference(value));
        }
        let id = uri.fragment().ok_or_else(|| err_invalid_reference_no_fragment(value))?.to_string();
        let path = uri.into_base_uri().to_string().trim().trim_end_matches('/').to_string();
        let namespace = if path.is_empty() { None } else { Some(path) };
        return Ok(Self { namespace, id });
      }
    }
    Err(err_invalid_reference(value))
  }
}

mod errors {
  use crate::{DsntkError, ToErrorMessage};

  /// Errors reported by [HRef](crate::href::HRef).
  #[derive(ToErrorMessage)]
  struct HRefError(String);

  /// Creates an error indicating an invalid reference.
  pub fn err_invalid_reference(s: &str) -> DsntkError {
    HRefError(format!("invalid reference: '{s}'")).into()
  }

  /// Creates an error indicating the missing fragment.
  pub fn err_invalid_reference_no_fragment(s: &str) -> DsntkError {
    HRefError(format!("no fragment in reference: '{s}'")).into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
      r#"Err(DsntkError("<HRefError> invalid reference: 'https::\\/dsntk.io/documents#id'"))"#,
      format!("{:?}", HRef::try_from("https::\\/dsntk.io/documents#id"))
    );
  }
}
