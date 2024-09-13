//! # Model validator against XML Schema Definitions

use crate::errors::*;
use crate::xml_utils::*;
use dsntk_common::Result;
use roxmltree::{Document, Node};

/// Validates a document containing DMN model against XML Schema Definitions.
pub fn validate_schema<'a>(document: &'a Document) -> Result<Node<'a, 'a>> {
  let root_element = document.root_element();
  validate_root_element(&root_element)?;
  Ok(root_element)
}

/// Validates if the root element's name is [NODE_DEFINITIONS].
fn validate_root_element(node: &Node) -> Result<()> {
  if node.tag_name().name() != NODE_DEFINITIONS {
    return Err(err_xml_unexpected_node(NODE_DEFINITIONS, node.tag_name().name()));
  }
  Ok(())
}
