use crate::errors::*;
use crate::DmnId;
use dsntk_common::{gen_id, to_uri, trim_multiline, HRef, Result, Uri};
use dsntk_feel::Name;
use yaml_rust::Yaml;

pub const YAML_ALLOWED_ANSWERS: &str = "allowedAnswers";
pub const YAML_DECISION: &str = "decision";
pub const YAML_DEFINITIONS: &str = "definitions";
const YAML_DESCRIPTION: &str = "description";
pub const YAML_EXPRESSION_LANGUAGE: &str = "expressionLanguage";
pub const YAML_HREF: &str = "href";
const YAML_LABEL: &str = "label";
pub const YAML_ID: &str = "id";
pub const YAML_INFORMATION_REQUIREMENT: &str = "informationRequirement";
pub const YAML_INPUT_DATA: &str = "inputData";
pub const YAML_LITERAL_EXPRESSION: &str = "literalExpression";
pub const YAML_NAME: &str = "name";
pub const YAML_NAMESPACE: &str = "namespace";
pub const YAML_QUESTION: &str = "question";
pub const YAML_REQUIRED_DECISION: &str = "requiredDecision";
pub const YAML_REQUIRED_INPUT: &str = "requiredInput";
pub const YAML_TEXT: &str = "text";
pub const YAML_TYPE_LANGUAGE: &str = "typeLanguage";
pub const YAML_TYPE_REF: &str = "typeRef";
pub const YAML_VARIABLE: &str = "variable";

/// Returns the value of the required attribute.
pub fn required_attribute(name: &str, yaml: &Yaml, attr_name: &str) -> Result<String> {
  let value = &yaml[attr_name];
  if value.is_badvalue() || value.is_null() || value.is_array() {
    return Err(err_yaml_expected_mandatory_attribute(name, attr_name));
  }
  let Some(string_value) = value.as_str() else {
    return Err(err_yaml_expected_mandatory_attribute(name, attr_name));
  };
  Ok(string_value.to_string())
}

/// Returns the value of the optional attribute.
pub fn optional_attribute(yaml: &Yaml, attr_name: &str) -> Option<String> {
  let value = &yaml[attr_name];
  if value.is_null() || value.is_badvalue() || value.is_array() {
    return None;
  }
  value.as_str().map(|s| s.trim().to_string())
}

/// Returns the required URI attribute.
pub fn required_uri(name: &str, yaml: &Yaml, attr_name: &str) -> Result<Uri> {
  to_uri(required_attribute(name, yaml, attr_name)?.as_str())
}

/// Returns an optional URI attribute.
pub fn optional_uri(node: &Yaml, attr_name: &str) -> Result<Option<Uri>> {
  if let Some(value) = optional_attribute(node, attr_name) {
    Ok(Some(to_uri(value.as_str())?))
  } else {
    Ok(None)
  }
}

/// Returns required name attribute.
pub fn required_name(name: &str, yaml: &Yaml) -> Result<String> {
  required_attribute(name, yaml, YAML_NAME)
}

/// Returns optional identifier if provided in the model, otherwise generates a new one.
pub fn optional_id(yaml: &Yaml) -> DmnId {
  optional_attribute(yaml, YAML_ID).map(DmnId::Provided).unwrap_or(DmnId::Generated(gen_id()))
}

/// Returns the required FEEL name.
pub fn required_feel_name(name: &str, node: &Yaml) -> Result<Name> {
  let input = required_name(name, node)?;
  Ok(dsntk_feel_parser::parse_longest_name(&input).unwrap_or(input.into()))
}

/// Returns optional description.
pub fn optional_description(yaml: &Yaml) -> Option<String> {
  optional_attribute(yaml, YAML_DESCRIPTION).map(trim_multiline)
}

/// Returns optional label.
pub fn optional_label(yaml: &Yaml) -> Option<String> {
  optional_attribute(yaml, YAML_LABEL).map(|value| value.trim().to_string())
}

/// Returns the required `href` attribute of the specified optional attribute.
pub fn optional_attribute_required_href(yaml: &Yaml, attr_name: &str) -> Result<Option<HRef>> {
  if let Some(child_yaml) = scalar(yaml, attr_name) {
    Ok(Some(HRef::try_from(required_attribute(attr_name, child_yaml, YAML_HREF)?.as_str())?))
  } else {
    Ok(None)
  }
}

/// Returns a scalar attribute with specified name.
pub fn scalar<'a>(yaml: &'a Yaml, attr_name: &'a str) -> Option<&'a Yaml> {
  let value = &yaml[attr_name];
  if value.is_badvalue() || value.is_null() || value.is_array() {
    None
  } else {
    Some(value)
  }
}
