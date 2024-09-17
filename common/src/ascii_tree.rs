//! # ASCII tree with colors

use crate::{ColorMode, ASCII_CLEAR};
use std::fmt;
use std::fmt::{Display, Formatter, Write};

/// Text with associated color control sequence to be displayed in terminal.
pub struct AsciiText {
  color: Option<String>,
  text: String,
}

impl AsciiText {
  /// Creates a new text without color control sequence.
  pub fn new(text: &str) -> Self {
    Self {
      color: None,
      text: text.to_string(),
    }
  }

  /// Creates a new text with associated color control sequence.
  pub fn with_color(text: &str, color: &str) -> Self {
    Self {
      color: Some(color.to_string()),
      text: text.to_string(),
    }
  }

  /// Returns a text with color control sequence based on provided [ColorMode].
  pub fn mode(&self, color_mode: &ColorMode) -> Self {
    AsciiText {
      color: match color_mode {
        ColorMode::On => self.color.clone(),
        _ => None,
      },
      text: self.text.clone(),
    }
  }
}

impl Display for AsciiText {
  /// Prints a text with associated color control sequence.
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    if let Some(color) = &self.color {
      write!(f, "{}{}{}", color, self.text, ASCII_CLEAR)
    } else {
      write!(f, "{}", self.text)
    }
  }
}

/// Collection of [AsciiText] segments with associated color control sequence.
pub struct AsciiLine(Vec<AsciiText>);

impl AsciiLine {
  pub fn builder() -> AsciiLineBuilder {
    AsciiLineBuilder(vec![])
  }

  pub fn mode(&self, color_mode: &ColorMode) -> Self {
    AsciiLine(self.0.iter().map(|ascii_text| ascii_text.mode(color_mode)).collect())
  }
}

impl Display for AsciiLine {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    for text in &self.0 {
      write!(f, "{}", text)?
    }
    Ok(())
  }
}

/// Builder for [AsciiLine].
pub struct AsciiLineBuilder(Vec<AsciiText>);

impl AsciiLineBuilder {
  pub fn text(mut self, text: &str) -> Self {
    self.0.push(AsciiText::new(text));
    self
  }

  pub fn with_color(mut self, text: &str, color: &str) -> Self {
    self.0.push(AsciiText::with_color(text, color));
    self
  }

  pub fn indent(mut self) -> Self {
    self.0.push(AsciiText::new("  "));
    self
  }

  pub fn colon(mut self) -> Self {
    self.0.push(AsciiText::new(":"));
    self
  }

  pub fn colon_space(mut self) -> Self {
    self.0.push(AsciiText::new(": "));
    self
  }

  pub fn build(self) -> AsciiLine {
    AsciiLine(self.0)
  }
}

/// Types of nodes in the coloured ASCII tree.
pub enum AsciiNode {
  /// Intermediary (or root) node in the tree (always has child nodes).
  Node(AsciiLine, Vec<AsciiNode>),
  /// Node being the leaf in the tree (has no child nodes).
  Leaf(Vec<AsciiLine>),
}

impl AsciiNode {
  pub fn leaf_builder() -> AsciiLeafBuilder {
    AsciiLeafBuilder(vec![])
  }

  pub fn node_builder(line: AsciiLine) -> AsciiNodeBuilder {
    AsciiNodeBuilder(line, vec![])
  }
}

/// Builder for [AsciiNode::Leaf].
pub struct AsciiLeafBuilder(Vec<AsciiLine>);

impl AsciiLeafBuilder {
  pub fn line(mut self, line: AsciiLine) -> Self {
    self.0.push(line);
    self
  }

  pub fn add_line(&mut self, line: AsciiLine) {
    self.0.push(line);
  }

  pub fn build(self) -> AsciiNode {
    AsciiNode::Leaf(self.0)
  }
}

/// Builder for [AsciiNode::Node].
pub struct AsciiNodeBuilder(AsciiLine, Vec<AsciiNode>);

impl AsciiNodeBuilder {
  pub fn child(mut self, child: AsciiNode) -> Self {
    self.1.push(child);
    self
  }

  pub fn opt_child(mut self, opt_child: Option<AsciiNode>) -> Self {
    if let Some(child) = opt_child {
      self.1.push(child);
    }
    self
  }

  pub fn add_child(&mut self, child: AsciiNode) {
    self.1.push(child);
  }

  pub fn build(self) -> AsciiNode {
    AsciiNode::Node(self.0, self.1)
  }
}

/// Writes the tree to provided writer starting from specified node.
pub fn write(f: &mut dyn Write, node: &AsciiNode, color_mode: &ColorMode) -> fmt::Result {
  write_node(f, node, &[], color_mode)
}

/// Writes the tree to provided writer starting from specified node with indentation.
pub fn write_indented(f: &mut dyn Write, node: &AsciiNode, color_mode: &ColorMode, indent: usize) -> fmt::Result {
  let mut buffer = String::new();
  let _ = write_node(&mut buffer, node, &[], color_mode);
  let indent = " ".repeat(indent);
  let mut tree = String::new();
  for line in buffer.lines() {
    let _ = writeln!(&mut tree, "{}{}", indent, line);
  }
  write!(f, "{}", tree)
}

