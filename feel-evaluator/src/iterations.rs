//! # FEEL iterator implementation

use dsntk_feel::context::FeelContext;
use dsntk_feel::values::{Value, Values, VALUE_FALSE, VALUE_TRUE};
use dsntk_feel::{value_null, Evaluator, FeelScope, Name};
use std::num::NonZeroIsize;

/// Iterator types.
#[derive(Debug, Clone)]
enum IteratorType {
  /// Iterator over a range of values.
  Range,
  /// Iterator over a list of values.
  List,
  /// Iterator over value(s) of bound variable defined in another iteration context.
  Variable(Name),
}

/// Iteration state.
struct IterationState {
  /// Type of the iterator.
  iterator_type: IteratorType,
  /// Name of the bound variable of the iteration context.
  variable: Name,
  /// Current value of the looping index.
  index: isize,
  /// Iteration step.
  step: NonZeroIsize,
  /// Iteration start value.
  start: isize,
  /// Iteration end value.
  end: isize,
  /// Collection of FEEL values to iterate over.
  values: Option<Values>,
  /// Flag indicating whether the current iteration is empty (does not iterate anymore).
  empty: bool,
}

impl IterationState {
  /// Creates a new iteration state for a range.
  fn new_range(variable: Name, start: isize, end: isize) -> Self {
    Self {
      iterator_type: IteratorType::Range,
      variable,
      index: start,
      step: if start <= end { NonZeroIsize::new(1).unwrap() } else { NonZeroIsize::new(-1).unwrap() },
      start,
      end,
      values: None,
      empty: true,
    }
  }

  /// Creates a new iteration state for a list of values.
  fn new_list(variable: Name, value: Value) -> Self {
    let values = match value {
      Value::List(values) => values,
      other => vec![other],
    };
    Self {
      iterator_type: IteratorType::List,
      variable,
      index: 0,
      step: NonZeroIsize::new(1).unwrap(),
      start: 0,
      end: (values.len() as isize) - 1,
      values: Some(values),
      empty: true,
    }
  }

  /// Creates a new iteration state for bound variable.
  fn new_variable(variable: Name, bound_variable: Name) -> Self {
    Self {
      iterator_type: IteratorType::Variable(bound_variable),
      variable,
      index: 0,
      step: NonZeroIsize::new(1).unwrap(),
      start: 0,
      end: 0,
      values: None,
      empty: true,
    }
  }

  /// ???
  fn bind_value(&mut self, ctx: &FeelContext) {
    if let IteratorType::Variable(bound_variable) = &self.iterator_type {
      if let Some(value) = ctx.get(bound_variable) {
        if self.index == self.start {
          let values = match value {
            Value::List(values) => values.clone(),
            other => vec![other.clone()],
          };
          self.index = 0;
          self.start = 0;
          self.end = (values.len() as isize) - 1;
          self.values = Some(values);
        }
      }
    }
  }

  /// Moves the iterator to the next value if any available.
  fn next(&mut self) -> bool {
    let s = self.step.get();
    if self.step.is_positive() {
      if self.index + s <= self.end {
        self.index += s;
        true
      } else {
        self.index = self.start;
        false
      }
    } else if self.index + s >= self.end {
      self.index += s;
      true
    } else {
      self.index = self.start;
      false
    }
  }

  /// Returns `true` when the iterator has some more values to iterate over.
  fn has_next(&self) -> bool {
    if self.step.is_positive() {
      self.index + self.step.get() <= self.end
    } else {
      self.index + self.step.get() >= self.end
    }
  }

  ///
  fn set_value(&mut self, ctx: &mut FeelContext) {
    match self.iterator_type {
      IteratorType::Range => {
        let value = Value::Number(self.index.into());
        ctx.set_entry(&self.variable, value);
        self.empty = false;
        return;
      }
      _ => {
        if let Some(values) = &self.values {
          let index = self.index as usize;
          if let Some(value) = values.get(index) {
            ctx.set_entry(&self.variable, value.clone());
            self.empty = false;
            return;
          }
        }
      }
    }
    self.empty = true;
  }

  ///
  fn is_variable(&self) -> bool {
    matches!(self.iterator_type, IteratorType::Variable(_))
  }
}

/// Iterator built from multiple iteration states.
#[derive(Default)]
pub(crate) struct FeelIterator {
  states: Vec<IterationState>,
}

impl FeelIterator {
  /// Creates a new iterator with default settings.
  pub fn new() -> Self {
    Self::default()
  }

