//! # Model validator against XML Schema Definitions

use crate::errors::*;
use crate::xml_utils::*;
use crate::DmnVersion;
use dsntk_common::Result;
use roxmltree::{Document, Node};

/// Validates a document containing DMN model against XML Schema Definitions.
pub fn validate_schema<'a>(document: &'a Document) -> Result<Node<'a, 'a>> {
  SchemaValidator::default().validate(document)
}

/// XSD Schema validator.
pub struct SchemaValidator {
  dmn_version: DmnVersion,
}

impl Default for SchemaValidator {
  fn default() -> Self {
    Self::new()
  }
}

impl SchemaValidator {
  /// Creates a new schema validator.
  fn new() -> Self {
    Self { dmn_version: DmnVersion::V13 }
  }

  /// Validates provided document against XML Schema.
  fn validate<'a>(&mut self, document: &'a Document) -> Result<Node<'a, 'a>> {
    let root_element = document.root_element();
    self.validate_root_element(&root_element)?;
    Ok(root_element)
  }

  /// Validates the root element.
  fn validate_root_element(&mut self, node: &Node) -> Result<()> {
    // check the name of the root element
    if node.tag_name().name() != NODE_DEFINITIONS {
      return Err(err_xml_unexpected_node(NODE_DEFINITIONS, node.tag_name().name()));
    }
    // check the presence of the required `xmlns` attribute (default namespace) or prefixed namespace
    let dmn_namespace = if let Some(namespace) = node.default_namespace() {
      namespace.to_string()
    } else if let Some(namespace) = node.namespaces().next() {
      namespace.uri().to_string()
    } else {
      return Err(err_no_default_namespace());
    };
    // check supported DMN schema versions
    self.dmn_version = match dmn_namespace.as_str() {
      NS_MODEL_13 => DmnVersion::V13,
      NS_MODEL_14 => DmnVersion::V14,
      NS_MODEL_15 => DmnVersion::V15,
      other => return Err(err_unsupported_schema(other)),
    };
    // root element must have required attribute `namespace`
    required_attribute(node, ATTR_NAMESPACE)?;
    // root element must have required attribute `name`
    required_attribute(node, ATTR_NAME)?;
    // reject not allowed attributes
    allowed_attributes(
      node,
      &[ATTR_NAME, ATTR_NAMESPACE, ATTR_ID, ATTR_TYPE_LANGUAGE, ATTR_LABEL, ATTR_EXPORTER, ATTR_EXPORTER_VERSION],
    )?;
    Ok(())
  }
}

fn allowed_attributes(node: &Node, allowed: &[&str]) -> Result<()> {
  for attribute in node.attributes() {
    if attribute.namespace().is_none() {
      let attribute_name = attribute.name();
      if !allowed.contains(&attribute_name) {
        return Err(err_not_allowed_attribute(attribute_name, node));
      }
    }
  }
  Ok(())
}
