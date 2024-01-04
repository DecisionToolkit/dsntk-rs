//! Implementation of a node in Abstract Syntax Tree for `FEEL` grammar.

use dsntk_common::{write, AsciiLine, AsciiNode, ColorMode};
use dsntk_feel::{FeelType, Name};
use std::fmt;
use std::fmt::{Display, Write};

/// Node of the Abstract Syntax Tree for `FEEL` grammar.
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
  /// Node representing an arithmetic operator `+` (addition).
  Add(Box<AstNode>, Box<AstNode>),

  /// Node representing a logical operator `and` (conjunction).
  And(Box<AstNode>, Box<AstNode>),

  /// Node representing `@` (at) literal.
  At(String),

  /// Node representing a comparison operator `between`.
  Between(Box<AstNode>, Box<AstNode>, Box<AstNode>),

  /// Node representing a value of type `boolean`.
  Boolean(bool),

  /// Node representing a comma separated list of AST nodes, used internally by parser.
  CommaList(Vec<AstNode>),

  /// Node representing a context.
  /// Context entries are stored in the order of appearance in definition.
  Context(Vec<AstNode>),

  /// Node representing single context entry; key-value pair.
  ContextEntry(Box<AstNode>, Box<AstNode>),

  /// Node representing the key of the context entry; the key in context entry
  /// may be a name or string literal. String literals are converted to one segment names
  /// containing exactly the value of the string.
  ContextEntryKey(Name),

  /// Node representing the type of a context. Context type is defined by names
  /// and types of all entries. This node contains a collection of types
  /// for all context entries in the order of appearance in context type definition.
  ContextType(Vec<AstNode>),

  /// Node representing single context type entry.
  ContextTypeEntry(
    /// Node representing entry name of the context key.
    Box<AstNode>,
    /// Node representing `FEEL` type of the context entry.
    Box<AstNode>,
  ),

  /// Node representing the key of the entry in context type definition.
  /// In context type definition, only `FEEL` name is allowed as an entry key.
  ContextTypeEntryKey(Name),

  /// Node representing arithmetic operator `/` (division).
  Div(Box<AstNode>, Box<AstNode>),

  /// Node representing `equal` comparison.
  Eq(Box<AstNode>, Box<AstNode>),

  /// Node representing an expression evaluated as a body of `for` expression.
  EvaluatedExpression(Box<AstNode>),

  /// Quantified expression `every`.
  Every(
    /// Node representing quantified contexts.
    Box<AstNode>,
    /// Node representing an expression after `satisfies` clause.
    Box<AstNode>,
  ),

  /// Node representing exponential function.
  Exp(Box<AstNode>, Box<AstNode>),

  /// Node representing a list of expressions.
  ExpressionList(Vec<AstNode>),

  /// Node representing `FEEL` type.
  FeelType(FeelType),

  /// Node representing filter expression.
  Filter(Box<AstNode>, Box<AstNode>),

  /// Node representing `for` expression.
  For(
    /// Node representing [iteration contexts](AstNode::IterationContexts).
    Box<AstNode>,
    /// Node representing an expression to be evaluated.
    Box<AstNode>,
  ),

  /// Node representing function's formal parameter.
  FormalParameter(
    /// Node representing the name of the parameter.
    Box<AstNode>,
    /// Node representing the `FEEL` type of the parameter.
    Box<AstNode>,
  ),

  /// Node representing a list of formal parameters.
  FormalParameters(Vec<AstNode>),

  /// Node representing function's body. This node holds mandatory function body
  /// and a flag indicating if the function is external.
  FunctionBody(Box<AstNode>, bool),

  /// Node representing function definition.
  /// This node holds function's formal parameter list and  function's body.
  FunctionDefinition(Box<AstNode>, Box<AstNode>),

  /// Node representing function invocation.
  FunctionInvocation(Box<AstNode>, Box<AstNode>),

  /// Node representing function type.
  FunctionType(
    /// Node representing function's parameter types as [AstNode::ParameterTypes].
    Box<AstNode>,
    /// Node representing function's result type.
    Box<AstNode>,
  ),

  /// Node representing `greater or equal` comparison.
  Ge(Box<AstNode>, Box<AstNode>),

  /// Node representing `greater than` comparison.
  Gt(Box<AstNode>, Box<AstNode>),

  /// Node representing `if` expression.
  If(
    /// Node representing the condition.
    Box<AstNode>,
    /// Node representing the expression to be evaluated when condition is `true`.
    Box<AstNode>,
    /// Node representing the expression to be evaluated when condition is `false`.
    Box<AstNode>,
  ),

  /// Node representing `in` operator.
  In(Box<AstNode>, Box<AstNode>),

  /// Node representing type checking function.
  InstanceOf(
    /// Node representing the tested value.
    Box<AstNode>,
    /// Node representing `FELL` type to be checked.
    Box<AstNode>,
  ),

  /// Node representing the interval end used in ranges.
  IntervalEnd(Box<AstNode>, bool),

  /// Node representing the interval start used in ranges.
  IntervalStart(Box<AstNode>, bool),

  /// Node representing the comparison operator `irrelevant`.
  Irrelevant,

  /// List of iteration contexts.
  IterationContexts(Vec<AstNode>),

  /// Node representing iteration context containing the variable name and a list of elements to iterate over.
  IterationContextList(
    /// Node representing variable name used in this iteration context.
    Box<AstNode>,
    /// Node representing a single list of elements to iterate over.
    Box<AstNode>,
  ),

  /// Node representing iteration context containing the variable name and a range of numbers to iterate over.
  IterationContextRange(
    /// Node representing variable name used in this iteration context.
    Box<AstNode>,
    /// Node representing the **start** of the range of numbers to iterate over.
    Box<AstNode>,
    /// Node representing the **end** of the range of numbers to iterate over.
    Box<AstNode>,
  ),

  /// Node representing `less or equal` comparison.
  Le(Box<AstNode>, Box<AstNode>),

  /// Node representing `less than` comparison.
  Lt(Box<AstNode>, Box<AstNode>),

  /// Node representing a list.
  List(Vec<AstNode>),

  /// Node representing a list type.
  ListType(Box<AstNode>),

  /// Node representing arithmetic operator `*` (multiplication).
  Mul(Box<AstNode>, Box<AstNode>),

  /// Node representing a `FEEL` name.
  Name(Name),

  /// Node representing single named parameter.
  NamedParameter(
    /// Node representing parameter name.
    Box<AstNode>,
    /// Node representing parameter type.
    Box<AstNode>,
  ),

  /// Node representing a collection of named parameters.
  NamedParameters(Vec<AstNode>),

  /// Node representing a negated list (used in negated unary tests).
  NegatedList(Vec<AstNode>),

  /// Node representing an unary arithmetic negation `-`.
  Neg(Box<AstNode>),

  /// Node representing `not equal` comparison.
  Nq(Box<AstNode>, Box<AstNode>),

  /// Node representing a value of type `Null`.
  Null,

  /// Node representing a value of type `number`.
  Numeric(
    /// Digits before decimal separator.
    String,
    /// Digits after decimal separator.
    String,
    /// Sign of the exponent.
    char,
    /// Digits of the exponent.
    String,
  ),

  /// Node representing a logical operator `or` (disjunction).
  Or(Box<AstNode>, Box<AstNode>),

  /// Node representing expression for selecting decision table's output value.
  Out(Box<AstNode>, Box<AstNode>),

  /// Node representing a name of the function's formal parameter.
  ParameterName(Name),

  /// Node representing a collection of function parameter types.
  ParameterTypes(Vec<AstNode>),

  /// Node representing a path expression.
  Path(Box<AstNode>, Box<AstNode>),

  /// Node representing a collection of positional parameters.
  PositionalParameters(Vec<AstNode>),

  /// Node representing a collection of names that constitute a qualified name.
  QualifiedName(Vec<AstNode>),

  /// Node representing a segment of a qualified name.
  QualifiedNameSegment(Name),

  /// List of quantified contexts.
  QuantifiedContexts(Vec<AstNode>),

  /// Quantified context containing variable name and evaluation expression.
  QuantifiedContext(
    /// Node representing variable name used in this quantified context.
    Box<AstNode>,
    /// Node representing evaluation expression.
    Box<AstNode>,
  ),

  /// Node representing a range of values.
  Range(Box<AstNode>, Box<AstNode>),

  /// Node representing range type.
  RangeType(Box<AstNode>),

  /// Node representing `satisfies` clause in quantified expression.
  Satisfies(Box<AstNode>),

  /// Node representing quantified expression `some`.
  Some(
    /// Node representing quantified contexts.
    Box<AstNode>,
    /// Node representing an expression after `satisfies` clause.
    Box<AstNode>,
  ),

  /// Node representing a value of type `string`.
  String(String),

  /// Node representing an arithmetic operator `-` (subtraction).
  Sub(Box<AstNode>, Box<AstNode>),

  /// Node representing unary comparison operator `>=`.
  UnaryGe(Box<AstNode>),

  /// Node representing unary comparison operator `>`.
  UnaryGt(Box<AstNode>),

  /// Node representing unary comparison operator `<=>`.
  UnaryLe(Box<AstNode>),

  /// Node representing unary comparison operator `<less than>`.
  UnaryLt(Box<AstNode>),

  /// Node representing unary comparison operator `=`.
  UnaryEq(Box<AstNode>),

  /// Node representing unary comparison operator `!=`.
  UnaryNe(Box<AstNode>),
}