  ///
  pub fn add_range(&mut self, variable: Name, start: isize, end: isize) {
    self.states.push(IterationState::new_range(variable, start, end));
  }

  ///
  pub fn add_list(&mut self, variable: Name, value: Value) {
    self.states.push(IterationState::new_list(variable, value));
  }

  ///
  pub fn add_variable(&mut self, variable: Name, bound_variable: Name) {
    self.states.push(IterationState::new_variable(variable, bound_variable));
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
      for state in self.iter_states_non_variable_mut() {
        state.set_value(&mut ctx);
      }
      for state in self.iter_states_variable_mut() {
        state.bind_value(&ctx);
        state.set_value(&mut ctx);
      }
      let empty_iteration = self.states.iter().fold(false, |acc, state| acc | state.empty);
      if !empty_iteration {
        handler(&ctx);
      }
      let last_state = self.states.len() - 1;
      for (i, state) in self.iter_states_mut().enumerate() {
        if i == last_state && !state.has_next() {
          return;
        }
        if state.next() {
          break;
        }
      }
    }
  }

  ///
  fn iter_states_mut(&mut self) -> impl Iterator<Item = &mut IterationState> {
    self.states.iter_mut().rev()
  }

  ///
  fn iter_states_non_variable_mut(&mut self) -> impl Iterator<Item = &mut IterationState> {
    self.states.iter_mut().rev().filter(|state| !state.is_variable())
  }

  ///
  fn iter_states_variable_mut(&mut self) -> impl Iterator<Item = &mut IterationState> {
    self.states.iter_mut().filter(|state| state.is_variable())
  }
}

///
pub struct ForExpressionEvaluator {
  iterator: FeelIterator,
  name_partial: Name,
}

impl ForExpressionEvaluator {
  ///
  pub fn new() -> Self {
    Self {
      iterator: FeelIterator::default(),
      name_partial: "partial".into(),
    }
  }

  ///
  pub fn add_list(&mut self, name: Name, value: Value) {
    self.iterator.add_list(name, value);
  }

  ///
  pub fn add_range(&mut self, name: Name, range_start: Value, range_end: Value) {
    if let Value::Number(start) = range_start {
      if let Value::Number(end) = range_end {
        if let Ok(i_start) = start.try_into() {
          if let Ok(i_end) = end.try_into() {
            self.iterator.add_range(name, i_start, i_end);
          }
        }
      }
    }
  }

  ///
  pub fn add_variable(&mut self, name: Name, variable: Name) {
    self.iterator.add_variable(name, variable);
  }

  ///
  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Values {
    let mut results = vec![];
    self.iterator.iterate(|ctx| {
      let mut iteration_context = ctx.clone();
      iteration_context.set_entry(&self.name_partial, Value::List(results.clone()));
      scope.push(iteration_context.clone());
      let iteration_value = evaluator(scope);
      scope.pop();
      results.push(iteration_value);
    });
    results
  }
}

///
pub struct SomeExpressionEvaluator {
  iterator: FeelIterator,
}

impl SomeExpressionEvaluator {
  ///
  pub fn new() -> Self {
    Self {
      iterator: FeelIterator::default(),
    }
  }

  ///
  pub fn add_list(&mut self, name: Name, value: Value) {
    self.iterator.add_list(name, value);
  }

  ///
  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Value {
    let mut result = VALUE_FALSE;
    let mut skip = false;
    self.iterator.iterate(|ctx| {
      if !skip {
        scope.push(ctx.clone());
        let value = evaluator(scope);
        match value {
          Value::Boolean(true) => {
            result = VALUE_TRUE;
            skip = true;
          }
          Value::Boolean(false) => {}
          _ => {
            result = value_null!();
            skip = true;
          }
        }
        scope.pop();
      }
    });
    result
  }
}

///
pub struct EveryExpressionEvaluator {
  iterator: FeelIterator,
}

impl EveryExpressionEvaluator {
  ///
  pub fn new() -> Self {
    Self { iterator: FeelIterator::new() }
  }

  ///
  pub fn add_list(&mut self, name: Name, value: Value) {
    self.iterator.add_list(name, value);
  }

  ///
  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Value {
    let mut result = VALUE_TRUE;
    let mut skip = false;
    self.iterator.iterate(|ctx| {
      if !skip {
        scope.push(ctx.clone());
        let value = evaluator(scope);
        scope.pop();
        match value {
          Value::Boolean(false) => {
            result = VALUE_FALSE;
            skip = true;
          }
          Value::Boolean(true) => {}
          _ => {
            result = value_null!();
            skip = true
          }
        }
      }
    });
    result
  }
}
