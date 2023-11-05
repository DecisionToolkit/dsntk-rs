//! # ASCII report of the DMN model

use dsntk_common::{color_256, write_indented, AsciiLine, AsciiNode, AsciiNodeBuilder, ColorMode, HRef};

use dsntk_feel::Name;
use dsntk_model::*;
use std::fmt::Write;

const LABEL_ALLOWED_ANSWERS: &str = "allowed answers";
const LABEL_BUSINESS_KNOWLEDGE_MODELS: &str = "Business knowledge models";
const LABEL_DECISION_SERVICES: &str = "Decision services";
const LABEL_DECISIONS: &str = "Decisions";
const LABEL_DECISIONS_MADE: &str = "decisions made";
const LABEL_DECISIONS_OWNED: &str = "decisions owned";
const LABEL_DESCRIPTION: &str = "description";
const LABEL_EXPRESSION_LANGUAGE: &str = "expression language";
const LABEL_ID: &str = "id";
const LABEL_INPUT_DATA: &str = "Input data";
const LABEL_KNOWLEDGE_SOURCES: &str = "Knowledge sources";
const LABEL_LABEL: &str = "label";
const LABEL_IMPACTING_DECISIONS: &str = "impacting decisions";
const LABEL_MODEL: &str = "Model";
const LABEL_NAME: &str = "name";
const LABEL_FEEL_NAME: &str = "FEEL name";
const LABEL_NAMESPACE: &str = "namespace";
const LABEL_ORGANISATION_UNITS: &str = "Organisation units";
const LABEL_PERFORMANCE_INDICATORS: &str = "Performance indicators";
const LABEL_QUESTION: &str = "question";
const LABEL_TYPE: &str = "type";
const LABEL_TYPE_LANGUAGE: &str = "type language";
const LABEL_URI: &str = "URI";
const LABEL_VARIABLE: &str = "variable (output)";

/// Color palette.
struct Colors {
  color_mode: ColorMode,
  color_default: String,
  color_name: String,
  color_label: String,
  color_id: String,
  color_uri: String,
  color_description: String,
  color_type: String,
  color_href: String,
}

impl Colors {
  /// Creates a new color palette based on color mode.
  fn new(color_mode: ColorMode) -> Self {
    Self {
      color_mode,
      color_default: color_256!(color_mode, 255),
      color_name: color_256!(color_mode, 184),
      color_label: color_256!(color_mode, 209),
      color_id: color_256!(color_mode, 82),
      color_uri: color_256!(color_mode, 56),
      color_description: color_256!(color_mode, 255),
      color_type: color_256!(color_mode, 74),
      color_href: color_256!(color_mode, 61),
    }
  }
  fn mode(&self) -> &ColorMode {
    &self.color_mode
  }
  fn default(&self) -> &str {
    &self.color_default
  }
  fn name(&self) -> &str {
    &self.color_name
  }
  fn label(&self) -> &str {
    &self.color_label
  }
  fn id(&self) -> &str {
    &self.color_id
  }
  fn uri(&self) -> &str {
    &self.color_uri
  }
  fn description(&self) -> &str {
    &self.color_description
  }
  fn typ(&self) -> &str {
    &self.color_type
  }
  fn href(&self) -> &str {
    &self.color_href
  }
}

/// Prints the model report to standard output.
pub fn print_model(definitions: &Definitions, color_mode: ColorMode) {
  let colors = &Colors::new(color_mode);
  let indent = 2;
  let mut output = String::new();
  write_model(&mut output, definitions, colors, indent);
  write_decisions(&mut output, definitions, colors, indent);
  write_business_knowledge_models(&mut output, definitions, colors, indent);
  write_decision_services(&mut output, definitions, colors, indent);
  write_knowledge_sources(&mut output, definitions, colors, indent);
  write_input_data(&mut output, definitions, colors, indent);
  write_performance_indicators(&mut output, definitions, colors, indent);
  write_organisation_units(&mut output, definitions, colors, indent);
  println!("{}", output);
}

