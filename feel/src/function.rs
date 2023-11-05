//! `FEEL` or `DMN` function body definition.

use crate::values::Value;
use crate::{Evaluator, FeelScope};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

/// Type alias of the closure that evaluates `FEEL` or `DMN` function body into [Value].
pub type FunctionBodyEvaluator = Arc<Evaluator>;

/// Function body may be defined multiple ways using `FEEL` or `DMN`.
/// This enum is the representation of all of these cases.
#[derive(Clone)]
pub enum FunctionBody {
  /// Function body created from context defined in `DMN` model.
  Context(FunctionBodyEvaluator),
  /// Function body created from `FEEL` textual expression defined in `DMN` model.
  LiteralExpression(FunctionBodyEvaluator),
  /// Function body created from decision table defined in `DMN` model.
  DecisionTable(FunctionBodyEvaluator),
  /// Function body created from function definition defined in `DMN` model.
  FunctionDefinition(FunctionBodyEvaluator),
  /// Function body created from invocation defined in `DMN` model.
  Invocation(FunctionBodyEvaluator),
  /// Function body created from relation defined in `DMN` model.
  Relation(FunctionBodyEvaluator),
  /// Function body created from decision service defined in `DMN` model.
  DecisionService(FunctionBodyEvaluator),
  /// Function body created from externally defined function in `Java` or `PMML`.
  External(FunctionBodyEvaluator),
}

impl FunctionBody {
  /// Evaluates function body, takes a [FeelScope] as input and returns evaluated [Value].
  pub fn evaluate(&self, scope: &FeelScope) -> Value {
    match self {
      FunctionBody::Context(evaluator) => evaluator(scope),
      FunctionBody::LiteralExpression(evaluator) => evaluator(scope),
      FunctionBody::DecisionTable(evaluator) => evaluator(scope),
      FunctionBody::FunctionDefinition(evaluator) => evaluator(scope),
      FunctionBody::Invocation(evaluator) => evaluator(scope),
      FunctionBody::Relation(evaluator) => evaluator(scope),
      FunctionBody::DecisionService(evaluator) => evaluator(scope),
      FunctionBody::External(evaluator) => evaluator(scope),
    }
  }
}

impl Debug for FunctionBody {
  ///
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      FunctionBody::Context(_) => write!(f, "FunctionBodyContext"),
      FunctionBody::LiteralExpression(_) => write!(f, "FunctionBodyLiteralExpression"),
      FunctionBody::DecisionTable(_) => write!(f, "FunctionBodyDecisionTable"),
      FunctionBody::FunctionDefinition(_) => write!(f, "FunctionBodyFunctionDefinition"),
      FunctionBody::Invocation(_) => write!(f, "FunctionBodyInvocation"),
      FunctionBody::Relation(_) => write!(f, "FunctionBodyRelation"),
      FunctionBody::DecisionService(_) => write!(f, "FunctionBodyDecisionService"),
      FunctionBody::External(_) => write!(f, "FunctionBodyExternal"),
    }
  }
}

impl PartialEq for FunctionBody {
  ///
  fn eq(&self, other: &Self) -> bool {
    match self {
      FunctionBody::Context(_) => matches!(other, FunctionBody::Context(_)),
      FunctionBody::LiteralExpression(_) => matches!(other, FunctionBody::LiteralExpression(_)),
      FunctionBody::DecisionTable(_) => matches!(other, FunctionBody::DecisionTable(_)),
      FunctionBody::FunctionDefinition(_) => matches!(other, FunctionBody::FunctionDefinition(_)),
      FunctionBody::Invocation(_) => matches!(other, FunctionBody::Invocation(_)),
      FunctionBody::Relation(_) => matches!(other, FunctionBody::Relation(_)),
      FunctionBody::DecisionService(_) => matches!(other, FunctionBody::DecisionService(_)),
      FunctionBody::External(_) => matches!(other, FunctionBody::External(_)),
    }
  }
}
