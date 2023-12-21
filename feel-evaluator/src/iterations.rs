//! ???

use dsntk_feel::context::FeelContext;
use dsntk_feel::values::{Value, Values};
use dsntk_feel::{Evaluator, FeelScope, Name};
use std::cmp::Ordering;

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
  step: isize,
  /// Iteration start value.
  start: isize,
  /// Iteration end value.
  end: isize,
  /// Collection of FEEL values to iterate over.
  values: Option<Values>,
}

impl IterationState {
  /// Creates a new iteration state for a range.
  fn new_range(variable: Name, start: isize, end: isize) -> Self {
    Self {
      iterator_type: IteratorType::Range,
      variable,
      index: start,
      step: if start <= end { 1 } else { -1 },
      start,
      end,
      values: None,
    }
  }

  /// Creates a new iteration state for a list of values.
  fn new_list(variable: Name, values: Values) -> Self {
    Self {
      iterator_type: IteratorType::List,
      variable,
      index: 0,
      step: 1,
      start: 0,
      end: (values.len() as isize) - 1,
      values: Some(values),
    }
  }

  /// Creates a new iteration state for bound variable.
  fn new_variable(variable: Name, bound_variable: Name) -> Self {
    Self {
      iterator_type: IteratorType::Variable(bound_variable),
      variable,
      index: 0,
      step: 1,
      start: 0,
      end: 0,
      values: None,
    }
  }

  /// If the iterator has more values, then move to next one.
  fn next(&mut self) -> bool {
    match self.step.cmp(&0) {
      Ordering::Greater => {
        if self.index + self.step <= self.end {
          self.index += self.step;
          true
        } else {
          self.index = self.start;
          false
        }
      }
      Ordering::Less => {
        if self.index + self.step >= self.end {
          self.index += self.step;
          true
        } else {
          self.index = self.start;
          false
        }
      }
      Ordering::Equal => panic!("iteration step must not be zero"),
    }
  }

  /// Returns `true` when the iterator has more values to iterate over.
  fn has_next(&self) -> bool {
    match self.step.cmp(&0) {
      Ordering::Greater => self.index + self.step <= self.end,
      Ordering::Less => self.index + self.step >= self.end,
      Ordering::Equal => panic!("iteration step must not be zero"),
    }
  }

  ///
  fn get_value(&self) -> Option<Value> {
    match self.iterator_type {
      IteratorType::Range => return Some(Value::Number(self.index.into())),
      IteratorType::List => {
        if let Some(values) = &self.values {
          let index = self.index as usize;
          if let Some(value) = values.get(index) {
            return Some(value.clone());
          }
        }
      }
      _ => {}
    }
    None
  }
}

/// Iterator built from multiple iteration states.
#[derive(Default)]
pub(crate) struct FeelIterator {
  states: Vec<IterationState>,
}

impl FeelIterator {
  ///
  pub fn add_range(&mut self, variable: Name, start: isize, end: isize) {
    self.states.push(IterationState::new_range(variable, start, end));
  }

  ///
  pub fn add_list(&mut self, variable: Name, values: Values) {
    self.states.push(IterationState::new_list(variable, values));
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
    self.states.reverse();
    loop {
      let mut empty_iteration = true;
      for state in &mut self.states {
        if let Some(value) = state.get_value() {
          ctx.set_entry(&state.variable, value);
          empty_iteration = false;
        }
      }
      if !empty_iteration {
        handler(&ctx);
      }
      let last_state = self.states.len() - 1;
      for (i, state) in self.states.iter_mut().enumerate() {
        if i == last_state && !state.has_next() {
          return; // the last value of the outermost iterator is reached, iterating is completed
        }
        if state.next() {
          break; // current iterator has more values to process, do not continue with other iterators
        }
      }
    }
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
    let values = match value {
      Value::List(values) => values,
      other => vec![other],
    };
    self.iterator.add_list(name, values);
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
    let values = match value {
      Value::List(values) => values,
      other => vec![other],
    };
    self.iterator.add_list(name, values);
  }

  ///
  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Value {
    let mut result = false;
    self.iterator.iterate(|ctx| {
      scope.push(ctx.clone());
      if let Value::Boolean(value) = evaluator(scope) {
        result = result || value;
      }
      scope.pop();
    });
    Value::Boolean(result)
  }
}

///
pub struct EveryExpressionEvaluator {
  iterator: FeelIterator,
}

impl EveryExpressionEvaluator {
  ///
  pub fn new() -> Self {
    Self {
      iterator: FeelIterator::default(),
    }
  }

  ///
  pub fn add_list(&mut self, name: Name, value: Value) {
    let values = match value {
      Value::List(values) => values,
      other => vec![other],
    };
    self.iterator.add_list(name, values);
  }

  ///
  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Value {
    let mut result = true;
    self.iterator.iterate(|ctx| {
      scope.push(ctx.clone());
      if let Value::Boolean(value) = evaluator(scope) {
        result = result && value;
      }
      scope.pop();
    });
    Value::Boolean(result)
  }
}