/// Writes the tree node to provided writer.
fn write_node(f: &mut dyn Write, tree: &AsciiNode, level: &[usize], color_mode: &ColorMode) -> fmt::Result {
  const NONE: &str = "   ";
  const EDGE: &str = " └─";
  const PIPE: &str = " │ ";
  const FORK: &str = " ├─";

  let max_pos = level.len();
  let mut second_line = String::new();
  for (pos, lev) in level.iter().enumerate() {
    let last_row = pos == max_pos - 1;
    if *lev == 1 {
      if !last_row {
        write!(f, "{}", NONE)?
      } else {
        write!(f, "{}", EDGE)?
      }
      second_line.push_str(NONE);
    } else {
      if !last_row {
        write!(f, "{}", PIPE)?
      } else {
        write!(f, "{}", FORK)?
      }
      second_line.push_str(PIPE);
    }
  }
  match tree {
    AsciiNode::Node(title, children) => {
      let mut deep = children.len();
      writeln!(f, " {}", title.mode(color_mode))?;
      for node in children {
        let mut level_next = level.to_vec();
        level_next.push(deep);
        deep -= 1;
        write_node(f, node, &level_next, color_mode)?;
      }
    }
    AsciiNode::Leaf(lines) => {
      for (i, line) in lines.iter().enumerate() {
        match i {
          0 => writeln!(f, " {}", line.mode(color_mode))?,
          _ => writeln!(f, "{} {}", second_line, line.mode(color_mode))?,
        }
      }
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXPECTED: &str = r#"
 node 4
 ├─ node 1
 │  ├─ line 1_1
 │  │  line 1_2
 │  │  line 1_3
 │  │  line 1_4
 │  └─ only one line
 ├─ node 2
 │  ├─ only one line
 │  ├─ line 2_1
 │  │  line 2_2
 │  │  line 2_3
 │  │  line 2_4
 │  └─ only one line
 └─ node 3
    ├─ node 1
    │  ├─ line 3_1_1
    │  │  line 3_1_2
    │  │  line 3_1_3
    │  │  line 3_1_4
    │  └─ only one line
    ├─ line 3_1
    │  line 3_2
    │  line 3_3
    │  line 3_4
    └─ only one line
"#;

  #[test]
  fn test_ascii_tree() {
    let root = AsciiNode::node_builder(AsciiLine::builder().text("node 4").build())
      .child(
        AsciiNode::node_builder(AsciiLine::builder().text("node 1").build())
          .child(
            AsciiNode::leaf_builder()
              .line(AsciiLine::builder().text("line 1_1").build())
              .line(AsciiLine::builder().text("line 1_2").build())
              .line(AsciiLine::builder().text("line 1_3").build())
              .line(AsciiLine::builder().text("line 1_4").build())
              .build(),
          )
          .child(AsciiNode::leaf_builder().line(AsciiLine::builder().text("only one line").build()).build())
          .build(),
      )
      .child(
        AsciiNode::node_builder(AsciiLine::builder().text("node 2").build())
          .child(AsciiNode::leaf_builder().line(AsciiLine::builder().text("only one line").build()).build())
          .child(
            AsciiNode::leaf_builder()
              .line(AsciiLine::builder().text("line 2_1").build())
              .line(AsciiLine::builder().text("line 2_2").build())
              .line(AsciiLine::builder().text("line 2_3").build())
              .line(AsciiLine::builder().text("line 2_4").build())
              .build(),
          )
          .child(AsciiNode::leaf_builder().line(AsciiLine::builder().text("only one line").build()).build())
          .build(),
      )
      .child(
        AsciiNode::node_builder(AsciiLine::builder().text("node 3").build())
          .child(
            AsciiNode::node_builder(AsciiLine::builder().text("node 1").build())
              .child(
                AsciiNode::leaf_builder()
                  .line(AsciiLine::builder().text("line 3_1_1").build())
                  .line(AsciiLine::builder().text("line 3_1_2").build())
                  .line(AsciiLine::builder().text("line 3_1_3").build())
                  .line(AsciiLine::builder().text("line 3_1_4").build())
                  .build(),
              )
              .child(AsciiNode::leaf_builder().line(AsciiLine::builder().text("only one line").build()).build())
              .build(),
          )
          .child(
            AsciiNode::leaf_builder()
              .line(AsciiLine::builder().text("line 3_1").build())
              .line(AsciiLine::builder().text("line 3_2").build())
              .line(AsciiLine::builder().text("line 3_3").build())
              .line(AsciiLine::builder().text("line 3_4").build())
              .build(),
          )
          .child(AsciiNode::leaf_builder().line(AsciiLine::builder().text("only one line").build()).build())
          .build(),
      )
      .build();

    let mut output = String::new();
    let _ = writeln!(&mut output);
    let _ = write(&mut output, &root, &ColorMode::Off);
    assert_eq!(EXPECTED, output);
  }
}
