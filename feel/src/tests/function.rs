use crate::values::Value;
use crate::{value_number, FeelScope, FunctionBody};
use std::sync::Arc;

#[test]
#[allow(clippy::redundant_clone)]
fn test_function_body_context() {
  let scope = &FeelScope::default();
  let fun_body = FunctionBody::Context(Arc::new(Box::new(|_: &FeelScope| value_number!(1))));
  assert_eq!(value_number!(1), fun_body.evaluate(scope));
  assert_eq!("FunctionBodyContext", format!("{fun_body:?}"));
  let fun_body_a = FunctionBody::Context(Arc::new(Box::new(|_: &FeelScope| value_number!(10))));
  assert_eq!(fun_body, fun_body_a);
  let fun_body_clone = fun_body.clone();
  assert_eq!(fun_body, fun_body_clone);
  assert_eq!(value_number!(1), fun_body_clone.evaluate(scope));
  assert_eq!("FunctionBodyContext", format!("{fun_body_clone:?}"))
}

#[test]
fn test_function_body_literal_expression() {
  let scope = &FeelScope::default();
  let fun_body = FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &FeelScope| value_number!(2))));
  assert_eq!(value_number!(2), fun_body.evaluate(scope));
  assert_eq!("FunctionBodyLiteralExpression", format!("{fun_body:?}"));
  let fun_body_a = FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &FeelScope| value_number!(20))));
  assert_eq!(fun_body, fun_body_a);
  assert_eq!(fun_body, fun_body.clone());
}

#[test]
fn test_function_body_decision_table() {
  let scope = &FeelScope::default();
  let fun_body = FunctionBody::DecisionTable(Arc::new(Box::new(|_: &FeelScope| value_number!(3))));
  assert_eq!(value_number!(3), fun_body.evaluate(scope));
  assert_eq!("FunctionBodyDecisionTable", format!("{fun_body:?}"));
  let fun_body_a = FunctionBody::DecisionTable(Arc::new(Box::new(|_: &FeelScope| value_number!(30))));
  assert_eq!(fun_body, fun_body_a);
  assert_eq!(fun_body, fun_body.clone());
}

#[test]
fn test_function_body_function_definition() {
  let scope = &FeelScope::default();
  let fun_body = FunctionBody::FunctionDefinition(Arc::new(Box::new(|_: &FeelScope| value_number!(4))));
  assert_eq!(value_number!(4), fun_body.evaluate(scope));
  assert_eq!("FunctionBodyFunctionDefinition", format!("{fun_body:?}"));
  let fun_body_a = FunctionBody::FunctionDefinition(Arc::new(Box::new(|_: &FeelScope| value_number!(40))));
  assert_eq!(fun_body, fun_body_a);
  assert_eq!(fun_body, fun_body.clone());
}

#[test]
fn test_function_body_invocation() {
  let scope = &FeelScope::default();
  let fun_body = FunctionBody::Invocation(Arc::new(Box::new(|_: &FeelScope| value_number!(5))));
  assert_eq!(value_number!(5), fun_body.evaluate(scope));
  assert_eq!("FunctionBodyInvocation", format!("{fun_body:?}"));
  let fun_body_a = FunctionBody::Invocation(Arc::new(Box::new(|_: &FeelScope| value_number!(50))));
  assert_eq!(fun_body, fun_body_a);
  assert_eq!(fun_body, fun_body.clone());
}

#[test]
fn test_function_body_relation() {
  let scope = &FeelScope::default();
  let fun_body = FunctionBody::Relation(Arc::new(Box::new(|_: &FeelScope| value_number!(6))));
  assert_eq!(value_number!(6), fun_body.evaluate(scope));
  assert_eq!("FunctionBodyRelation", format!("{fun_body:?}"));
  let fun_body_a = FunctionBody::Relation(Arc::new(Box::new(|_: &FeelScope| value_number!(60))));
  assert_eq!(fun_body, fun_body_a);
  assert_eq!(fun_body, fun_body.clone());
}

#[test]
fn test_function_body_decision_service() {
  let scope = &FeelScope::default();
  let fun_body = FunctionBody::DecisionService(Arc::new(Box::new(|_: &FeelScope| value_number!(7))));
  assert_eq!(value_number!(7), fun_body.evaluate(scope));
  assert_eq!("FunctionBodyDecisionService", format!("{fun_body:?}"));
  let fun_body_a = FunctionBody::DecisionService(Arc::new(Box::new(|_: &FeelScope| value_number!(70))));
  assert_eq!(fun_body, fun_body_a);
  assert_eq!(fun_body, fun_body.clone());
}

#[test]
fn test_function_body_external() {
  let scope = &FeelScope::default();
  let fun_body = FunctionBody::External(Arc::new(Box::new(|_: &FeelScope| value_number!(8))));
  assert_eq!(value_number!(8), fun_body.evaluate(scope));
  assert_eq!("FunctionBodyExternal", format!("{fun_body:?}"));
  let fun_body_a = FunctionBody::External(Arc::new(Box::new(|_: &FeelScope| value_number!(80))));
  assert_eq!(fun_body, fun_body_a);
  assert_eq!(fun_body, fun_body.clone());
}
