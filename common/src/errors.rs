//! # Definition of the common error type

use std::{any, fmt};

/// Common result type.
pub type Result<T, E = DsntkError> = std::result::Result<T, E>;

/// Common trait to be implemented by structs defining a specific error.
pub trait ToErrorMessage {
  /// Convert error definition to message string.
  fn message(self) -> String;
}

/// Error definition used by all components of this project.
#[derive(Debug, PartialEq, Eq)]
pub struct DsntkError(String);

impl fmt::Display for DsntkError {
  /// Implementation of [Display](fmt::Display) trait for [DsntkError].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl DsntkError {
  /// Creates a new [DsntkError] with specified source name and error message.
  pub fn new(source: &str, message: &str) -> Self {
    Self(format!("<{source}> {message}"))
  }
}

impl<T> From<T> for DsntkError
where
  T: ToErrorMessage,
{
  /// Converts any type that implements [ToErrorMessage] trait to [DsntkError].
  fn from(value: T) -> Self {
    let error_type_name = any::type_name::<T>().split("::").last().unwrap_or("UnknownError");
    DsntkError::new(error_type_name, &value.message())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    assert_eq!("<TestError> unexpected", format!("{}", DsntkError::new("TestError", "unexpected")));
  }

  #[test]
  fn test_debug() {
    assert_eq!(r#"DsntkError("<TestError> unexpected")"#, format!("{:?}", DsntkError::new("TestError", "unexpected")));
  }

  #[test]
  fn test_equal() {
    let err1 = DsntkError::new("TestError", "unexpected");
    let err2 = DsntkError::new("TestError", "unexpected");
    assert!((err1 == err2));
  }

  #[test]
  fn test_not_equal() {
    let err1 = DsntkError::new("TestError", "expected");
    let err2 = DsntkError::new("TestError", "unexpected");
    assert!((err1 != err2));
  }

  #[test]
  fn test_total_eq() {
    DsntkError::new("TestError", "unexpected").assert_receiver_is_total_eq();
  }
}
