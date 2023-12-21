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
  ///
  fn next(&mut self) -> bool {
    match self.step.cmp(&0) {
      Ordering::Greater => {
        if self.index + self.step <= self.end {
          self.index += self.step;
          false
        } else {
          self.index = self.start;
          true
        }
      }
      Ordering::Less => {
        if self.index + self.step >= self.end {
          self.index += self.step;
          false
        } else {
          self.index = self.start;
          true
        }
      }
      Ordering::Equal => panic!("iteration step must not be zero"),
    }
  }

  ///
  fn has_next(&self) -> bool {
    match self.step.cmp(&0) {
      Ordering::Greater => self.index + self.step <= self.end,
      Ordering::Less => self.index + self.step >= self.end,
      Ordering::Equal => panic!("iteration step must not be zero"),
    }
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
    self.states.push(IterationState {
      iterator_type: IteratorType::Range,
      variable,
      index: start,
      step: if start <= end { 1 } else { -1 },
      start,
      end,
      values: None,
    });
  }

  ///
  pub fn add_list(&mut self, variable: Name, values: Values) {
    self.states.push(IterationState {
      iterator_type: IteratorType::List,
      variable,
      index: 0,
      step: 1,
      start: 0,
      end: (values.len() as isize) - 1,
      values: Some(values),
    });
  }

  ///
  pub fn add_variable(&mut self, variable: Name, binding: Name) {
    self.states.push(IterationState {
      iterator_type: IteratorType::Variable(binding),
      variable,
      index: 0,
      step: 1,
      start: 0,
      end: 0,
      values: None,
    });
  }

  /// Iterates over all iteration states.
  pub fn iterate<F>(&mut self, mut handler: F)
  where
    F: FnMut(&FeelContext),
  {
    if self.states.is_empty() {
      return;
    }
    let mut iteration_context = FeelContext::new();
    self.states.reverse();
    loop {
      let mut is_empty_iteration = true;
      for iteration_state in &mut self.states {
        match &iteration_state.iterator_type {
          IteratorType::Range => {
            let value = Value::Number(iteration_state.index.into());
            iteration_context.set_entry(&iteration_state.variable, value);
            is_empty_iteration = false;
          }
          IteratorType::List => {
            if let Some(values) = &iteration_state.values {
              let index = iteration_state.index as usize;
              if let Some(value) = values.get(index) {
                iteration_context.set_entry(&iteration_state.variable, value.clone());
                is_empty_iteration = false;
              }
            }
          }
          _ => {}
        }
      }
      if !is_empty_iteration {
        handler(&iteration_context);
      }
      let last_state = self.states.len() - 1;
      for (i, iteration_state) in self.states.iter_mut().enumerate() {
        if i == last_state && !iteration_state.has_next() {
          return;
        }
        if !iteration_state.next() {
          break;
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
