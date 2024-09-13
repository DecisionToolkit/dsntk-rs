//! # Model validator against XML Schema Definitions

use crate::errors::*;
use crate::xml_utils::*;
use crate::DmnVersion;
use dsntk_common::Result;
use roxmltree::{Document, Node, NodeType};

const V_DEFINITIONS: ([&str; 2], [&str; 8], [&str; 16]) = (
  [ATTR_NAME, ATTR_NAMESPACE],
  [
    ATTR_EXPORTER,
    ATTR_EXPORTER_VERSION,
    ATTR_EXPRESSION_LANGUAGE,
    ATTR_ID,
    ATTR_LABEL,
    ATTR_NAME,
    ATTR_NAMESPACE,
    ATTR_TYPE_LANGUAGE,
  ],
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

const V_INPUT_DATA: ([&str; 1], [&str; 3], [&str; 3]) = ([ATTR_NAME], [ATTR_ID, ATTR_LABEL, ATTR_NAME], [NODE_DESCRIPTION, NODE_EXTENSION_ELEMENTS, NODE_VARIABLE]);

/// Validates a document containing DMN model against XML Schema Definitions.
pub fn validate_schema<'a>(document: &'a Document) -> Result<Node<'a, 'a>> {
  SchemaValidator::default().validate(document)
}

#[derive(Default)]
struct Namespaces {
  dmn: Option<String>,
}

impl Namespaces {
  /// Adds a namespace.
  fn add(&mut self, uri: &str) -> Result<()> {
    match uri {
      NS_MODEL_15 | NS_MODEL_14 | NS_MODEL_13 => {
        if let Some(dmn_uri) = &self.dmn {
          if dmn_uri == uri {
            return Err(err_duplicated_namespace(uri));
          }
        }
        self.dmn = Some(uri.to_string());
      }
      _ => {}
    }
    Ok(())
  }

  /// Returns the DMN version based on namespace URI defined in the standard.
  fn dmn_version(&self) -> Result<DmnVersion> {
    let Some(dmn_uri) = &self.dmn else {
      return Err(err_no_supported_namespace());
    };
    match dmn_uri.as_str() {
      NS_MODEL_15 => Ok(DmnVersion::V15),
      NS_MODEL_14 => Ok(DmnVersion::V14),
      NS_MODEL_13 => Ok(DmnVersion::V13),
      _ => Err(err_no_supported_namespace()),
    }
  }

  fn is_dmn(&self, uri: &str) -> bool {
    if let Some(dmn_uri) = &self.dmn {
      uri == dmn_uri
    } else {
      false
    }
  }
}

/// XSD Schema validator.
struct SchemaValidator {
  namespaces: Namespaces,
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
    Self {
      namespaces: Namespaces::default(),
      dmn_version: DmnVersion::V13,
    }
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
    // check the presence of namespaces
    for namespace in node.namespaces() {
      self.namespaces.add(namespace.uri())?;
    }
    // check the DMN version
    self.dmn_version = self.namespaces.dmn_version()?;
    // check if required attributes are present
    self.required_attributes(node, &V_DEFINITIONS.0)?;
    // reject not allowed attributes
    self.allowed_attributes(node, &V_DEFINITIONS.1)?;
    // reject not allowed child nodes
    self.allowed_children(node, &V_DEFINITIONS.2)?;
    Ok(())
  }

  /// Validates the input data nodes defined in the root node.
  fn validate_input_data(&mut self, node: &Node) -> Result<()> {
    for ref child_node in node.children().filter(|n| is(n, NODE_INPUT_DATA)) {
      // check if required attributes are present
      self.required_attributes(child_node, &V_INPUT_DATA.0)?;
      // reject not allowed attributes
      self.allowed_attributes(child_node, &V_INPUT_DATA.1)?;
      // reject not allowed child nodes
      self.allowed_children(child_node, &V_INPUT_DATA.2)?;
    }
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
      let attribute_name = attribute.name();
      if attribute.namespace().is_none() {
        if !allowed.contains(&attribute_name) {
          return Err(err_not_allowed_attribute("", attribute_name, node));
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
        let child_node_namespace = child_node.tag_name().namespace().unwrap_or("");
        println!("DDDC: '{}:{}'", child_node_name, child_node_namespace);
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
