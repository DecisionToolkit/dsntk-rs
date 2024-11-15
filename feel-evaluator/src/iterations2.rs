//! # Implementation of `for`, `some` and `every` expressions

use dsntk_feel::context::FeelContext;
use dsntk_feel::values::{Value, Values};
use dsntk_feel::{FeelNumber, Name};

trait IterationState {
  fn bind_value(&mut self, _ctx: &FeelContext) {}
  fn set_value(&mut self, _ctx: &mut FeelContext) {}
  fn next(&mut self) -> bool {
    false
  }
  fn has_next(&self) -> bool {
    false
  }
  fn is_empty(&self) -> bool {
    false
  }
  fn is_variable(&self) -> bool {
    false
  }
}

/// Iteration stepping value.
#[derive(Debug)]
enum IterationDirection {
  Ascending,
  Descending,
}

struct NumberIntervalState {
  /// Name of the bound variable in this iterator state.
  variable: Name,
  /// Iteration starting value.
  start: FeelNumber,
  /// Iteration ending value.
  end: FeelNumber,
  /// Iteration step.
  step: FeelNumber,
  /// Iteration step direction.
  direction: IterationDirection,
  /// Current iteration value.
  current: FeelNumber,
}

impl NumberIntervalState {
  fn init(variable: Name, start: FeelNumber, end: FeelNumber) -> Box<dyn IterationState> {
    Box::new(Self {
      variable,
      start,
      end,
      step: FeelNumber::one(),
      direction: if start <= end { IterationDirection::Ascending } else { IterationDirection::Descending },
      current: start,
    })
  }
}

impl IterationState for NumberIntervalState {
  fn set_value(&mut self, ctx: &mut FeelContext) {
    ctx.set_entry(&self.variable, Value::Number(self.current));
  }

  fn next(&mut self) -> bool {
    match self.direction {
      IterationDirection::Ascending => {
        if self.current + self.step <= self.end {
          self.current += self.step;
          true
        } else {
          self.current = self.start;
          false
        }
      }
      IterationDirection::Descending => {
        if self.current - self.step >= self.end {
          self.current -= self.step;
          true
        } else {
          self.current = self.start;
          false
        }
      }
    }
  }

  fn has_next(&self) -> bool {
    match self.direction {
      IterationDirection::Ascending => self.current + self.step <= self.end,
      IterationDirection::Descending => self.current - self.step >= self.end,
    }
  }
}

struct ListState {
  /// Name of the bound variable in this iterator state.
  variable: Name,
  /// Values to iterate over.
  values: Values,
  /// Iteration step.
  step: usize,
  /// Current iteration index.
  current: usize,
}

impl ListState {
  fn init(variable: Name, value: Value) -> Box<dyn IterationState> {
    Box::new(Self {
      variable,
      values: match value {
        Value::List(values) => values,
        single => vec![single],
      },
      step: 1,
      current: 0,
    })
  }
}

impl IterationState for ListState {
  fn set_value(&mut self, ctx: &mut FeelContext) {
    if !self.values.is_empty() {
      ctx.set_entry(&self.variable, self.values[self.current].clone());
    }
  }

  fn next(&mut self) -> bool {
    if self.current + self.step < self.values.len() {
      self.current += self.step;
      true
    } else {
      self.current = 0;
      false
    }
  }

  fn has_next(&self) -> bool {
    self.current < self.values.len() - 1
  }

  fn is_empty(&self) -> bool {
    self.values.is_empty()
  }
}

struct VariableState {
  /// Name of the bound variable in this iterator state.
  variable: Name,
  /// Name of the bound variable in another iteration state.
  bound_variable: Name,
  /// Values to iterate over.
  values: Values,
  /// Iteration step.
  step: usize,
  /// Current iteration index.
  current: usize,
}

impl VariableState {
  fn init(variable: Name, bound_variable: Name) -> Box<dyn IterationState> {
    Box::new(Self {
      variable,
      bound_variable,
      values: vec![],
      step: 1,
      current: 0,
    })
  }
}

impl IterationState for VariableState {
  fn bind_value(&mut self, ctx: &FeelContext) {
    //TODO The order of checks can be optimized, comparing firstly with zero will be more optimal
    if let Some(value) = ctx.get(&self.bound_variable) {
      if self.current == 0 {
        self.values = match value {
          Value::List(values) => values.clone(),
          single => vec![single.clone()],
        };
      }
    }
  }

  fn set_value(&mut self, ctx: &mut FeelContext) {
    if !self.values.is_empty() {
      ctx.set_entry(&self.variable, self.values[self.current].clone());
    }
  }

  fn is_variable(&self) -> bool {
    true
  }
}

/// Iterator built from multiple iteration states.
#[derive(Default)]
pub struct FeelIterator {
  states: Vec<Box<dyn IterationState>>,
}

impl FeelIterator {
  /// Creates a new, empty iterator.
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_interval(&mut self, variable: Name, start: Value, end: Value) {
    if let Value::Number(start) = start {
      if let Value::Number(end) = end {
        self.states.push(NumberIntervalState::init(variable, start, end));
      }
    }
  }

  pub fn add_list(&mut self, variable: Name, value: Value) {
    self.states.push(ListState::init(variable, value));
  }

  pub fn add_variable(&mut self, variable: Name, bound_variable: Name) {
    self.states.push(VariableState::init(variable, bound_variable));
  }

  /// Iterates over all iteration states.
  pub fn iterate<F>(&mut self, mut handler: F)
  where
    F: FnMut(&FeelContext),
  {
    if self.states.is_empty() {
      return;
    }
    let mut ctx = FeelContext::new();
    loop {
      for state in self.states.iter_mut().rev().filter(|state| !state.is_variable()) {
        state.set_value(&mut ctx);
      }
      for state in self.states.iter_mut().filter(|state| state.is_variable()) {
        state.bind_value(&ctx);
        state.set_value(&mut ctx);
      }
      let is_empty_iteration = self.states.iter().any(|state| state.is_empty());
      if !is_empty_iteration {
        handler(&ctx);
      }
      let last_state_index = self.states.len() - 1;
      for (i, state) in self.states.iter_mut().rev().enumerate() {
        if i == last_state_index && !state.has_next() {
          return;
        }
        if state.next() {
          break;
        }
      }
    }
  }
}