/// Writes a title text.
fn write_title(w: &mut dyn Write, title: &str) {
  let bar = "─".repeat(title.len());
  let _ = writeln!(w);
  let _ = writeln!(w, "┌─{}─┐", bar);
  let _ = writeln!(w, "│ {} │", title);
  let _ = writeln!(w, "└─{}─┘", bar);
}

/// Writes an empty line when counter > 0.
fn write_empty_line(w: &mut dyn Write, counter: usize) {
  if counter > 0 {
    let _ = writeln!(w);
  }
}

/// Write model properties.
fn write_model(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  write_title(w, LABEL_MODEL);
  let node_model = AsciiNode::node_builder(AsciiLine::builder().with_color(definitions.name(), colors.name()).build())
    .child(build_feel_name(definitions.feel_name(), colors))
    .child(build_namespace_leaf(definitions.namespace(), colors))
    .opt_child(build_label(definitions.label(), colors))
    .opt_child(build_id(definitions.opt_id(), colors))
    .opt_child(build_labeled_uri(LABEL_EXPRESSION_LANGUAGE, definitions.expression_language(), colors))
    .opt_child(build_labeled_uri(LABEL_TYPE_LANGUAGE, definitions.type_language(), colors))
    .opt_child(build_opt_labeled_text("exporter", definitions.exporter(), colors.default()))
    .opt_child(build_opt_labeled_text("exporter version", definitions.exporter_version(), colors.default()))
    .opt_child(build_description(definitions.description(), colors))
    .opt_child(build_extension_elements(definitions.extension_elements(), colors))
    .opt_child(build_extension_attributes(definitions.extension_attributes(), colors))
    .build();
  let _ = write_indented(w, &node_model, colors.mode(), indent);
}

/// Write decisions.
fn write_decisions(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let decisions = definitions.decisions();
  if !decisions.is_empty() {
    write_title(w, LABEL_DECISIONS);
  }
  for (i, decision) in decisions.iter().enumerate() {
    let node_decision = builder_name(decision.name(), colors)
      .child(build_feel_name(decision.feel_name(), colors))
      .opt_child(build_label(decision.label(), colors))
      .opt_child(build_id(decision.opt_id(), colors))
      .opt_child(build_opt_labeled_multiline_text(LABEL_QUESTION, decision.question(), colors.default()))
      .opt_child(build_opt_labeled_multiline_text(LABEL_ALLOWED_ANSWERS, decision.allowed_answers(), colors.default()))
      .opt_child(build_description(decision.description(), colors))
      .child(build_variable(decision.variable(), colors))
      .opt_child(build_authority_requirements(decision.authority_requirements(), colors))
      .opt_child(build_extension_elements(decision.extension_elements(), colors))
      .opt_child(build_extension_attributes(decision.extension_attributes(), colors))
      .build();
    write_empty_line(w, i);
    let _ = write_indented(w, &node_decision, colors.mode(), indent);
  }
}

/// Write business knowledge models.
fn write_business_knowledge_models(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let business_knowledge_models = definitions.business_knowledge_models();
  if !business_knowledge_models.is_empty() {
    write_title(w, LABEL_BUSINESS_KNOWLEDGE_MODELS);
  }
  for (i, bkm) in business_knowledge_models.iter().enumerate() {
    let node_decision = builder_name(bkm.name(), colors)
      .child(build_feel_name(bkm.feel_name(), colors))
      .opt_child(build_label(bkm.label(), colors))
      .opt_child(build_id(bkm.opt_id(), colors))
      .opt_child(build_description(bkm.description(), colors))
      .child(build_variable(bkm.variable(), colors))
      .opt_child(build_authority_requirements(bkm.authority_requirements(), colors))
      .opt_child(build_extension_elements(bkm.extension_elements(), colors))
      .opt_child(build_extension_attributes(bkm.extension_attributes(), colors))
      .build();
    write_empty_line(w, i);
    let _ = write_indented(w, &node_decision, colors.mode(), indent);
  }
}