impl Display for AstNode {
  /// Converts [AstNode] to textual representation, including child nodes.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}\n    ", ast_tree(self, &ColorMode::Off))
  }
}

impl AstNode {
  /// Prints a trace of the AST, starting from specified node.
  pub fn trace(&self) -> String {
    let output = format!("      AST:{self}");
    println!("{output}");
    output
  }
}

/// Returns ASCII tree representation of the specified node.
pub fn ast_tree(node: &AstNode, color_mode: &ColorMode) -> String {
  let mut tree = String::new();
  let _ = write(&mut tree, &ast_node_to_tree(node), color_mode);
  let mut output = String::new();
  for line in tree.lines() {
    let _ = write!(&mut output, "\n      {}", line);
  }
  output
}

/// Converts [AstNode] into ASCII [AsciiNode] node.
fn ast_node_to_tree(node: &AstNode) -> AsciiNode {
  match node {
    AstNode::Add(lhs, rhs) => node_2("Add", lhs, rhs),
    AstNode::And(lhs, rhs) => node_2("And", lhs, rhs),
    AstNode::At(mid) => node_and_leaf("At", &format!("`{mid}`")),
    AstNode::Between(lhs, mid, rhs) => node_3("Between", lhs, mid, rhs),
    AstNode::Boolean(mid) => node_and_leaf("Boolean", &format!("`{mid}`")),
    AstNode::CommaList(mid) => node_n("CommaList", mid),
    AstNode::Context(items) => node_n("Context", items),
    AstNode::ContextEntry(lhs, rhs) => node_2("ContextEntry", lhs, rhs),
    AstNode::ContextEntryKey(mid) => node_and_leaf("ContextEntryKey", &format!("`{mid}`")),
    AstNode::ContextType(items) => node_n("ContextType", items),
    AstNode::ContextTypeEntry(lhs, rhs) => node_2("ContextTypeEntry", lhs, rhs),
    AstNode::ContextTypeEntryKey(mid) => node_and_leaf("Name", &format!("`{mid}`")),
    AstNode::Div(lhs, rhs) => node_2("Div", lhs, rhs),
    AstNode::Eq(lhs, rhs) => node_2("Eq", lhs, rhs),
    AstNode::EvaluatedExpression(mid) => node_1("EvaluatedExpression", mid),
    AstNode::Every(lhs, rhs) => node_2("Every", lhs, rhs),
    AstNode::Exp(lhs, rhs) => node_2("Exp", lhs, rhs),
    AstNode::ExpressionList(items) => node_n("ExpressionList", items),
    AstNode::FeelType(lhs) => node_and_leaf("FeelType", &lhs.to_string()),
    AstNode::Filter(lhs, rhs) => node_2("Filter", lhs, rhs),
    AstNode::For(lhs, rhs) => node_2("For", lhs, rhs),
    AstNode::FormalParameter(lhs, rhs) => node_2("FormalParameter", lhs, rhs),
    AstNode::FormalParameters(items) => node_n("FormalParameters", items),
    AstNode::FunctionBody(lhs, external) => node_and_label("FunctionBody", lhs, " (external)", "", *external),
    AstNode::FunctionDefinition(lhs, rhs) => node_2("FunctionDefinition", lhs, rhs),
    AstNode::FunctionInvocation(lhs, rhs) => node_2("FunctionInvocation", lhs, rhs),
    AstNode::FunctionType(lhs, rhs) => node_2("FunctionType", lhs, rhs),
    AstNode::Ge(lhs, rhs) => node_2("Ge", lhs, rhs),
    AstNode::Gt(lhs, rhs) => node_2("Gt", lhs, rhs),
    AstNode::If(lhs, mid, rhs) => node_3("If", lhs, mid, rhs),
    AstNode::In(lhs, rhs) => node_2("In", lhs, rhs),
    AstNode::InstanceOf(lhs, rhs) => node_2("InstanceOf", lhs, rhs),
    AstNode::IntervalEnd(lhs, closed) => node_and_label("IntervalEnd", lhs, " (closed)", " (opened)", *closed),
    AstNode::IntervalStart(lhs, closed) => node_and_label("IntervalStart", lhs, " (closed)", " (opened)", *closed),
    AstNode::Irrelevant => leaf("Irrelevant"),
    AstNode::IterationContexts(items) => node_n("IterationContexts", items),
    AstNode::IterationContextList(lhs, rhs) => node_2("IterationContextSingle", lhs, rhs),
    AstNode::IterationContextRange(lhs, mid, rhs) => node_3("IterationContextRange", lhs, mid, rhs),
    AstNode::Le(lhs, rhs) => node_2("Le", lhs, rhs),
    AstNode::List(mid) => node_n("List", mid),
    AstNode::ListType(lhs) => node_1("ListType", lhs),
    AstNode::Lt(lhs, rhs) => node_2("Lt", lhs, rhs),
    AstNode::Mul(lhs, rhs) => node_2("Mul", lhs, rhs),
    AstNode::Name(mid) => node_and_leaf("Name", &format!("`{mid}`")),
    AstNode::NamedParameter(lhs, rhs) => node_2("NamedParameter", lhs, rhs),
    AstNode::NamedParameters(items) => node_n("NamedParameters", items),
    AstNode::Neg(mid) => node_1("Neg", mid),
    AstNode::NegatedList(mid) => node_n("NegatedList", mid),
    AstNode::Nq(lhs, rhs) => node_2("Nq", lhs, rhs),
    AstNode::Null => leaf("Null"),
    AstNode::Numeric(before, after, sign, exponent) => node_and_leaf("Numeric", &numeric_to_tree_string(before, after, sign, exponent)),
    AstNode::Or(lhs, rhs) => node_2("Or", lhs, rhs),
    AstNode::Out(lhs, rhs) => node_2("Out", lhs, rhs),
    AstNode::ParameterName(lhs) => node_and_leaf("ParameterName", &format!("`{lhs}`")),
    AstNode::ParameterTypes(items) => node_n("ParameterTypes", items),
    AstNode::Path(lhs, rhs) => node_2("Path", lhs, rhs),
    AstNode::PositionalParameters(items) => node_n("PositionalParameters", items),
    AstNode::QualifiedName(items) => node_n("QualifiedName", items),
    AstNode::QualifiedNameSegment(lhs) => node_and_leaf("Name", &format!("`{lhs}`")),
    AstNode::QuantifiedContext(lhs, rhs) => node_2("QuantifiedContext", lhs, rhs),
    AstNode::QuantifiedContexts(items) => node_n("QuantifiedContexts", items),
    AstNode::Range(lhs, rhs) => node_2("Range", lhs, rhs),
    AstNode::RangeType(lhs) => node_1("RangeType", lhs),
    AstNode::Satisfies(mid) => node_1("Satisfies", mid),
    AstNode::Some(lhs, rhs) => node_2("Some", lhs, rhs),
    AstNode::String(mid) => node_and_leaf("String", &format!("`{mid}`")),
    AstNode::Sub(lhs, rhs) => node_2("Sub", lhs, rhs),
    AstNode::UnaryGe(mid) => node_1("UnaryGe", mid),
    AstNode::UnaryGt(mid) => node_1("UnaryGt", mid),
    AstNode::UnaryLe(mid) => node_1("UnaryLe", mid),
    AstNode::UnaryLt(mid) => node_1("UnaryLt", mid),
    AstNode::UnaryEq(mid) => node_1("UnaryEq", mid),
    AstNode::UnaryNe(mid) => node_1("UnaryNe", mid),
  }
}

