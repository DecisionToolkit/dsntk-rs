use dsntk_common::{DsntkError, ToErrorMessage};

/// `FEEL` number errors.
#[derive(ToErrorMessage)]
struct FeelNumberError(String);

/// Creates invalid number literal error.
pub fn err_invalid_number_literal(s: &str) -> DsntkError {
  FeelNumberError(format!("invalid number literal '{s}'")).into()
}

/// Creates number conversion error.
pub fn err_number_conversion_failed() -> DsntkError {
  FeelNumberError("number conversion failed".to_string()).into()
}