/// Write decision services.
fn write_decision_services(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let decision_services = definitions.decision_services();
  if !decision_services.is_empty() {
    write_title(w, LABEL_DECISION_SERVICES);
  }
  for (i, decision_service) in decision_services.iter().enumerate() {
    let node_decision = builder_name(decision_service.name(), colors)
      .child(build_feel_name(decision_service.feel_name(), colors))
      .opt_child(build_label(decision_service.label(), colors))
      .opt_child(build_id(decision_service.opt_id(), colors))
      .opt_child(build_description(decision_service.description(), colors))
      .child(build_variable(decision_service.variable(), colors))
      .opt_child(build_extension_elements(decision_service.extension_elements(), colors))
      .opt_child(build_extension_attributes(decision_service.extension_attributes(), colors))
      .build();
    write_empty_line(w, i);
    let _ = write_indented(w, &node_decision, colors.mode(), indent);
  }
}

/// Write knowledge sources.
fn write_knowledge_sources(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let knowledge_sources = definitions.knowledge_sources();
  if !knowledge_sources.is_empty() {
    write_title(w, LABEL_KNOWLEDGE_SOURCES);
  }
  for (i, knowledge_source) in knowledge_sources.iter().enumerate() {
    let node_decision = builder_name(knowledge_source.name(), colors)
      .child(build_feel_name(knowledge_source.feel_name(), colors))
      .opt_child(build_label(knowledge_source.label(), colors))
      .opt_child(build_id(knowledge_source.opt_id(), colors))
      .opt_child(build_description(knowledge_source.description(), colors))
      .opt_child(build_extension_elements(knowledge_source.extension_elements(), colors))
      .opt_child(build_extension_attributes(knowledge_source.extension_attributes(), colors))
      .opt_child(build_authority_requirements(knowledge_source.authority_requirements(), colors))
      .build();
    write_empty_line(w, i);
    let _ = write_indented(w, &node_decision, colors.mode(), indent);
  }
}

/// Write input data.
fn write_input_data(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let input_data = definitions.input_data();
  if !input_data.is_empty() {
    write_title(w, LABEL_INPUT_DATA);
  }
  for (i, input) in input_data.iter().enumerate() {
    let node_input = builder_name(input.name(), colors)
      .child(build_feel_name(input.feel_name(), colors))
      .opt_child(build_label(input.label(), colors))
      .opt_child(build_id(input.opt_id(), colors))
      .opt_child(build_description(input.description(), colors))
      .child(build_variable(input.variable(), colors))
      .opt_child(build_extension_elements(input.extension_elements(), colors))
      .opt_child(build_extension_attributes(input.extension_attributes(), colors))
      .build();
    write_empty_line(w, i);
    let _ = write_indented(w, &node_input, colors.mode(), indent);
  }
}

/// Write performance indicators.
fn write_performance_indicators(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let performance_indicators = definitions.performance_indicators();
  if !performance_indicators.is_empty() {
    write_title(w, LABEL_PERFORMANCE_INDICATORS);
  }
  for (i, performance_indicator) in performance_indicators.iter().enumerate() {
    // prepare a node with impacting decisions
    let mut impacting_decisions_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_IMPACTING_DECISIONS).colon().build());
    for impacting_decision in performance_indicator.impacting_decisions() {
      impacting_decisions_builder.add_child(build_href(impacting_decision, colors));
    }
    let node_impacting_decisions = impacting_decisions_builder.build();
    let node_performance_indicator = builder_name(performance_indicator.name(), colors)
      .child(build_feel_name(performance_indicator.feel_name(), colors))
      .opt_child(build_label(performance_indicator.label(), colors))
      .opt_child(build_id(performance_indicator.opt_id(), colors))
      .opt_child(build_description(performance_indicator.description(), colors))
      .opt_child(build_uri(performance_indicator.uri(), colors))
      .child(node_impacting_decisions)
      .opt_child(build_extension_elements(performance_indicator.extension_elements(), colors))
      .opt_child(build_extension_attributes(performance_indicator.extension_attributes(), colors))
      .build();
    write_empty_line(w, i);
    let _ = write_indented(w, &node_performance_indicator, colors.mode(), indent);
  }
}

