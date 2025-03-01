//! # URI reference
//!
//! This [HRef] struct utilizes an **href** attribute whose value must be a valid URI reference
//! [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986), where the path components
//! may be absolute or relative, the reference has no query component,
//! and the fragment consists of the value of the **id** of the referenced DMN element.

use self::errors::*;
use crate::DsntkError;
use uriparse::{URIReference, URI};

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
  fn try_from(s: &str) -> Result<Self, Self::Error> {
    if let Ok(uri_reference) = URIReference::try_from(s) {
      if uri_reference.has_query() {
        return Err(err_invalid_reference(s));
      }
      let id = uri_reference.fragment().ok_or_else(|| err_invalid_reference_no_fragment(s))?.to_string();
      let namespace = if uri_reference.is_uri() {
        let uri = URI::try_from(uri_reference).unwrap();
        Some(uri.into_base_uri().to_string().trim().trim_end_matches('/').to_string())
      } else {
        let path = uri_reference.path().to_string().trim().trim_end_matches('/').to_string();
        if path.is_empty() {
          None
        } else {
          Some(path)
        }
      };
      return Ok(Self { namespace, id });
    }
    Err(err_invalid_reference(s))
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
