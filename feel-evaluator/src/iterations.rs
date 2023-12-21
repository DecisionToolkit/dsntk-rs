//! ???

use dsntk_feel::context::FeelContext;
use dsntk_feel::values::{Value, Values};
use dsntk_feel::{Evaluator, FeelScope, Name};
use std::cmp::Ordering;

/// Iteration types.
#[derive(Debug, Clone)]
enum IteratorType {
  /// Iterator over a range of values.
  Range,
  /// Iterator over a list of values.
  List,
  /// Iterator over values in binding variable.
  Variable(Name),
}

/// Iteration state properties.
struct IteratorState {
  /// Enumeration indicating if the iteration is over the range of values or a list of values.
  iteration_type: IteratorType,
  /// Name of the variable used in iteration state.
  variable: Name,
  /// Current value of the looping index.
  index: isize,
  /// Iteration step.
  step: isize,
  /// Iteration start value.
  start: isize,
  /// Iteration end value.
  end: isize,
  /// Collection of FEEL values to iterate over (if not a range).
  values: Option<Values>,
}

/// Single iterator.
#[derive(Default)]
pub(crate) struct FeelIterator {
  iteration_states: Vec<IteratorState>,
}

impl FeelIterator {
  ///
  pub fn add_range(&mut self, variable: Name, start: isize, end: isize) {
    self.iteration_states.push(IteratorState {
      iteration_type: IteratorType::Range,
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
    self.iteration_states.push(IteratorState {
      iteration_type: IteratorType::List,
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
    self.iteration_states.push(IteratorState {
      iteration_type: IteratorType::Variable(binding),
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
    if self.iteration_states.is_empty() {
      return;
    }
    //self.sort_iteration_states();
    self.iteration_states.reverse();
    let mut iteration_context = FeelContext::new();
    'outer: loop {
      let mut is_empty_iteration = true;
      for iteration_state in &mut self.iteration_states {
        match &iteration_state.iteration_type {
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
          IteratorType::Variable(binding) => {
            iteration_state.end = 0;
            match iteration_context.get(binding) {
              Some(Value::List(values)) => {
                iteration_state.end = (values.len() as isize);
                let index = iteration_state.index as usize;
                if let Some(value) = values.get(index) {
                  iteration_context.set_entry(&iteration_state.variable, value.clone());
                  is_empty_iteration = false;
                }
              }
              Some(value) => {
                iteration_context.set_entry(&iteration_state.variable, value.clone());
                is_empty_iteration = false;
                iteration_state.end = 1;
              }
              _ => {}
            }
            // if let Some(value) = iteration_context.get(binding).cloned() {
            //   println!("DDD: {} => {}", iteration_state.variable, binding);
            //   println!("DDD: value = {}", value);
            //
            // }
            // if let Some(values) = iteration_context.get(variable) {
            //   let index = iteration_state.index as usize;
            //   if let Some(value) = values.get(index) {
            //     iteration_context.set_entry(&iteration_state.variable, value.clone());
            //     is_empty_iteration = false;
            //   }
            // }
          }
        }
      }
      if !is_empty_iteration {
        handler(&iteration_context);
      }
      let last_iteration_state_index = self.iteration_states.len() - 1;
      let mut overflow = true;
      'inner: for (x, iteration_state) in self.iteration_states.iter_mut().enumerate() {
        if overflow {
          if x == last_iteration_state_index {
            if iteration_state.step > 0 && iteration_state.index + iteration_state.step > iteration_state.end {
              break 'outer;
            }
            if iteration_state.step < 0 && iteration_state.index + iteration_state.step < iteration_state.end {
              break 'outer;
            }
          }
          if iteration_state.step > 0 {
            if iteration_state.index + iteration_state.step <= iteration_state.end {
              iteration_state.index += iteration_state.step;
              overflow = false;
            } else {
              iteration_state.index = iteration_state.start;
              overflow = true;
            }
          }
          if iteration_state.step < 0 {
            if iteration_state.index + iteration_state.step >= iteration_state.end {
              iteration_state.index += iteration_state.step;
              overflow = false;
            } else {
              iteration_state.index = iteration_state.start;
              overflow = true;
            }
          }
          if iteration_state.step == 0 {
            break 'outer;
          }
        } else {
          break 'inner;
        }
      }
    }
  }

  /// Sorts the iteration states this way, that states pointing
  /// to another binding variables are shifted to the end of the list.
  fn sort_iteration_states(&mut self) {
    self.iteration_states.reverse();
    self.iteration_states.sort_by(|a, b| {
      let flag_a = matches!(a.iteration_type, IteratorType::Variable(_));
      let flag_b = matches!(b.iteration_type, IteratorType::Variable(_));
      match (flag_a, flag_b) {
        (false, false) => Ordering::Equal,
        (false, true) => Ordering::Less,
        (true, false) => Ordering::Greater,
        (true, true) => Ordering::Equal,
      }
    });
  }
}

///
pub struct ForExpressionEvaluator {
  feel_iterator: FeelIterator,
  name_partial: Name,
}

impl ForExpressionEvaluator {
  ///
  pub fn new() -> Self {
    Self {
      feel_iterator: FeelIterator::default(),
      name_partial: "partial".into(),
    }
  }

  ///
  pub fn add_list(&mut self, name: Name, value: Value) {
    let values = match value {
      Value::List(values) => values,
      other => vec![other],
    };
    self.feel_iterator.add_list(name, values);
  }

  ///
  pub fn add_range(&mut self, name: Name, range_start: Value, range_end: Value) {
    if let Value::Number(start) = range_start {
      if let Value::Number(end) = range_end {
        if let Ok(i_start) = start.try_into() {
          if let Ok(i_end) = end.try_into() {
            self.feel_iterator.add_range(name, i_start, i_end);
          }
        }
      }
    }
  }

  ///
  pub fn add_variable(&mut self, name: Name, variable: Name) {
    self.feel_iterator.add_variable(name, variable);
  }

  ///
  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Values {
    let mut results = vec![];
    self.feel_iterator.iterate(|ctx| {
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
  feel_iterator: FeelIterator,
}

impl SomeExpressionEvaluator {
  ///
  pub fn new() -> Self {
    Self {
      feel_iterator: FeelIterator::default(),
    }
  }

  ///
  pub fn add_list(&mut self, name: Name, value: Value) {
    let values = match value {
      Value::List(values) => values,
      other => vec![other],
    };
    self.feel_iterator.add_list(name, values);
  }

  ///
  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Value {
    let mut result = false;
    self.feel_iterator.iterate(|ctx| {
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
  feel_iterator: FeelIterator,
}

impl EveryExpressionEvaluator {
  ///
  pub fn new() -> Self {
    Self {
      feel_iterator: FeelIterator::default(),
    }
  }

  ///
  pub fn add_list(&mut self, name: Name, value: Value) {
    let values = match value {
      Value::List(values) => values,
      other => vec![other],
    };
    self.feel_iterator.add_list(name, values);
  }

  ///
  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Value {
    let mut result = true;
    self.feel_iterator.iterate(|ctx| {
      scope.push(ctx.clone());
      if let Value::Boolean(value) = evaluator(scope) {
        result = result && value;
      }
      scope.pop();
    });
    Value::Boolean(result)
  }
}
