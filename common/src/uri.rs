//! # URI

use self::errors::*;
use crate::Result;
use std::convert::TryFrom;
use uriparse::{URIReference, URI};

pub type Uri = String;

pub fn to_uri(value: &str) -> Result<Uri> {
  if let Ok(uri_reference) = URIReference::try_from(value) {
    if let Ok(uri) = URI::try_from(uri_reference) {
      if uri.has_query() || uri.has_fragment() {
        return Err(err_invalid_uri(value));
      }
      return Ok(uri.to_string().trim().trim_end_matches('/').to_string());
    }
  }
  Err(err_invalid_uri(value))
}

mod errors {
  use crate::{DsntkError, ToErrorMessage};

  /// Errors reported by [Uri](crate::uri::Uri).
  #[derive(ToErrorMessage)]
  struct UriError(String);

  /// Creates an error indicating an invalid URI.
  pub fn err_invalid_uri(s: &str) -> DsntkError {
    UriError(format!("invalid reference: '{s}'")).into()
  }
}

#[cfg(test)]
mod tests {
  use crate::to_uri;

  #[test]
  fn test_empty_uri() {
    assert!(to_uri("").is_err())
  }
}
