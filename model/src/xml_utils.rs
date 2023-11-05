//! # XML utilities

use crate::errors::*;
use dsntk_common::Result;
use roxmltree::Node;
use std::str::FromStr;

/// Returns the value of the required attribute.
pub fn required_attribute(node: &Node, attr_name: &str) -> Result<String> {
  if let Some(attr_value) = node.attribute(attr_name) {
    Ok(attr_value.to_owned())
  } else {
    Err(err_xml_expected_mandatory_attribute(&node_name_pos(node), attr_name))
  }
}

/// Returns the value of the mandatory color attribute.
pub fn required_color_part(node: &Node, attr_name: &str) -> Result<u8> {
  u8::from_str(&required_attribute(node, attr_name)?).map_err(|e| err_invalid_color_value(&e.to_string()))
}

/// Returns the value of the mandatory double value.
pub fn required_double(node: &Node, attr_name: &str) -> Result<f64> {
  f64::from_str(&required_attribute(node, attr_name)?).map_err(|e| err_invalid_double_value(&e.to_string()))
}

/// Returns the value of the optional attribute.
pub fn optional_attribute(node: &Node, attr_name: &str) -> Option<String> {
  node.attribute(attr_name).map(|attr_value| attr_value.to_owned())
}

/// Returns the value of the optional double attribute or the default value, when this attribute is not defined.
pub fn optional_double(node: &Node, attr_name: &str) -> Option<f64> {
  optional_attribute(node, attr_name).and_then(|value| f64::from_str(&value).ok())
}

/// Returns the value of the optional bool attribute.
pub fn optional_bool(node: &Node, attr_name: &str) -> Option<bool> {
  optional_attribute(node, attr_name).and_then(|value| bool::from_str(&value).ok())
}

/// Returns required textual content of the node.
pub fn required_content(node: &Node) -> Result<String> {
  if let Some(text) = node.text() {
    Ok(text.to_owned())
  } else {
    Err(err_xml_expected_mandatory_text_content(node.tag_name().name()))
  }
}

/// Returns optional textual content of the node.
pub fn optional_content(node: &Node) -> Option<String> {
  node.text().map(|text| text.to_owned())
}

/// Returns required child node or raises an error when there is no child with given name.
pub fn required_child<'a>(node: &'a Node, child_name: &str) -> Result<Node<'a, 'a>> {
  if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
    Ok(child_node)
  } else {
    Err(err_required_child_node_is_missing(&node_name_pos(node), child_name))
  }
}

/// Returns child node when there is a child with the given name.
pub fn optional_child<'a>(node: &'a Node, child_name: &str) -> Option<Node<'a, 'a>> {
  node.children().find(|n| n.tag_name().name() == child_name)
}

/// Returns the required text content of the required child node.
pub fn required_child_required_content(node: &Node, child_name: &str) -> Result<String> {
  if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
    required_content(&child_node)
  } else {
    Err(err_xml_expected_mandatory_child_node(&node_name_pos(node), child_name))
  }
}

/// Returns the required content of the optional child node.
pub fn optional_child_required_content(node: &Node, child_name: &str) -> Result<Option<String>> {
  if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
    Ok(Some(required_content(&child_node)?))
  } else {
    Ok(None)
  }
}

/// Returns the optional content of the optional child node.
pub fn optional_child_optional_content(node: &Node, child_name: &str) -> Option<String> {
  if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
    optional_content(&child_node)
  } else {
    None
  }
}

/// Utility function that returns the node's name with its position in the original document.
pub fn node_name_pos(node: &Node) -> String {
  format!("'{}' at [{}]", node.tag_name().name(), node.document().text_pos_at(node.range().start))
}