/// Creates a tree node with one child node.
fn node_1(name: &str, mid: &AstNode) -> AsciiNode {
  AsciiNode::node_builder(AsciiLine::builder().text(name).build()).child(ast_node_to_tree(mid)).build()
}

/// Creates a tree node with two child nodes.
fn node_2(name: &str, lhs: &AstNode, rhs: &AstNode) -> AsciiNode {
  AsciiNode::node_builder(AsciiLine::builder().text(name).build())
    .child(ast_node_to_tree(lhs))
    .child(ast_node_to_tree(rhs))
    .build()
}

/// Creates a tree node with three child nodes.
fn node_3(name: &str, lhs: &AstNode, mid: &AstNode, rhs: &AstNode) -> AsciiNode {
  AsciiNode::node_builder(AsciiLine::builder().text(name).build())
    .child(ast_node_to_tree(lhs))
    .child(ast_node_to_tree(mid))
    .child(ast_node_to_tree(rhs))
    .build()
}

/// Creates a tree node with multiple child nodes.
fn node_n(name: &str, items: &[AstNode]) -> AsciiNode {
  let mut node_builder = AsciiNode::node_builder(AsciiLine::builder().text(name).build());
  if items.is_empty() {
    node_builder.add_child(AsciiNode::leaf_builder().line(AsciiLine::builder().text("(empty)").build()).build());
  } else {
    for item in items {
      node_builder.add_child(ast_node_to_tree(item));
    }
  }
  node_builder.build()
}

/// Creates a node with single leaf node.
fn node_and_leaf(name: &str, leaf: &str) -> AsciiNode {
  AsciiNode::node_builder(AsciiLine::builder().text(name).build())
    .child(AsciiNode::leaf_builder().line(AsciiLine::builder().text(leaf).build()).build())
    .build()
}

/// Creates a single node with additional label.
fn node_and_label(name: &str, lhs: &AstNode, label_true: &str, label_false: &str, label_flag: bool) -> AsciiNode {
  let name_label = if label_flag { label_true } else { label_false };
  AsciiNode::node_builder(AsciiLine::builder().text(name).text(name_label).build())
    .child(ast_node_to_tree(lhs))
    .build()
}

/// Creates a leaf node.
fn leaf(leaf: &str) -> AsciiNode {
  AsciiNode::leaf_builder().line(AsciiLine::builder().text(leaf).build()).build()
}

fn numeric_to_tree_string(before: &str, after: &str, _sign: &char, _exponent: &str) -> String {
  let mut output = String::new();
  let _ = write!(&mut output, "`{before}");
  if !after.is_empty() {
    let _ = write!(&mut output, ".{after}");
  }
  let _ = write!(&mut output, "`");
  output
}
