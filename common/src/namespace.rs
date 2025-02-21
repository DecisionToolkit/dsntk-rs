//! # Conversion utilities for namespaces

use url::Url;

/// Converts an URI into its RDNN-like equivalent.
///
/// Unless the input namespace URI is invalid or does not contain a domain name,
/// this function returns its RDNN-like equivalent. Domain name segments are reversed,
/// path segments order is preserved, all segments are joined with
/// the forward slash `/` character.
///
/// Returns `None` for namespace URIs that can not be converted to its RDNN-like equivalent.
///
/// # Examples
///
/// ```
/// use dsntk_common::to_rdnn;
///
/// let rdnn = to_rdnn("https://dsntk.io/system-1/component-1");
/// assert_eq!(Some("io/dsntk/system-1/component-1".to_string()), rdnn);
///
/// let rdnn = to_rdnn("https://dsntk.io");
/// assert_eq!(Some("io/dsntk".to_string()), rdnn);
///
/// let rdnn = to_rdnn("dsntk.io");
/// assert_eq!(None, rdnn);
///
/// ```
pub fn to_rdnn(input: &str) -> Option<String> {
  let Ok(url) = Url::parse(input) else {
    return None;
  };
  let segments = url.path_segments()?;
  let mut path_segments = segments.map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>();
  let domain = url.domain()?;
  let mut domain_segments = domain.split('.').collect::<Vec<&str>>();
  domain_segments.reverse();
  domain_segments.append(&mut path_segments);
  Some(domain_segments.join("/"))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_rdnn(expected: &str, actual: Option<String>) {
    let value = actual.unwrap();
    let url = format!("https://dsntk.io/{}", value);
    assert!(Url::parse(&url).is_ok());
    assert_eq!(expected, value);
  }

  #[test]
  fn test_to_rdnn() {
    assert_rdnn("au/com/montera/www/spec/DMN/0099-arithmetic-negation", to_rdnn("https://www.montera.com.au/spec/DMN/0099-arithmetic-negation"));
    assert_rdnn("au/com/montera/www/spec/DMN/0099-arithmetic-negation", to_rdnn("https://www.montera.com.au/spec/DMN/0099-arithmetic-negation#_427d2e23-d096-47d5-b861-ebb7f37f461e"));
    assert_rdnn("au/com/montera/www", to_rdnn("https://www.montera.com.au"));
    assert_rdnn("io/dsntk/model-2/decision-5", to_rdnn("https://dsntk.io/model-2/decision-5"));
    assert_eq!(None, to_rdnn("https://www.montera.com.au|error"));
    assert_eq!(None, to_rdnn("https://127.0.0.1"));
    assert_eq!(None, to_rdnn("montera.com"));
    assert_eq!(None, to_rdnn("data:text/plain,HelloWorld"));
  }
}
