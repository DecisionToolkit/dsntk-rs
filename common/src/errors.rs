//! # Definition of the common result and error types

use std::fmt::Display;
use std::{any, fmt};

/// Common result type.
pub type Result<T, E = DsntkError> = std::result::Result<T, E>;

/// Common trait to be implemented by structs defining a specific error.
pub trait ToErrorMessage {
  /// Convert error definition to message string.
  fn message(self) -> String;
}

/// Error definition used by all components.
#[derive(Debug, PartialEq, Eq)]
pub struct DsntkError(String);

impl Display for DsntkError {
  /// Implementation of [Display] trait for [DsntkError].
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