/// Write organisation units.
fn write_organisation_units(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let organisation_units = definitions.organisation_units();
  if !organisation_units.is_empty() {
    write_title(w, LABEL_ORGANISATION_UNITS);
  }
  for (i, organisation_units) in definitions.organisation_units().iter().enumerate() {
    // prepare a node with decisions made
    let mut decisions_made_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_DECISIONS_MADE).colon().build());
    for decision_made in organisation_units.decisions_made() {
      decisions_made_builder.add_child(build_href(decision_made, colors));
    }
    let node_decisions_made = decisions_made_builder.build();
    // prepare a node with decisions owned
    let mut decisions_made_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_DECISIONS_OWNED).colon().build());
    for decision_owned in organisation_units.decisions_owned() {
      decisions_made_builder.add_child(build_href(decision_owned, colors));
    }
    let node_decisions_owned = decisions_made_builder.build();
    let node_organisation_unit = builder_name(organisation_units.name(), colors)
      .child(build_feel_name(organisation_units.feel_name(), colors))
      .opt_child(build_label(organisation_units.label(), colors))
      .opt_child(build_id(organisation_units.opt_id(), colors))
      .opt_child(build_description(organisation_units.description(), colors))
      .opt_child(build_uri(organisation_units.uri(), colors))
      .child(node_decisions_made)
      .child(node_decisions_owned)
      .opt_child(build_extension_elements(organisation_units.extension_elements(), colors))
      .opt_child(build_extension_attributes(organisation_units.extension_attributes(), colors))
      .build();
    write_empty_line(w, i);
    let _ = write_indented(w, &node_organisation_unit, colors.mode(), indent);
  }
}

/// Prepares a node builder containing a name as a root element.
fn builder_name(text: &str, colors: &Colors) -> AsciiNodeBuilder {
  AsciiNode::node_builder(AsciiLine::builder().with_color(text, colors.name()).build())
}

/// Builds a leaf node containing a name.
fn build_name(text: &str, colors: &Colors) -> AsciiNode {
  build_labeled_text(LABEL_NAME, text, colors.name())
}

/// Builds a leaf node containing a FEEL name.
fn build_feel_name(feel_name: &Name, colors: &Colors) -> AsciiNode {
  build_labeled_text(LABEL_FEEL_NAME, &feel_name.to_string(), colors.name())
}

/// Builds a leaf node containing the value of the identifier.
fn build_id(id: Option<&String>, colors: &Colors) -> Option<AsciiNode> {
  build_opt_labeled_text(LABEL_ID, &id.cloned(), colors.id())
}

/// Builds a leaf node containing description.
fn build_description(opt_text: &Option<String>, colors: &Colors) -> Option<AsciiNode> {
  build_opt_labeled_multiline_text(LABEL_DESCRIPTION, opt_text, colors.description())
}

/// Builds a leaf node containing the value of the label.
fn build_label(opt_text: &Option<String>, colors: &Colors) -> Option<AsciiNode> {
  build_opt_labeled_text(LABEL_LABEL, opt_text, colors.label())
}

/// Builds a leaf node containing a label.
fn build_href(href: &HRef, colors: &Colors) -> AsciiNode {
  let text = href.id();
  AsciiNode::leaf_builder()
    .line(AsciiLine::builder().with_color("#", colors.href()).with_color(text, colors.href()).build())
    .build()
}

/// Builds a leaf node containing an URI.
fn build_uri(opt_text: &Option<String>, colors: &Colors) -> Option<AsciiNode> {
  build_opt_labeled_text(LABEL_URI, opt_text, colors.uri())
}

/// Builds a leaf node containing a labeled URI.
fn build_labeled_uri(label: &str, opt_text: &Option<String>, colors: &Colors) -> Option<AsciiNode> {
  build_opt_labeled_text(label, opt_text, colors.uri())
}

