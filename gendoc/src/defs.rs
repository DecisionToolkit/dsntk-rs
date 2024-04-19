//! # HTML and CSS common definitions

//--------------------------------------------------------------------------------------------------
// FONTS
//--------------------------------------------------------------------------------------------------

pub const FONT_NORMAL: &str = "https://fonts.googleapis.com/css2?family=Barlow:ital,wght@0,300;0,400;0,500;0,600;1,300;1,400;1,500;1,600&display=swap";
pub const FONT_CONDENSED: &str = "https://fonts.googleapis.com/css2?family=Barlow+Condensed:ital,wght@0,300;0,400;0,500;0,600;1,300;1,400;1,500;1,600&display=swap";
pub const FONT_MONO: &str = "https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,300;0,400;0,500;0,600;1,300;1,400;1,500;1,600&display=swap";

//--------------------------------------------------------------------------------------------------
// CSS DEFINITIONS
//--------------------------------------------------------------------------------------------------

/// Class name for the container for diagram.
pub const CLASS_DIAGRAM_CONTAINER: &str = "diagram-container";

/// Class name for diagram title.
pub const CLASS_DIAGRAM_TITLE: &str = "diagram-title";

/// Class name for diagram section.
pub const CLASS_DIAGRAM_SECTION: &str = "diagram-section";

/// Class name for the container for model element details.
pub const CLASS_MODEL_ELEMENT_CONTAINER: &str = "model-element-container";

/// Class name for the name of the model element.
pub const CLASS_MODEL_ELEMENT_NAME: &str = "model-element-name";

/// Class name for the type of the model element.
pub const CLASS_MODEL_ELEMENT_TYPE: &str = "model-element-type";

/// Class name for the element displaying `description`.
pub const CLASS_DESCRIPTION: &str = "description";

/// Class name for the element containing a `description`.
pub const CLASS_DESCRIPTION_CONTAINER: &str = "description-container";

/// Class name for the container for expression instance details.
pub const CLASS_EXPRESSION_INSTANCE_CONTAINER: &str = "expression-instance-container";

/// Class name for literal expression.
pub const CLASS_LITERAL_EXPRESSION: &str = "literal-expression";

/// Class name for decision table.
pub const CLASS_DECISION_TABLE: &str = "decision-table";

/// Class name for information item name.
pub const CLASS_INFORMATION_ITEM_NAME: &str = "information-item-name";

/// Class name for grid container.
pub const CLASS_GRID_CONTAINER: &str = "grid-container";

/// Class name for grid body.
pub const CLASS_GRID_BODY: &str = "grid-body";

/// Class name for horizontal double line.
pub const CLASS_HORIZONTAL_DOUBLE_LINE: &str = "horizontal-double-line";

/// Class name for vertical double line.
pub const CLASS_VERTICAL_DOUBLE_LINE: &str = "vertical-double-line";

/// Class name for hit policy.
pub const CLASS_HIT_POLICY: &str = "hit-policy";

/// Class name for input expression.
pub const CLASS_INPUT_EXPRESSION: &str = "input-expression";

/// Class name for input allowed values.
pub const CLASS_INPUT_ALLOWED_VALUES: &str = "input-allowed-values";

/// Class name for input entry.
pub const CLASS_INPUT_ENTRY: &str = "input-entry";

/// Class name for output label.
pub const CLASS_OUTPUT_LABEL: &str = "output-label";

/// Class name for output component.
pub const CLASS_OUTPUT_COMPONENT: &str = "output-component";

/// Class name for output allowed values.
pub const CLASS_OUTPUT_ALLOWED_VALUES: &str = "output-allowed-values";

/// Class name for output entry.
pub const CLASS_OUTPUT_ENTRY: &str = "output-entry";

/// Class name for rule number.
pub const CLASS_RULE_NUMBER: &str = "rule-number";

/// Class name for annotation label.
pub const CLASS_ANNOTATION_LABEL: &str = "annotation-label";

/// Class name for annotation allowed values.
pub const CLASS_ANNOTATION_ALLOWED_VALUES: &str = "annotation-allowed-values";

/// Class name for annotation entry.
pub const CLASS_ANNOTATION_ENTRY: &str = "annotation-entry";

/// Class name for variable details' container.
pub const CLASS_VARIABLE_DETAILS_CONTAINER: &str = "variable-details-container";

/// Class name for variable details heading.
pub const CLASS_VARIABLE_DETAILS_HEADING: &str = "variable-details-heading";

/// Class name for variable details properties.
pub const CLASS_VARIABLE_DETAILS_PROPERTIES: &str = "variable-details-properties";

/// Class name for variable details property name.
pub const CLASS_VARIABLE_DETAILS_PROPERTY_NAME: &str = "variable-details-property-name";

/// Class name for variable details property value.
pub const CLASS_VARIABLE_DETAILS_PROPERTY_VALUE: &str = "variable-details-property-value";

/// Class name for variable details property value type.
pub const CLASS_VARIABLE_DETAILS_PROPERTY_VALUE_TYPE: &str = "variable-details-property-value-type";

//--------------------------------------------------------------------------------------------------
// HTML DEFINITIONS
//--------------------------------------------------------------------------------------------------

/// Text of the heading displayed before model diagrams section.
pub const HEADING_MODEL_DIAGRAMS: &str = "Model diagrams";

/// Text of the heading displayed before model elements section.
pub const HEADING_MODEL_ELEMENTS: &str = "Model elements";

/// Text of the heading displayed before the output variable properties.
pub const HEADING_OUTPUT_DATA: &str = "Output data";

/// Text of the heading displayed before the input variable properties.
pub const HEADING_INPUT_DATA: &str = "Input data";

//--------------------------------------------------------------------------------------------------
// DATA DEFINITIONS
//--------------------------------------------------------------------------------------------------

/// Empty element content.
pub const NO_CONTENT: &str = "";
