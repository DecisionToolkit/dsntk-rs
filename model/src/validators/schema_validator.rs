//! # Model validator against XML Schema Definitions

use crate::errors::*;
use crate::xml_utils::*;
use crate::DmnVersion;
use dsntk_common::Result;
use roxmltree::{Document, Node, NodeType};

const DEFINITIONS: ([&str; 2], [&str; 7], [&str; 16]) = (
  [ATTR_NAME, ATTR_NAMESPACE],
  [ATTR_EXPORTER, ATTR_EXPORTER_VERSION, ATTR_ID, ATTR_LABEL, ATTR_NAME, ATTR_NAMESPACE, ATTR_TYPE_LANGUAGE],
  [
    NODE_ASSOCIATION,
    NODE_BUSINESS_KNOWLEDGE_MODEL,
    NODE_DECISION,
    NODE_DECISION_SERVICE,
    NODE_DESCRIPTION,
    NODE_DMNDI,
    NODE_ELEMENT_COLLECTION,
    NODE_EXTENSION_ELEMENTS,
    NODE_GROUP,
    NODE_IMPORT,
    NODE_INPUT_DATA,
    NODE_ITEM_DEFINITION,
    NODE_KNOWLEDGE_SOURCE,
    NODE_ORGANISATION_UNIT,
    NODE_PERFORMANCE_INDICATOR,
    NODE_TEXT_ANNOTATION,
  ],
);

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
    self.validate_input_data(&root_element)?;
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
    // // root element must have required attribute `namespace`
    // required_attribute(node, ATTR_NAMESPACE)?;
    // // root element must have required attribute `name`
    // required_attribute(node, ATTR_NAME)?;
    // check if required attributes are present
    self.required_attributes(node, &DEFINITIONS.0)?;
    // reject not allowed attributes
    self.allowed_attributes(node, &DEFINITIONS.1)?;
    // reject not allowed child nodes
    self.allowed_children(node, &DEFINITIONS.2)?;
    Ok(())
  }

  /// Validates the input data nodes defined in the root node.
  fn validate_input_data(&mut self, node: &Node) -> Result<()> {
    for child in node.children().filter(|n| is(n, NODE_INPUT_DATA)) {}
    Ok(())
  }

  fn required_attributes(&mut self, node: &Node, required: &[&str]) -> Result<()> {
    for required_name in required {
      required_attribute(node, required_name)?;
    }
    Ok(())
  }

  /// Verifies if only allowed attributes are defined in the specified node.
  fn allowed_attributes(&mut self, node: &Node, allowed: &[&str]) -> Result<()> {
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

  /// Verifies if only allowed child nodes are present in the specified node.
  fn allowed_children(&mut self, node: &Node, allowed: &[&str]) -> Result<()> {
    for child_node in node.children() {
      if child_node.node_type() == NodeType::Element {
        let child_node_name = child_node.tag_name().name();
        if !allowed.contains(&child_node_name) {
          return Err(err_not_allowed_child_node(child_node_name, node));
        }
      }
    }
    Ok(())
  }
}

fn is(node: &Node, name: &str) -> bool {
  node.tag_name().name() == name
}