/// Builds a leaf node containing a namespace.
fn build_namespace_leaf(text: &str, colors: &Colors) -> AsciiNode {
  build_labeled_text(LABEL_NAMESPACE, text, colors.uri())
}

/// Builds a leaf node containing a type.
fn build_type(text: &str, colors: &Colors) -> AsciiNode {
  build_labeled_text(LABEL_TYPE, text, colors.typ())
}

/// Builds a node containing output variable properties.
fn build_variable(variable: &InformationItem, colors: &Colors) -> AsciiNode {
  AsciiNode::node_builder(AsciiLine::builder().text(LABEL_VARIABLE).build())
    .child(build_name(variable.name(), colors))
    .child(build_feel_name(variable.feel_name(), colors))
    .opt_child(build_id(variable.opt_id(), colors))
    .opt_child(build_label(variable.label(), colors))
    .child(build_type(variable.type_ref(), colors))
    .opt_child(build_description(variable.description(), colors))
    .opt_child(build_extension_elements(variable.extension_elements(), colors))
    .opt_child(build_extension_attributes(variable.extension_attributes(), colors))
    .build()
}

fn build_extension_elements(_extension_elements: &[ExtensionElement], _colors: &Colors) -> Option<AsciiNode> {
  None
}

fn build_extension_attributes(_extension_attributes: &[ExtensionAttribute], _colors: &Colors) -> Option<AsciiNode> {
  None
}

fn build_authority_requirements(authority_requirements: &Vec<AuthorityRequirement>, colors: &Colors) -> Option<AsciiNode> {
  if !authority_requirements.is_empty() {
    let mut authority_requirements_builder = AsciiNode::node_builder(AsciiLine::builder().text("authority requirements:").build());
    for authority_requirement in authority_requirements {
      let a = AsciiNode::node_builder(AsciiLine::builder().text("authority requirement").build())
        .opt_child(build_id(authority_requirement.opt_id(), colors))
        .opt_child(build_label(authority_requirement.label(), colors))
        .opt_child(build_description(authority_requirement.description(), colors))
        .opt_child(build_extension_elements(authority_requirement.extension_elements(), colors))
        .opt_child(build_extension_attributes(authority_requirement.extension_attributes(), colors))
        .build();
      authority_requirements_builder.add_child(a);
    }
    return Some(authority_requirements_builder.build());
  }
  None
}

/// Builds a leaf with a single line containing label and coloured value.
fn build_labeled_text(label: &str, text: &str, color: &str) -> AsciiNode {
  AsciiNode::leaf_builder()
    .line(AsciiLine::builder().text(label).colon_space().with_color(text, color).build())
    .build()
}

/// Builds optional a leaf with a single line containing label and coloured value.
fn build_opt_labeled_text(label: &str, opt_text: &Option<String>, color: &str) -> Option<AsciiNode> {
  opt_text.as_ref().map(|text| {
    AsciiNode::leaf_builder()
      .line(AsciiLine::builder().text(label).colon_space().with_color(text, color).build())
      .build()
  })
}

/// Builds a node containing optional labeled multiline text.
fn build_opt_labeled_multiline_text(label: &str, opt_text: &Option<String>, color: &str) -> Option<AsciiNode> {
  opt_text.as_ref().map(|text| {
    let mut leaf_builder = AsciiNode::leaf_builder();
    leaf_builder.add_line(AsciiLine::builder().text(label).colon().build());
    let mut min_spaces = if text.is_empty() { 0 } else { usize::MAX };
    for line in text.lines().filter(|s| !s.trim().is_empty()) {
      let space_count = line.len() - line.trim_start().len();
      if space_count < min_spaces {
        min_spaces = space_count;
      }
    }
    for line in text.lines().filter(|s| !s.trim().is_empty()).map(|s| s[min_spaces..].trim_end()) {
      leaf_builder.add_line(AsciiLine::builder().indent().with_color(line, color).build());
    }
    leaf_builder.build()
  })
}
