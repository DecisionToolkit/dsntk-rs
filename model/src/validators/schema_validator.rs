//! # Model validator against XML Schema Definitions

use crate::errors::*;
use crate::xml_utils::*;
use crate::DmnVersion;
use dsntk_common::Result;
use roxmltree::{Document, Node, NodeType};

/// Validates a document containing DMN model against XML Schema Definitions.
pub fn validate_schema<'a>(document: &'a Document) -> Result<Node<'a, 'a>> {
  SchemaValidator::default().validate(document)
}

#[derive(Default)]
struct Namespaces {
  dmn: Option<String>,
  dmndi: Option<String>,
  dc: Option<String>,
  di: Option<String>,
}

impl Namespaces {
  /// Adds a namespace.
  fn add(&mut self, uri: &str) -> Result<()> {
    match uri {
      NAMESPACE_DMN_13 | NAMESPACE_DMN_14 | NAMESPACE_DMN_15 => {
        self.dmn = Some(uri.to_string());
      }
      NAMESPACE_DMNDI_13 | NAMESPACE_DMNDI_15 => {
        self.dmndi = Some(uri.to_string());
      }
      NAMESPACE_DC_13 => {
        self.dc = Some(uri.to_string());
      }
      NAMESPACE_DI_13 => {
        self.di = Some(uri.to_string());
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
      NAMESPACE_DMN_15 => Ok(DmnVersion::V15),
      NAMESPACE_DMN_14 => Ok(DmnVersion::V14),
      NAMESPACE_DMN_13 => Ok(DmnVersion::V13),
      _ => Err(err_no_supported_namespace()),
    }
  }

  fn is_valid(&self, uri: &str) -> bool {
    self.dmn.as_ref().map(|v| v == uri).unwrap_or(false)
      || self.dmndi.as_ref().map(|v| v == uri).unwrap_or(false)
      || self.dc.as_ref().map(|v| v == uri).unwrap_or(false)
      || self.di.as_ref().map(|v| v == uri).unwrap_or(false)
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
    self.validate_definitions_node(&root_element)?;
    self.validate_input_data_nodes(&root_element)?;
    //self.validate_decision_nodes(&root_element)?;
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
    Ok(())
  }

  /// Validates the `definitions` node.
  fn validate_definitions_node(&mut self, node: &Node) -> Result<()> {
    match self.dmn_version {
      _ => self.standard_checks(node, &v13::V_DEFINITIONS.0, &v13::V_DEFINITIONS.1, &v13::V_DEFINITIONS.2, &v13::V_DEFINITIONS.3)?,
    }
    Ok(())
  }

  /// Validates the `inputData` nodes.
  fn validate_input_data_nodes(&mut self, node: &Node) -> Result<()> {
    for ref child_node in node.children().filter(|n| is(n, NODE_INPUT_DATA)) {
      match self.dmn_version {
        _ => self.standard_checks(child_node, &v13::V_INPUT_DATA.0, &v13::V_INPUT_DATA.1, &v13::V_INPUT_DATA.2, &v13::V_INPUT_DATA.3)?,
      }
    }
    Ok(())
  }

  /// Validates the `decision` nodes.
  fn validate_decision_nodes(&mut self, node: &Node) -> Result<()> {
    for ref child_node in node.children().filter(|n| is(n, NODE_DECISION)) {
      match self.dmn_version {
        DmnVersion::V13 => self.standard_checks(child_node, &v13::V_DECISION.0, &v13::V_DECISION.1, &v13::V_DECISION.2, &v13::V_DECISION.3)?,
        _ => self.standard_checks(child_node, &v14::V_DECISION.0, &v14::V_DECISION.1, &v14::V_DECISION.2, &v14::V_DECISION.3)?,
      }
    }
    Ok(())
  }

  fn standard_checks(&self, node: &Node, ra: &[&str], aa: &[&str], rc: &[&str], ac: &[&str]) -> Result<()> {
    // check if required attributes are present
    self.required_attributes(node, ra)?;
    // reject not allowed attributes
    self.allowed_attributes(node, aa)?;
    // check if required child nodes are present
    self.required_children(node, rc)?;
    // reject not allowed child nodes
    self.allowed_children(node, ac)?;
    Ok(())
  }

  /// Verifies if required attributes are present in specified node.
  fn required_attributes(&self, node: &Node, required: &[&str]) -> Result<()> {
    for required_name in required {
      required_attribute(node, required_name)?;
    }
    Ok(())
  }

  /// Verifies if only allowed attributes are defined in the specified node.
  fn allowed_attributes(&self, node: &Node, allowed: &[&str]) -> Result<()> {
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

  /// Verifies if required child nodes are present in the specified node.
  fn required_children(&self, _node: &Node, _required: &[&str]) -> Result<()> {
    Ok(())
  }

  /// Verifies if only allowed child nodes are present in the specified node.
  fn allowed_children(&self, node: &Node, allowed: &[&str]) -> Result<()> {
    for child_node in node.children() {
      if child_node.node_type() == NodeType::Element {
        let child_node_name = child_node.tag_name().name();
        if let Some(child_node_namespace) = child_node.tag_name().namespace() {
          if !self.namespaces.is_valid(child_node_namespace) {
            return Err(err_not_allowed_child_node(child_node_name, node));
          }
        }
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

mod v13 {
  use crate::xml_utils::*;

  pub const V_DEFINITIONS: ([&str; 2], [&str; 8], [&str; 0], [&str; 16]) = (
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
    [],
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

  pub const V_INPUT_DATA: ([&str; 1], [&str; 4], [&str; 0], [&str; 3]) = (
    [ATTR_NAME],
    [ATTR_ID, ATTR_LABEL, ATTR_NAME, NODE_VARIABLE],
    [],
    [NODE_DESCRIPTION, NODE_EXTENSION_ELEMENTS, NODE_VARIABLE],
  );

  pub const V_DECISION: ([&str; 1], [&str; 3], [&str; 0], [&str; 21]) = (
    [ATTR_NAME],
    [ATTR_ID, NODE_AUTHORITY_REQUIREMENT, ATTR_NAME],
    [],
    [
      NODE_ALLOWED_ANSWERS,
      NODE_AUTHORITY_REQUIREMENT,
      NODE_CONTEXT,
      NODE_DECISION_MAKER,
      NODE_DECISION_OWNER,
      NODE_DECISION_TABLE,
      NODE_DESCRIPTION,
      NODE_EXTENSION_ELEMENTS,
      NODE_FUNCTION_DEFINITION,
      NODE_IMPACTED_PERFORMANCE_INDICATOR,
      NODE_INFORMATION_REQUIREMENT,
      NODE_INVOCATION,
      NODE_KNOWLEDGE_REQUIREMENT,
      NODE_LIST,
      NODE_LITERAL_EXPRESSION,
      NODE_QUESTION,
      NODE_RELATION,
      NODE_SUPPORTED_OBJECTIVE,
      NODE_USING_PROCESS,
      NODE_USING_TASK,
      NODE_VARIABLE,
    ],
  );
}

mod v14 {
  use crate::xml_utils::*;

  pub const V_DECISION: ([&str; 1], [&str; 3], [&str; 0], [&str; 26]) = (
    [ATTR_NAME],
    [ATTR_ID, NODE_AUTHORITY_REQUIREMENT, ATTR_NAME],
    [],
    [
      NODE_ALLOWED_ANSWERS,
      NODE_AUTHORITY_REQUIREMENT,
      NODE_CONDITIONAL,
      NODE_CONTEXT,
      NODE_DECISION_MAKER,
      NODE_DECISION_OWNER,
      NODE_DECISION_TABLE,
      NODE_DESCRIPTION,
      NODE_EVERY,
      NODE_EXTENSION_ELEMENTS,
      NODE_FILTER,
      NODE_FOR,
      NODE_FUNCTION_DEFINITION,
      NODE_IMPACTED_PERFORMANCE_INDICATOR,
      NODE_INFORMATION_REQUIREMENT,
      NODE_INVOCATION,
      NODE_KNOWLEDGE_REQUIREMENT,
      NODE_LIST,
      NODE_LITERAL_EXPRESSION,
      NODE_QUESTION,
      NODE_RELATION,
      NODE_SOME,
      NODE_SUPPORTED_OBJECTIVE,
      NODE_USING_PROCESS,
      NODE_USING_TASK,
      NODE_VARIABLE,
    ],
  );
}
