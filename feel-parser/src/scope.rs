//! Implementation of the scope used while parsing FEEL expressions.

use crate::context::ParsingContext;
use dsntk_feel::Name;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;

/// Parsing scope.
pub struct ParsingScope {
  /// The stack of parsing contexts.
  stack: RefCell<Vec<ParsingContext>>,
}

impl From<&dsntk_feel::FeelScope> for ParsingScope {
  /// Temporary - remove
  fn from(scope: &dsntk_feel::FeelScope) -> Self {
    let stack = RefCell::new(vec![]);
    for feel_context in scope.contexts() {
      stack.borrow_mut().push(feel_context.into());
    }
    Self { stack }
  }
}

impl Default for ParsingScope {
  /// Creates a default parsing scope containing single parsing context.
  fn default() -> Self {
    Self {
      stack: RefCell::new(vec![ParsingContext::default()]),
    }
  }
}

impl fmt::Display for ParsingScope {
  /// Converts parsing scope to text representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.stack.borrow_mut().iter().map(|ctx| ctx.to_string()).collect::<Vec<String>>().join(", "))
  }
}

impl ParsingScope {
  /// Returns a context removed from the top of the stack.
  pub fn pop(&self) -> Option<ParsingContext> {
    self.stack.borrow_mut().pop()
  }

  /// Puts a context on the top of the stack.
  pub fn push(&self, ctx: ParsingContext) {
    self.stack.borrow_mut().push(ctx);
  }

  /// Puts a default context on the top of the stack.
  pub fn push_default(&self) {
    self.stack.borrow_mut().push(ParsingContext::default());
  }

  /// Sets a specified name in context placed on the top of the stack.
  pub fn set_name(&self, name: Name) {
    if let Some(last_ctx) = self.stack.borrow_mut().last_mut() {
      last_ctx.set_name(name);
    }
  }

  /// Sets a context under a specified name in the context placed on the top of the stack.
  pub fn set_context(&self, name: Name, ctx: ParsingContext) {
    if let Some(last_ctx) = self.stack.borrow_mut().last_mut() {
      last_ctx.set_context(name, ctx);
    }
  }

  /// Returns a set of flattened keys from all contexts in scope.
  pub fn flattened_keys(&self) -> HashSet<String> {
    self.stack.borrow().iter().flat_map(|ctx| ctx.flattened_keys()).collect::<HashSet<String>>()
  }
}
