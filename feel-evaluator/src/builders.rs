//! # Evaluator builders for FEEL expressions

use crate::bifs;
use crate::evaluator_java::evaluate_external_java_function;
use crate::evaluator_pmml::evaluate_external_pmml_function;
use crate::iterations::{EveryExpressionEvaluator, ForExpressionEvaluator, SomeExpressionEvaluator};
use crate::macros::invalid_argument_type;
use dsntk_feel::bif::Bif;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::{Value, Values, VALUE_FALSE, VALUE_TRUE};
use dsntk_feel::{value_null, Evaluator, FeelNumber, FeelScope, FeelType, FunctionBody, Name, QualifiedName};
use dsntk_feel_parser::{AstNode, ClosureBuilder};
use dsntk_feel_temporal::{FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelTime, FeelYearsAndMonthsDuration};
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashSet};
use std::convert::TryFrom;
use std::file;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

/// Build context.
#[derive(Default)]
pub struct BuildContext {
  // Currently not used, but left here for future extensions.
}

///
pub fn build_evaluator(bx: &BuildContext, node: &AstNode) -> Evaluator {
  match node {
    AstNode::Add(lhs, rhs) => build_add(bx, lhs, rhs),
    AstNode::And(lhs, rhs) => build_and(bx, lhs, rhs),
    AstNode::At(rhs) => build_at(bx, rhs),
    AstNode::Between(lhs, mhs, rhs) => build_between(bx, lhs, mhs, rhs),
    AstNode::Boolean(lhs) => build_boolean(bx, *lhs),
    AstNode::Context(lhs) => build_context(bx, lhs),
    AstNode::ContextEntry(lhs, rhs) => build_context_entry(bx, lhs, rhs),
    AstNode::ContextEntryKey(lhs) => build_context_entry_key(bx, lhs),
    AstNode::ContextType(lhs) => build_context_type(bx, lhs),
    AstNode::ContextTypeEntry(lhs, rhs) => build_context_type_entry(bx, lhs, rhs),
    AstNode::ContextTypeEntryKey(lhs) => build_context_type_entry_key(bx, lhs),
    AstNode::Div(lhs, rhs) => build_div(bx, lhs, rhs),
    AstNode::Eq(lhs, rhs) => build_eq(bx, lhs, rhs),
    AstNode::EvaluatedExpression(lhs) => build_evaluated_expression(bx, lhs),
    AstNode::Every(lhs, rhs) => build_every(bx, lhs, rhs),
    AstNode::Exp(lhs, rhs) => build_exp(bx, lhs, rhs),
    AstNode::ExpressionList(lhs) => build_expression_list(bx, lhs),
    AstNode::FeelType(lhs) => build_feel_type(bx, lhs),
    AstNode::Filter(lhs, rhs) => build_filter(bx, lhs, rhs),
    AstNode::For(lhs, rhs) => build_for(bx, lhs, rhs),
    AstNode::FormalParameter(lhs, rhs) => build_formal_parameter(bx, lhs, rhs),
    AstNode::FormalParameters(lhs) => build_formal_parameters(bx, lhs),
    AstNode::FunctionBody(lhs, rhs) => build_function_body(bx, lhs, rhs),
    AstNode::FunctionDefinition(lhs, rhs) => build_function_definition(bx, lhs, rhs),
    AstNode::FunctionInvocation(lhs, rhs) => build_function_invocation(bx, lhs, rhs),
    AstNode::FunctionType(lhs, rhs) => build_function_type(bx, lhs, rhs),
    AstNode::Ge(lhs, rhs) => build_ge(bx, lhs, rhs),
    AstNode::Gt(lhs, rhs) => build_gt(bx, lhs, rhs),
    AstNode::If(lhs, mid, rhs) => build_if(bx, lhs, mid, rhs),
    AstNode::In(lhs, rhs) => build_in(bx, lhs, rhs),
    AstNode::InstanceOf(lhs, rhs) => build_instance_of(bx, lhs, rhs),
    AstNode::IntervalEnd(lhs, rhs) => build_interval_end(bx, lhs, rhs),
    AstNode::IntervalStart(lhs, rhs) => build_interval_start(bx, lhs, rhs),
    AstNode::Irrelevant => build_irrelevant(bx),
    AstNode::Le(lhs, rhs) => build_le(bx, lhs, rhs),
    AstNode::List(items) => build_list(bx, items),
    AstNode::ListType(lhs) => build_list_type(bx, lhs),
    AstNode::Lt(lhs, rhs) => build_lt(bx, lhs, rhs),
    AstNode::Mul(lhs, rhs) => build_mul(bx, lhs, rhs),
    AstNode::Name(name) => build_name(bx, name.clone()),
    AstNode::NamedParameter(lhs, rhs) => build_named_parameter(bx, lhs, rhs),
    AstNode::NamedParameters(lhs) => build_named_parameters(bx, lhs),
    AstNode::Neg(rhs) => build_neg(bx, rhs),
    AstNode::NegatedList(lhs) => build_negated_list(bx, lhs),
    AstNode::Null => build_null(bx),
    AstNode::Numeric(lhs, rhs) => build_numeric(bx, lhs, rhs),
    AstNode::Nq(lhs, rhs) => build_nq(bx, lhs, rhs),
    AstNode::Or(lhs, rhs) => build_or(bx, lhs, rhs),
    AstNode::Out(lhs, rhs) => build_out(bx, lhs, rhs),
    AstNode::ParameterName(lhs) => build_parameter_name(bx, lhs),
    AstNode::ParameterTypes(lhs) => build_parameter_types(bx, lhs),
    AstNode::Path(lhs, rhs) => build_path(bx, lhs, rhs),
    AstNode::QualifiedName(lhs) => build_qualified_name(bx, lhs),
    AstNode::QualifiedNameSegment(lhs) => build_qualified_name_segment(bx, lhs),
    AstNode::Range(lhs, rhs) => build_range(bx, lhs, rhs),
    AstNode::RangeType(lhs) => build_range_type(bx, lhs),
    AstNode::Some(lhs, rhs) => build_some(bx, lhs, rhs),
    AstNode::String(lhs) => build_string(bx, lhs),
    AstNode::Sub(lhs, rhs) => build_sub(bx, lhs, rhs),
    AstNode::UnaryGe(lhs) => build_unary_ge(bx, lhs),
    AstNode::UnaryGt(lhs) => build_unary_gt(bx, lhs),
    AstNode::UnaryLe(lhs) => build_unary_le(bx, lhs),
    AstNode::UnaryLt(lhs) => build_unary_lt(bx, lhs),
    AstNode::CommaList { .. }
    | AstNode::IterationContexts { .. }
    | AstNode::IterationContextList { .. }
    | AstNode::IterationContextRange { .. }
    | AstNode::PositionalParameters { .. }
    | AstNode::QuantifiedContext { .. }
    | AstNode::QuantifiedContexts { .. }
    | AstNode::Satisfies { .. } => build_err_msg(err_msg_unexpected_node(node)),
  }
}

///
fn build_add(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Number(lh + rh),
        value @ Value::Null(_) => value,
        _ => value_null!("incompatible types in addition: {}({}) + {}({})", lhv, lhv.type_of(), rhv, rhv.type_of()),
      },
      Value::String(mut lh) => {
        if let Value::String(rh) = rhv {
          lh.push_str(&rh);
          Value::String(lh)
        } else {
          value_null!("expected string as a second argument in addition")
        }
      }
      Value::Date(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => {
          if let Some(result) = lh + rh {
            Value::Date(result)
          } else {
            value_null!("invalid result while adding days and time duration to date")
          }
        }
        Value::YearsAndMonthsDuration(rh) => {
          if let Some(a) = lh + rh {
            Value::Date(a)
          } else {
            value_null!("invalid result while adding years and months duration to date")
          }
        }
        other => invalid_argument_type!("add", "years and months duration", other.type_of()),
      },
      Value::DateTime(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => {
          if let Some(a) = lh + rh {
            Value::DateTime(a)
          } else {
            value_null!("invalid result while adding days and time duration to date and time")
          }
        }
        Value::YearsAndMonthsDuration(rh) => {
          if let Some(a) = lh + rh {
            Value::DateTime(a)
          } else {
            value_null!("invalid result while adding years and months duration to date and time")
          }
        }
        other => invalid_argument_type!("add", "days and time duration, years and months duration", other.type_of()),
      },
      Value::Time(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::Time(lh + rh),
        other => invalid_argument_type!("add", "days and time duration", other.type_of()),
      },
      Value::DaysAndTimeDuration(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::DaysAndTimeDuration(lh + rh),
        Value::Date(rh) => {
          if let Some(result) = rh + lh {
            Value::Date(result)
          } else {
            value_null!("invalid result while adding date to days and time duration")
          }
        }
        Value::DateTime(rh) => {
          if let Some(a) = rh + lh {
            Value::DateTime(a)
          } else {
            value_null!("invalid result while adding date and time to days and time duration")
          }
        }
        Value::Time(rh) => Value::Time(rh + lh),
        other => invalid_argument_type!("add", "days and time duration, date and time", other.type_of()),
      },
      Value::YearsAndMonthsDuration(lh) => match rhv {
        Value::Date(rh) => {
          if let Some(a) = rh + lh {
            Value::Date(a)
          } else {
            value_null!("invalid result while adding date to years and months duration")
          }
        }
        Value::DateTime(rh) => {
          if let Some(a) = rh + lh {
            Value::DateTime(a)
          } else {
            value_null!("invalid result while adding date and time to years and months duration")
          }
        }
        Value::YearsAndMonthsDuration(rh) => Value::YearsAndMonthsDuration(lh + rh),
        other => invalid_argument_type!("add", "years and months duration, date and time", other.type_of()),
      },
      value @ Value::Null(_) => value,
      other => invalid_argument_type!(
        "add",
        "number, string, date and time, days and time duration, years and months duration, null",
        other.type_of()
      ),
    }
  })
}

/// Builds evaluator of temporal expression after `@` (at) literal.
fn build_at(_bx: &BuildContext, text: &str) -> Evaluator {
  if let Ok(date) = FeelDate::from_str(text) {
    Box::new(move |_: &FeelScope| Value::Date(date.clone()))
  } else if let Ok(date_time) = FeelDateTime::try_from(text) {
    Box::new(move |_: &FeelScope| Value::DateTime(date_time.clone()))
  } else if let Ok(time) = text.parse::<FeelTime>() {
    Box::new(move |_: &FeelScope| Value::Time(time.clone()))
  } else if let Ok(ym_duration) = FeelYearsAndMonthsDuration::try_from(text) {
    Box::new(move |_: &FeelScope| Value::YearsAndMonthsDuration(ym_duration.clone()))
  } else if let Ok(dt_duration) = FeelDaysAndTimeDuration::try_from(text) {
    Box::new(move |_: &FeelScope| Value::DaysAndTimeDuration(dt_duration.clone()))
  } else {
    let invalid_at_literal = format!("invalid @ literal: {}", text);
    Box::new(move |_: &FeelScope| value_null!(invalid_at_literal))
  }
}

/// Prepares null value with error message for second argument in `between` operator.
macro_rules! between_null2 {
  ($expected:literal, $actual:expr) => {
    Value::Null(Some(format!(
      "expected {} as a second argument in 'between' operator, actual value type is {}",
      $expected, $actual
    )))
  };
}

/// Prepares null value with error message for third argument in `between` operator.
macro_rules! between_null3 {
  ($expected:literal, $actual:expr) => {
    Value::Null(Some(format!(
      "expected {} as a third argument in 'between' operator, actual value type is {}",
      $expected, $actual
    )))
  };
}

///
fn build_between(bx: &BuildContext, lhs: &AstNode, mhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let mhe = build_evaluator(bx, mhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let mhv = mhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => {
        if let Value::Number(mh) = mhv {
          if let Value::Number(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("number", rhv.type_of())
          }
        } else {
          between_null2!("number", mhv.type_of())
        }
      }
      Value::String(lh) => {
        if let Value::String(mh) = mhv {
          if let Value::String(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("string", rhv.type_of())
          }
        } else {
          between_null2!("string", mhv.type_of())
        }
      }
      Value::Date(lh) => {
        if let Value::Date(mh) = mhv {
          if let Value::Date(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("date", rhv.type_of())
          }
        } else {
          between_null2!("date", mhv.type_of())
        }
      }
      Value::Time(lh) => {
        if let Value::Time(mh) = mhv {
          if let Value::Time(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("time", rhv.type_of())
          }
        } else {
          between_null2!("time", mhv.type_of())
        }
      }
      Value::DateTime(lh) => {
        if let Value::DateTime(mh) = mhv {
          if let Value::DateTime(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("date and time", rhv.type_of())
          }
        } else {
          between_null2!("date and time", mhv.type_of())
        }
      }
      Value::DaysAndTimeDuration(lh) => {
        if let Value::DaysAndTimeDuration(mh) = mhv {
          if let Value::DaysAndTimeDuration(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("days and time duration", rhv.type_of())
          }
        } else {
          between_null2!("days and time duration", mhv.type_of())
        }
      }
      Value::YearsAndMonthsDuration(lh) => {
        if let Value::YearsAndMonthsDuration(mh) = mhv {
          if let Value::YearsAndMonthsDuration(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("years and months duration", rhv.type_of())
          }
        } else {
          between_null2!("years and months duration", mhv.type_of())
        }
      }
      other => {
        if other.is_null() {
          other
        } else {
          value_null!("unexpected value type in 'between' operator: {}", other.type_of())
        }
      }
    }
  })
}

///
fn build_boolean(_bx: &BuildContext, lhs: bool) -> Evaluator {
  Box::new(move |_: &FeelScope| Value::Boolean(lhs))
}

/// Semantics of conjunction.
/// ```text
/// left        right        result
///─────────────────────────────────
/// true        true         true
/// true        false        false
/// true        otherwise    null
/// false       true         false
/// false       false        false
/// false       otherwise    false
/// otherwise   true         null
/// otherwise   false        false
/// otherwise   otherwise    null
/// ```
fn build_and(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Boolean(lh) => match rhv {
        Value::Boolean(rh) => Value::Boolean(lh && rh),
        _ => {
          if lh {
            value_null!()
          } else {
            Value::Boolean(false)
          }
        }
      },
      _ => match rhv {
        Value::Boolean(rh) => {
          if rh {
            value_null!()
          } else {
            Value::Boolean(false)
          }
        }
        _ => value_null!(),
      },
    }
  })
}

///
fn build_context(bx: &BuildContext, lhs: &[AstNode]) -> Evaluator {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(bx, node));
  }
  Box::new(move |scope: &FeelScope| {
    let mut evaluated_ctx = FeelContext::default();
    // prepare special context in scope, used for already evaluated context entries
    scope.push(FeelContext::default());
    // evaluate context entries
    for evaluator in &evaluators {
      match evaluator(scope) {
        Value::ContextEntry(name, value) => {
          if evaluated_ctx.contains_entry(&name) {
            // duplicated context entry keys are not allowed
            scope.pop();
            return value_null!("duplicated context entry key: {}", name);
          } else {
            // add newly evaluated entry to evaluated context
            evaluated_ctx.set_entry(&name, (*value).clone());
            // add newly evaluated entry to special context
            scope.set_value(&name, *value);
          }
        }
        value_null @ Value::Null(_) => return value_null,
        other => return value_null!("expected context entry, actual value type is {}", other.type_of()),
      }
    }
    // remove special context from scope
    scope.pop();
    Value::Context(evaluated_ctx)
  })
}

///
fn build_context_entry(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::ContextEntryKey(name) => Value::ContextEntry(name, Box::new(rhv)),
      _ => value_null!("expected context entry key, actual value type is {}", lhv.type_of()),
    }
  })
}

///
fn build_context_entry_key(_bx: &BuildContext, lhs: &Name) -> Evaluator {
  let name = lhs.clone();
  Box::new(move |_: &FeelScope| Value::ContextEntryKey(name.clone()))
}

///
fn build_context_type(bx: &BuildContext, lhs: &[AstNode]) -> Evaluator {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(bx, node));
  }
  Box::new(move |scope: &FeelScope| {
    let mut entries = BTreeMap::new();
    for evaluator in &evaluators {
      match evaluator(scope) {
        Value::ContextTypeEntry(name, feel_type) => {
          entries.insert(name, feel_type.clone());
        }
        null @ Value::Null(_) => return null,
        other => return value_null!("expected context type entry, actual value type is {}", other.type_of()),
      }
    }
    Value::FeelType(FeelType::Context(entries))
  })
}

///
fn build_context_type_entry(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Value::ContextTypeEntryKey(name) = lhv {
      if let Value::FeelType(feel_type) = rhv {
        Value::ContextTypeEntry(name, feel_type)
      } else {
        value_null!("expected a type in context type entry")
      }
    } else {
      value_null!("expected a name in context type entry")
    }
  })
}

///
fn build_context_type_entry_key(_bx: &BuildContext, lhs: &Name) -> Evaluator {
  let name = lhs.clone();
  Box::new(move |_: &FeelScope| Value::ContextTypeEntryKey(name.clone()))
}

///
fn build_div(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => {
          if rh.abs() == FeelNumber::zero() {
            value_null!("[division] division by zero")
          } else {
            Value::Number(lh / rh)
          }
        }
        _ => value_null!("[division] incompatible types: {} / {}", lhv, rhv),
      },
      Value::DaysAndTimeDuration(ref lh) => match rhv {
        Value::Number(rh) => {
          if rh.is_zero() {
            value_null!("[division] division by zero")
          } else {
            let lv = FeelNumber::from(lh.as_nanos()) / rh;
            if let Ok(v) = FeelNumber::try_into(lv) {
              Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_n(v))
            } else {
              value_null!("[division] error: {} / {}", lhv, rhv)
            }
          }
        }
        Value::DaysAndTimeDuration(rh) => {
          if rh.as_nanos() == 0 {
            value_null!("[division] division by zero")
          } else {
            let lvl = FeelNumber::from(lh.as_nanos());
            let rvl = FeelNumber::from(rh.as_nanos());
            Value::Number(lvl / rvl)
          }
        }
        _ => value_null!("[division] incompatible types: {} / {}", lhv, rhv),
      },
      Value::YearsAndMonthsDuration(ref lh) => match rhv {
        Value::Number(rh) => {
          if rh.is_zero() {
            value_null!("[division] division by zero")
          } else {
            let vl = FeelNumber::from(lh.as_months()) / rh;
            if let Ok(v) = FeelNumber::try_into(vl) {
              Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_m(v))
            } else {
              value_null!("[division] error: {} / {}", lhv, rhv)
            }
          }
        }
        Value::YearsAndMonthsDuration(rh) => {
          if rh.as_months() == 0 {
            value_null!("[division] division by zero")
          } else {
            let lvl = FeelNumber::from(lh.as_months());
            let rvl = FeelNumber::from(rh.as_months());
            Value::Number(lvl / rvl)
          }
        }
        _ => value_null!("[division] incompatible types: {} / {}", lhv, rhv),
      },
      _ => value_null!("[division] incompatible types: {} / {}", lhv, rhv),
    }
  })
}

///
fn build_expression_list(bx: &BuildContext, lhs: &[AstNode]) -> Evaluator {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(bx, node));
  }
  Box::new(move |scope: &FeelScope| {
    let mut values = vec![];
    for evaluator in &evaluators {
      values.push(evaluator(scope))
    }
    Value::ExpressionList(values)
  })
}

///
fn build_exp(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Value::Number(lh) = lhv {
      if let Value::Number(rh) = rhv {
        if let Some(result) = lh.pow(&rh) {
          Value::Number(result)
        } else {
          value_null!("exponentiation result is not a finite number")
        }
      } else {
        value_null!("exponentiation exponent is not a number")
      }
    } else {
      value_null!("exponentiation base is not a number")
    }
  })
}

///
fn build_feel_type(_bx: &BuildContext, lhs: &FeelType) -> Evaluator {
  let feel_type = lhs.clone();
  Box::new(move |_: &FeelScope| Value::FeelType(feel_type.clone()))
}

///
fn build_filter(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  let name_item: Name = "item".into();
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    match lhv {
      Value::List(values) => {
        let mut filtered_values = vec![];
        for value in &values {
          let (added_local_context, has_item_entry) = if let Value::Context(local_context) = value {
            scope.push(local_context.clone());
            if local_context.contains_entry(&name_item) {
              (true, true)
            } else {
              (true, false)
            }
          } else {
            (false, false)
          };
          if !has_item_entry {
            let mut special_context = FeelContext::default();
            special_context.set_entry(&name_item, value.clone());
            scope.push(special_context);
          }
          let rhv = rhe(scope);
          if let Value::Boolean(true) = rhv {
            filtered_values.push(value.clone());
          }
          if !has_item_entry {
            scope.pop();
          }
          if added_local_context {
            scope.pop();
          }
        }
        let rhv = rhe(scope);
        match rhv {
          Value::Number(index) => {
            if index.is_integer() {
              let list_size = values.len();
              if list_size > 0 {
                if !index.is_negative() {
                  let n = {
                    if let Ok(u_index) = usize::try_from(index) {
                      u_index
                    } else {
                      return value_null!("index is out of range 1..2⁶⁴: {}", index.to_string());
                    }
                  };
                  if n > 0 && n <= list_size {
                    // unwrap below is safe, index `n` is checked above, `values` variable is immutable
                    values.get(n - 1).unwrap().to_owned()
                  } else {
                    value_null!("index in filter is out of range [1..{}], actual index is {}", list_size, n)
                  }
                } else {
                  let n = {
                    if let Ok(u_index) = usize::try_from(index.abs()) {
                      u_index
                    } else {
                      return value_null!("index is out of range 1..2⁶⁴: {}", index.to_string());
                    }
                  };
                  if n > 0 && n <= list_size {
                    // unwrap below is safe, index `n` is checked above, `values` variable is immutable
                    values.get(list_size - n).unwrap().to_owned()
                  } else {
                    value_null!("index in filter is out of range [-{}..-1], actual index is -{}", list_size, n)
                  }
                }
              } else {
                // return null when the list is empty, no matter what value the index has
                value_null!()
              }
            } else {
              value_null!("index in filter must be an integer value, actual value is {}", index)
            }
          }
          _ => Value::List(filtered_values),
        }
      }
      v @ Value::Number(_)
      | v @ Value::Boolean(_)
      | v @ Value::String(_)
      | v @ Value::Date(_)
      | v @ Value::DateTime(_)
      | v @ Value::Time(_)
      | v @ Value::DaysAndTimeDuration(_)
      | v @ Value::YearsAndMonthsDuration(_)
      | v @ Value::Context(_) => match rhe(scope) {
        Value::Boolean(flag) => {
          if flag {
            Value::List(vec![v])
          } else {
            Value::List(Values::default())
          }
        }
        Value::Number(num) => {
          if num.is_one() || (-num).is_one() {
            v
          } else {
            value_null!("for singletons, only filter index with value 1 or -1 is accepted")
          }
        }
        _ => value_null!("only number or boolean indexes are allowed in filters"),
      },
      other => value_null!("unexpected value type in filter: {}", other.type_of()),
    }
  })
}

///
fn build_for(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  enum IterationContextType {
    Range,
    List,
    Variable,
  }
  let rhe = build_evaluator(bx, rhs);
  let mut evaluators_range = vec![];
  let mut evaluators_list = vec![];
  let mut evaluators_variable = vec![];
  let mut evaluators_order = vec![];
  let mut binding_variables = HashSet::new();
  if let AstNode::IterationContexts(items) = lhs {
    for item in items {
      match item {
        AstNode::IterationContextRange(variable_name, range_start_node, range_end_node) => {
          if let AstNode::Name(name) = variable_name.borrow() {
            let evaluator_range_start = build_evaluator(bx, range_start_node);
            let evaluator_range_end = build_evaluator(bx, range_end_node);
            evaluators_range.push((name.clone(), evaluator_range_start, evaluator_range_end));
            evaluators_order.push((IterationContextType::Range, evaluators_range.len() - 1));
            binding_variables.insert(name.clone());
          }
        }
        AstNode::IterationContextList(variable_name, expr_node) => {
          if let AstNode::Name(name) = variable_name.borrow() {
            if let AstNode::Name(variable_name) = expr_node.borrow() {
              if binding_variables.contains(variable_name) {
                evaluators_variable.push((name.clone(), variable_name.clone()));
                evaluators_order.push((IterationContextType::Variable, evaluators_variable.len() - 1));
                binding_variables.insert(name.clone());
                continue;
              }
            }
            let evaluator_list = build_evaluator(bx, expr_node);
            evaluators_list.push((name.clone(), evaluator_list));
            evaluators_order.push((IterationContextType::List, evaluators_list.len() - 1));
            binding_variables.insert(name.clone());
          }
        }
        _ => {}
      }
    }
  }
  Box::new(move |scope: &FeelScope| {
    let mut expression_evaluator = ForExpressionEvaluator::new();
    for (fit, index) in &evaluators_order {
      match fit {
        IterationContextType::Range => {
          let (name, evaluator_range_start, evaluator_range_end) = &evaluators_range[*index];
          expression_evaluator.add_range(name.clone(), evaluator_range_start(scope), evaluator_range_end(scope));
        }
        IterationContextType::List => {
          let (name, evaluator_single) = &evaluators_list[*index];
          expression_evaluator.add_list(name.clone(), evaluator_single(scope));
        }
        IterationContextType::Variable => {
          let (name, variable_name) = &evaluators_variable[*index];
          expression_evaluator.add_variable(name.clone(), variable_name.clone());
        }
      }
    }
    Value::List(expression_evaluator.evaluate(scope, &rhe))
  })
}

///
fn build_formal_parameter(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Value::ParameterName(parameter_name) = lhv {
      if let Value::FeelType(parameter_type) = rhv {
        Value::FormalParameter(parameter_name, parameter_type)
      } else {
        value_null!("expected type of the formal parameter")
      }
    } else {
      value_null!("expected name of the formal parameter")
    }
  })
}

///
fn build_formal_parameters(bx: &BuildContext, lhs: &[AstNode]) -> Evaluator {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(bx, node));
  }
  Box::new(move |scope: &FeelScope| {
    let mut formal_parameters = vec![];
    for evaluator in &evaluators {
      match evaluator(scope) {
        Value::FormalParameter(parameter_name, parameter_type) => {
          formal_parameters.push((parameter_name, parameter_type));
        }
        null @ Value::Null(_) => return null,
        other => return value_null!("expected formal parameter, actual value type is: {}", other.type_of()),
      }
    }
    Value::FormalParameters(formal_parameters)
  })
}

///
fn build_function_body(bx: &BuildContext, lhs: &AstNode, rhs: &bool) -> Evaluator {
  if *rhs {
    build_external_function_body(bx, lhs)
  } else {
    build_internal_function_body(bx, lhs)
  }
}

///
fn build_internal_function_body(bx: &BuildContext, lhs: &AstNode) -> Evaluator {
  let lhe = Arc::new(build_evaluator(bx, lhs));
  Box::new(move |_: &FeelScope| Value::FunctionBody(FunctionBody::LiteralExpression(lhe.clone()), false))
}

///
fn build_external_function_body(bx: &BuildContext, lhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let mapping_value = lhe(scope);
    match mapping_value {
      Value::Context(mapping_information) => {
        if let Some(Value::Context(java_mapping)) = mapping_information.get_entry(&"java".into()) {
          return if let Some(Value::String(class_name)) = java_mapping.get_entry(&"class".into()) {
            if let Some(Value::String(method_signature)) = java_mapping.get_entry(&"method signature".into()) {
              let java_class_name = class_name.to_owned();
              let java_method_signature = method_signature.to_owned();
              let java_evaluator = Box::new(move |_: &FeelScope| Value::ExternalJavaFunction(java_class_name.clone(), java_method_signature.clone())) as Evaluator;
              let lhe = Arc::new(java_evaluator);
              Value::FunctionBody(FunctionBody::External(lhe), true)
            } else {
              value_null!("invalid Java function mapping, no method signature entry in context {}", java_mapping)
            }
          } else {
            value_null!("invalid Java function mapping, no class name entry in context {}", java_mapping)
          };
        }
        if let Some(Value::Context(pmml_mapping)) = mapping_information.get_entry(&"pmml".into()) {
          return if let Some(Value::String(document)) = pmml_mapping.get_entry(&"document".into()) {
            if let Some(Value::String(model_name)) = pmml_mapping.get_entry(&"model".into()) {
              let pmml_document = document.to_owned();
              let pmml_model_name = model_name.to_owned();
              let pmml_evaluator = Box::new(move |_: &FeelScope| Value::ExternalPmmlFunction(pmml_document.clone(), pmml_model_name.clone())) as Evaluator;
              let lhe = Arc::new(pmml_evaluator);
              Value::FunctionBody(FunctionBody::External(lhe), true)
            } else {
              value_null!("invalid PMML function mapping, no model name entry in context {}", pmml_mapping)
            }
          } else {
            value_null!("invalid PMML function mapping, no document entry in context {}", pmml_mapping)
          };
        }
        value_null!("invalid external function mapping, expected 'java' or 'pmml' entry in context {}", mapping_information)
      }
      other => value_null!("invalid external function mapping, expected context, actual value is {}", other),
    }
  })
}

///
fn build_function_definition(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let closure = ClosureBuilder::from_function_definition(lhs, rhs);
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::FormalParameters(parameters) => {
        if let Value::FunctionBody(body, external) = rhv {
          // evaluate closure context
          let mut closure_ctx = FeelContext::default();
          for closure_name in closure.iter() {
            if let Some(closure_value) = scope.search_entry(closure_name) {
              closure_ctx.create_entry(closure_name, closure_value);
            }
          }
          //TODO Check if `FeelType::Any` is always ok for function result type in function definition.
          Value::FunctionDefinition(parameters, body, external, closure.clone(), closure_ctx, FeelType::Any)
        } else {
          value_null!("invalid body in function definition")
        }
      }
      null @ Value::Null(_) => null,
      other => value_null!("expected formal parameters, actual value type is {}", other.type_of()),
    }
  })
}

///
fn build_eq(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Some(result) = eval_ternary_equality(&lhv, &rhv) {
      Value::Boolean(result)
    } else {
      value_null!("equal err '{}' =?= '{}'", lhv, rhv)
    }
  })
}

///
fn build_evaluated_expression(bx: &BuildContext, lhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| lhe(scope))
}

///
fn build_every(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let mut expr_evaluators = vec![];
  let AstNode::QuantifiedContexts(items) = lhs else {
    return build_err_msg(err_msg_expected_node("QuantifiedContexts", lhs));
  };
  for item in items {
    if let AstNode::QuantifiedContext(variable_name, expr_node) = item {
      if let AstNode::Name(name) = variable_name.borrow() {
        let evaluator_single = build_evaluator(bx, expr_node);
        expr_evaluators.push((name.clone(), evaluator_single));
      }
    }
  }
  let AstNode::Satisfies(satisfies) = rhs else {
    return build_err_msg(err_msg_expected_node("Satisfies", lhs));
  };
  let satisfies_evaluator = build_evaluator(bx, satisfies);
  Box::new(move |scope: &FeelScope| {
    let mut expression_evaluator = EveryExpressionEvaluator::new();
    for (name, expr_evaluator) in &expr_evaluators {
      expression_evaluator.add_list(name.clone(), expr_evaluator(scope));
    }
    expression_evaluator.evaluate(scope, &satisfies_evaluator)
  })
}

///
fn build_function_invocation(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  match rhs {
    AstNode::PositionalParameters(parameters) => build_function_invocation_with_positional_parameters(bx, lhs, parameters),
    node @ AstNode::NamedParameters(_) => build_function_invocation_with_named_parameters(bx, lhs, node),
    _ => build_err_msg(err_msg_expected_positional_or_named_parameter(rhs)),
  }
}

///
fn build_function_invocation_with_positional_parameters(bx: &BuildContext, lhs: &AstNode, rhs: &[AstNode]) -> Evaluator {
  let mut argument_evaluators = vec![];
  for node in rhs {
    argument_evaluators.push(build_evaluator(bx, node));
  }
  let function_evaluator = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let function = function_evaluator(scope);
    let args = argument_evaluators.iter().map(|evaluator| evaluator(scope)).collect::<Vec<Value>>();
    match function {
      Value::BuiltInFunction(bif) => bifs::positional::evaluate_bif(bif, &args),
      Value::FunctionDefinition(params, body, external, _, closure_ctx, result_type) => {
        if external {
          eval_external_function_with_positional_parameters(scope, &args, &params, &body, result_type)
        } else {
          eval_function_with_positional_parameters(scope, &args, &params, &body, closure_ctx, result_type)
        }
      }
      _ => value_null!("expected built-in function name or function definition, actual is {}", function),
    }
  })
}

///
fn build_function_invocation_with_named_parameters(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let function_evaluator = build_evaluator(bx, lhs);
  let arguments_evaluator = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let function = function_evaluator(scope);
    let args = arguments_evaluator(scope);
    match function {
      Value::BuiltInFunction(bif) => bifs::named::evaluate_bif(bif, &args),
      Value::FunctionDefinition(params, body, external, _, closure_ctx, result_type) => {
        if external {
          eval_external_function_with_named_parameters(scope, &args, &params, &body, result_type)
        } else {
          eval_function_with_named_parameters(scope, &args, &params, &body, closure_ctx, result_type)
        }
      }
      _ => value_null!("expected built-in function name or function definition, actual is {}", function),
    }
  })
}

///
fn build_function_type(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Value::ParameterTypes(types) = lhv {
      if let Value::FeelType(result_type) = rhv {
        let parameter_types = types
          .iter()
          .filter_map(|value| if let Value::FeelType(feel_type) = value { Some(feel_type.clone()) } else { None })
          .collect();
        Value::FeelType(FeelType::Function(parameter_types, Box::new(result_type)))
      } else {
        value_null!("expected function's result type")
      }
    } else {
      value_null!("expected function's parameter types")
    }
  })
}

///
fn build_ge(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_number"),
      },
      Value::String(lh) => match rhv {
        Value::String(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_string"),
      },
      Value::Date(lh) => match rhv {
        Value::Date(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_date"),
      },
      Value::DateTime(lh) => match rhv {
        Value::DateTime(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_date_time"),
      },
      Value::Time(lh) => match rhv {
        Value::Time(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_time"),
      },
      Value::DaysAndTimeDuration(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_days_and_time_duration"),
      },
      Value::YearsAndMonthsDuration(lh) => match rhv {
        Value::YearsAndMonthsDuration(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_years_and_months_duration"),
      },
      _ => value_null!("eval_greater_or_equal"),
    }
  })
}

///
fn build_gt(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_then_number"),
      },
      Value::String(lh) => match rhv {
        Value::String(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_then_string"),
      },
      Value::Date(lh) => match rhv {
        Value::Date(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_then_date"),
      },
      Value::DateTime(lh) => match rhv {
        Value::DateTime(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_then_date_time"),
      },
      Value::Time(lh) => match rhv {
        Value::Time(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_then_time"),
      },
      Value::DaysAndTimeDuration(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_days_and_time_duration"),
      },
      Value::YearsAndMonthsDuration(lh) => match rhv {
        Value::YearsAndMonthsDuration(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_years_and_months_duration"),
      },
      _ => value_null!("eval_greater_then"),
    }
  })
}

///
fn build_if(bx: &BuildContext, lhs: &AstNode, mhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let mhe = build_evaluator(bx, mhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| match lhe(scope) {
    Value::Boolean(true) => mhe(scope),
    Value::Boolean(false) | Value::Null(_) => rhe(scope),
    _ => value_null!("condition in 'if' expression is not a boolean value"),
  })
}

///
fn build_in(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match rhv {
      inner @ Value::Null(_)
      | inner @ Value::Number(_)
      | inner @ Value::String(_)
      | inner @ Value::Boolean(_)
      | inner @ Value::Date(_)
      | inner @ Value::Time(_)
      | inner @ Value::DateTime(_)
      | inner @ Value::YearsAndMonthsDuration(_)
      | inner @ Value::DaysAndTimeDuration(_)
      | inner @ Value::Context(_) => eval_in_equal(&lhv, &inner),
      Value::Range(l, l_closed, r, r_closed) => eval_in_range(&lhv, &l, l_closed, &r, r_closed),
      Value::List(r_inner) => {
        if let Value::List(l_inner) = lhv {
          eval_in_list_in_list(&l_inner, &r_inner)
        } else {
          eval_in_list(&lhv, &r_inner)
        }
      }
      Value::ExpressionList(inner) => eval_in_list(&lhv, &inner),
      Value::NegatedCommaList(inner) => eval_in_negated_list(&lhv, &inner),
      Value::UnaryLess(inner) => eval_in_unary_less(&lhv, inner.borrow()),
      Value::UnaryLessOrEqual(inner) => eval_in_unary_less_or_equal(&lhv, inner.borrow()),
      Value::UnaryGreater(inner) => eval_in_unary_greater(&lhv, inner.borrow()),
      Value::UnaryGreaterOrEqual(inner) => eval_in_unary_greater_or_equal(&lhv, inner.borrow()),
      Value::Irrelevant => VALUE_TRUE,
      _ => value_null!("unexpected argument type in 'in' operator: {}", rhv.type_of()),
    }
  })
}

///
fn build_interval_end(bx: &BuildContext, lhs: &AstNode, rhs: &bool) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let closed = *rhs;
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::IntervalEnd(Box::new(lhv), closed)
  })
}

///
fn build_interval_start(bx: &BuildContext, lhs: &AstNode, rhs: &bool) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let closed = *rhs;
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::IntervalStart(Box::new(lhv), closed)
  })
}

///
fn build_irrelevant(_bx: &BuildContext) -> Evaluator {
  Box::new(move |_: &FeelScope| Value::Irrelevant)
}

///
fn build_instance_of(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Value::FeelType(feel_type) = rhv {
      match lhv {
        Value::Number { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::Number => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::String { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::String => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::Boolean { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::Boolean => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::Date { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::Date => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::DateTime { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::DateTime => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::Time { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::Time => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::YearsAndMonthsDuration { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::YearsAndMonthsDuration => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::DaysAndTimeDuration { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::DaysAndTimeDuration => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::Null { .. } => match feel_type {
          FeelType::Null => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        value @ Value::Range { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          expected => Value::Boolean(value.type_of() == expected),
        },
        value @ Value::List { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          expected => Value::Boolean(value.type_of().instance_of(&expected)),
        },
        value @ Value::Context { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          expected => Value::Boolean(value.type_of().instance_of(&expected)),
        },
        value @ Value::FunctionDefinition { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          expected => Value::Boolean(value.type_of() == expected),
        },
        other => value_null!("invalid value in 'instance of' operator: {}", other),
      }
    } else {
      Value::Boolean(lhv.type_of() == rhv.type_of())
    }
  })
}

///
fn build_le(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_number"),
      },
      Value::String(lh) => match rhv {
        Value::String(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_string"),
      },
      Value::Date(lh) => match rhv {
        Value::Date(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_date"),
      },
      Value::DateTime(lh) => match rhv {
        Value::DateTime(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_date_time"),
      },
      Value::Time(lh) => match rhv {
        Value::Time(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_time"),
      },
      Value::DaysAndTimeDuration(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_days_and_time_duration"),
      },
      Value::YearsAndMonthsDuration(lh) => match rhv {
        Value::YearsAndMonthsDuration(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_years_and_months_duration"),
      },
      _ => value_null!("eval_less_or_equal"),
    }
  })
}

///
fn build_lt(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_number"),
      },
      Value::String(lh) => match rhv {
        Value::String(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_string"),
      },
      Value::Date(lh) => match rhv {
        Value::Date(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_date"),
      },
      Value::DateTime(lh) => match rhv {
        Value::DateTime(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_date_time"),
      },
      Value::Time(lh) => match rhv {
        Value::Time(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_time"),
      },
      Value::DaysAndTimeDuration(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_days_and_time_duration"),
      },
      Value::YearsAndMonthsDuration(lh) => match rhv {
        Value::YearsAndMonthsDuration(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_years_and_months_duration"),
      },
      _ => value_null!("eval_less_then"),
    }
  })
}

///
fn build_list(bx: &BuildContext, lhs: &[AstNode]) -> Evaluator {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(bx, node));
  }
  Box::new(move |scope: &FeelScope| {
    let mut values = vec![];
    for evaluator in &evaluators {
      values.push(evaluator(scope))
    }
    Value::List(values)
  })
}

///
fn build_list_type(bx: &BuildContext, lhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    if let Value::FeelType(feel_type) = lhv {
      Value::FeelType(FeelType::List(Box::new(feel_type)))
    } else {
      value_null!("expected a feel type")
    }
  })
}

///
fn build_mul(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Number(lh * rh),
        Value::DaysAndTimeDuration(ref rh) => {
          let val = lh * FeelNumber::from(rh.as_nanos());
          if let Ok(v) = FeelNumber::try_into(val) {
            Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_n(v))
          } else {
            value_null!("multiplication result is out of range of days and time duration")
          }
        }
        Value::YearsAndMonthsDuration(ref rh) => {
          let val = lh * FeelNumber::from(rh.as_months());
          if let Ok(v) = FeelNumber::try_into(val) {
            Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_m(v))
          } else {
            value_null!("multiplication result is out of range of years and months duration")
          }
        }
        _ => value_null!("[multiplication] incompatible types: {} * {}", lhv, rhv),
      },
      Value::DaysAndTimeDuration(ref lh) => match rhv {
        Value::Number(rh) => {
          let val = FeelNumber::from(lh.as_nanos()) * rh;
          if let Ok(v) = FeelNumber::try_into(val) {
            Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_n(v))
          } else {
            value_null!("multiplication result is out of range of days and time duration")
          }
        }
        _ => value_null!("[multiplication] incompatible types: {} * {}", lhv, rhv),
      },
      Value::YearsAndMonthsDuration(ref lh) => match rhv {
        Value::Number(rh) => {
          let val = FeelNumber::from(lh.as_months()) * rh;
          if let Ok(v) = FeelNumber::try_into(val) {
            Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_m(v))
          } else {
            value_null!("multiplication result is out of range of years and months duration")
          }
        }
        _ => value_null!("[multiplication] incompatible types: {} * {}", lhv, rhv),
      },
      value @ Value::Null(_) => value,
      other => value_null!("unexpected value type in multiplication: {}", other.type_of()),
    }
  })
}

///
fn build_name(_bx: &BuildContext, name: Name) -> Evaluator {
  Box::new(move |scope: &FeelScope| {
    if let Some(value) = scope.get_value(&name) {
      value
    } else if let Ok(bif) = Bif::from_str(&name.to_string()) {
      Value::BuiltInFunction(bif)
    } else {
      value_null!("context has no value for key '{}'", name)
    }
  })
}

///
fn build_named_parameter(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  if let AstNode::ParameterName(name) = lhs {
    let lhv = Value::ParameterName(name.clone());
    let rhe = build_evaluator(bx, rhs);
    return Box::new(move |scope: &FeelScope| {
      let rhv = rhe(scope);
      Value::NamedParameter(Box::new(lhv.clone()), Box::new(rhv))
    });
  }
  build_err_msg(err_expected_parameter_name(lhs))
}

fn build_named_parameters(bx: &BuildContext, lhs: &[AstNode]) -> Evaluator {
  let mut evaluators = vec![];
  for item in lhs {
    evaluators.push(build_evaluator(bx, item));
  }
  Box::new(move |scope: &FeelScope| {
    let mut parameters = BTreeMap::new();
    let mut position = 1_usize;
    for evaluator in &evaluators {
      if let Value::NamedParameter(name, value) = evaluator(scope) {
        if let Value::ParameterName(name) = *name {
          parameters.insert(name, (*value, position));
          position += 1;
        }
      }
    }
    Value::NamedParameters(parameters)
  })
}

///
fn build_neg(bx: &BuildContext, lhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    match lhv {
      Value::Number(lh) => Value::Number(-lh),
      Value::DaysAndTimeDuration(lh) => Value::DaysAndTimeDuration(-lh),
      Value::YearsAndMonthsDuration(lh) => Value::YearsAndMonthsDuration(-lh),
      other => value_null!("unexpected type in arithmetic negation: {}", other.type_of()),
    }
  })
}

///
fn build_negated_list(bx: &BuildContext, lhs: &[AstNode]) -> Evaluator {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(bx, node));
  }
  Box::new(move |scope: &FeelScope| {
    let mut values = vec![];
    for evaluator in &evaluators {
      values.push(evaluator(scope))
    }
    Value::NegatedCommaList(values)
  })
}

///
fn build_null(_bx: &BuildContext) -> Evaluator {
  Box::new(move |_: &FeelScope| Value::Null(None))
}

///
fn build_numeric(_bx: &BuildContext, lhs: &str, rhs: &str) -> Evaluator {
  let text = format!("{lhs}.{rhs}");
  if let Ok(num) = text.parse::<FeelNumber>() {
    Box::new(move |_: &FeelScope| Value::Number(num))
  } else {
    Box::new(move |_: &FeelScope| value_null!("failed to convert text '{}' into number", text))
  }
}

///
fn build_nq(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Some(result) = eval_ternary_equality(&lhv, &rhv) {
      Value::Boolean(!result)
    } else {
      value_null!()
    }
  })
}

/// Semantics of disjunction.
/// ```text
/// left        right        result
///─────────────────────────────────
/// true        true         true
/// true        false        true
/// true        otherwise    true
/// false       true         true
/// false       false        false
/// false       otherwise    null
/// otherwise   true         true
/// otherwise   false        null
/// otherwise   otherwise    null
/// ```
fn build_or(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Boolean(lh) => match rhv {
        Value::Boolean(rh) => Value::Boolean(lh || rh),
        _ => {
          if lh {
            Value::Boolean(true)
          } else {
            value_null!()
          }
        }
      },
      _ => match rhv {
        Value::Boolean(rh) => {
          if rh {
            Value::Boolean(true)
          } else {
            value_null!()
          }
        }
        _ => value_null!(),
      },
    }
  })
}

///
fn build_out(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let ine = build_in(bx, lhs, rhs);
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let inv = ine(scope);
    let lhv = lhe(scope);
    match inv {
      Value::Boolean(true) => lhv,
      _ => value_null!(),
    }
  })
}

///
fn build_parameter_name(_bx: &BuildContext, lhs: &Name) -> Evaluator {
  let name = lhs.to_owned();
  Box::new(move |_: &FeelScope| Value::ParameterName(name.clone()))
}

///
fn build_parameter_types(bx: &BuildContext, lhs: &[AstNode]) -> Evaluator {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(bx, node));
  }
  Box::new(move |scope: &FeelScope| {
    let mut values = vec![];
    for evaluator in &evaluators {
      values.push(evaluator(scope))
    }
    Value::ParameterTypes(values)
  })
}

///
fn build_qualified_name(bx: &BuildContext, lhs: &[AstNode]) -> Evaluator {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(bx, node));
  }
  Box::new(move |scope: &FeelScope| {
    let mut names = vec![];
    for evaluator in &evaluators {
      if let Value::QualifiedNameSegment(name) = evaluator(scope) {
        names.push(name);
      }
    }
    scope.search(&names).unwrap_or_else(|| value_null!("no value for qualified name"))
  })
}

///
fn build_qualified_name_segment(_bx: &BuildContext, name: &Name) -> Evaluator {
  let name = name.to_owned();
  Box::new(move |_: &FeelScope| Value::QualifiedNameSegment(name.to_owned()))
}

///
fn get_property_from_value(value: Value, name: &Name) -> Value {
  let property_name = name.to_string();
  match value {
    Value::Date(date) => match property_name.as_str() {
      "year" => Value::Number(date.year().into()),
      "month" => Value::Number(date.month().into()),
      "day" => Value::Number(date.day().into()),
      "weekday" => {
        if let Some(day_of_week) = date.day_of_week() {
          Value::Number(day_of_week.1.into())
        } else {
          value_null!("could not retrieve weekday for date")
        }
      }
      other => value_null!("no such property in date: {}", other),
    },
    Value::DateTime(date_time) => match property_name.as_str() {
      "year" => Value::Number(date_time.year().into()),
      "month" => Value::Number(date_time.month().into()),
      "day" => Value::Number(date_time.day().into()),
      "weekday" => {
        if let Some(day_of_week) = date_time.day_of_week() {
          Value::Number(day_of_week.1.into())
        } else {
          value_null!("could not retrieve weekday for date and time")
        }
      }
      "hour" => Value::Number(date_time.hour().into()),
      "minute" => Value::Number(date_time.minute().into()),
      "second" => Value::Number(date_time.second().into()),
      "time offset" => {
        if let Some(offset) = date_time.feel_time_offset() {
          Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_s(offset as i64))
        } else {
          value_null!("could not retrieve time offset for date and time")
        }
      }
      "timezone" => {
        if let Some(feel_time_zone) = date_time.feel_time_zone() {
          Value::String(feel_time_zone)
        } else {
          value_null!("could not retrieve timezone for date and time")
        }
      }
      other => value_null!("no such property in date and time: {}", other),
    },
    Value::Time(time) => match property_name.as_str() {
      "hour" => Value::Number(time.hour().into()),
      "minute" => Value::Number(time.minute().into()),
      "second" => Value::Number(time.second().into()),
      "time offset" => {
        if let Some(offset) = time.feel_time_offset() {
          Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_s(offset as i64))
        } else {
          value_null!("could not retrieve time offset for time")
        }
      }
      "timezone" => {
        if let Some(feel_time_zone) = time.feel_time_zone() {
          Value::String(feel_time_zone)
        } else {
          value_null!("could not retrieve timezone for time")
        }
      }
      other => value_null!("no such property in time: {}", other),
    },
    Value::DaysAndTimeDuration(dt_duration) => match property_name.as_str() {
      "days" => Value::Number(dt_duration.get_days().into()),
      "hours" => Value::Number(dt_duration.get_hours().into()),
      "minutes" => Value::Number(dt_duration.get_minutes().into()),
      "seconds" => Value::Number(dt_duration.get_seconds().into()),
      other => value_null!("no such property in days and time duration: {}", other),
    },
    Value::YearsAndMonthsDuration(ym_duration) => match property_name.as_str() {
      "years" => Value::Number(ym_duration.years().into()),
      "months" => Value::Number(ym_duration.months().into()),
      other => value_null!("no such property in years and months duration: {}", other),
    },
    Value::Range(rs, cs, re, ce) => match property_name.as_str() {
      "start" => *rs,
      "start included" => Value::Boolean(cs),
      "end" => *re,
      "end included" => Value::Boolean(ce),
      other => value_null!("no such property in range: {}", other),
    },
    Value::UnaryGreater(value) => match property_name.as_str() {
      "start" => *value,
      "start included" => Value::Boolean(false),
      "end included" => Value::Boolean(false),
      other => value_null!("no such property in unary greater: {}", other),
    },
    Value::UnaryGreaterOrEqual(value) => match property_name.as_str() {
      "start" => *value,
      "start included" => Value::Boolean(true),
      "end included" => Value::Boolean(false),
      other => value_null!("no such property in unary greater or equal: {}", other),
    },
    Value::UnaryLess(value) => match property_name.as_str() {
      "end" => *value,
      "start included" => Value::Boolean(false),
      "end included" => Value::Boolean(false),
      other => value_null!("no such property in unary less: {}", other),
    },
    Value::UnaryLessOrEqual(value) => match property_name.as_str() {
      "end" => *value,
      "start included" => Value::Boolean(false),
      "end included" => Value::Boolean(true),
      other => value_null!("no such property in unary less or equal: {}", other),
    },
    v @ Value::Null(_) => v,
    other => value_null!("unexpected type: {}, for property: {}", other.type_of(), property_name),
  }
}

///
fn build_qualified_name_from_path(node: &AstNode) -> Option<QualifiedName> {
  match node {
    AstNode::Path(lhs, rhs) => {
      if let AstNode::Name(name) = lhs.borrow() {
        if let Some(mut qualified_name) = build_qualified_name_from_path(rhs) {
          qualified_name.insert(0, name.clone());
          return Some(qualified_name);
        }
      }
      None
    }
    AstNode::Name(name) => Some(name.clone().into()),
    _ => None,
  }
}

///
fn build_path(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let Some(qualified_name) = build_qualified_name_from_path(rhs) else {
    return build_err_msg(err_invalid_qualified_name(rhs));
  };
  let mut property_path = qualified_name.clone();
  let property_name = property_path.pop().unwrap();
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    match lhv {
      Value::Context(context) => {
        if let Some(value) = context.search_entry(&qualified_name) {
          return value.clone();
        }
        if let Some(value) = context.search_entry(&property_path) {
          return get_property_from_value(value.clone(), &property_name);
        }
        value_null!("build_path: no entry {} in context: {}", qualified_name, context)
      }
      Value::List(items) => {
        let mut result = vec![];
        for item in items {
          if let Value::Context(context) = item {
            if let Some(value) = context.search_entry(&qualified_name) {
              result.push(value.clone());
            } else if let Some(value) = context.search_entry(&property_path) {
              result.push(get_property_from_value(value.clone(), &property_name));
            } else {
              result.push(value_null!());
            }
          } else {
            return value_null!("build_path: no context in list");
          }
        }
        Value::List(result)
      }
      other => get_property_from_value(other, &property_name),
    }
  })
}

///
fn build_range(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Value::IntervalStart(lhv, l_closed) = lhv {
      if let Value::IntervalEnd(rhv, r_closed) = rhv {
        Value::Range(lhv, l_closed, rhv, r_closed)
      } else {
        value_null!("expected interval end")
      }
    } else {
      value_null!("expected interval start")
    }
  })
}

///
fn build_range_type(bx: &BuildContext, lhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    if let Value::FeelType(feel_type) = lhv {
      Value::FeelType(FeelType::Range(Box::new(feel_type)))
    } else {
      value_null!("expected a feel type")
    }
  })
}

///
fn build_some(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let mut expr_evaluators = vec![];
  let AstNode::QuantifiedContexts(items) = lhs else {
    return build_err_msg(err_msg_expected_node("QuantifiedContexts", lhs));
  };
  for item in items {
    if let AstNode::QuantifiedContext(variable_name, expr_node) = item {
      if let AstNode::Name(name) = variable_name.borrow() {
        let evaluator_single = build_evaluator(bx, expr_node);
        expr_evaluators.push((name.clone(), evaluator_single));
      }
    }
  }
  let AstNode::Satisfies(satisfies) = rhs else {
    return build_err_msg(err_msg_expected_node("Satisfies", lhs));
  };
  let satisfies_evaluator = build_evaluator(bx, satisfies);
  Box::new(move |scope: &FeelScope| {
    let mut expression_evaluator = SomeExpressionEvaluator::new();
    for (name, expr_evaluator) in &expr_evaluators {
      expression_evaluator.add_list(name.clone(), expr_evaluator(scope));
    }
    expression_evaluator.evaluate(scope, &satisfies_evaluator)
  })
}

///
fn build_string(_bx: &BuildContext, lhs: &str) -> Evaluator {
  let value = Value::String(lhs.to_string());
  Box::new(move |_: &FeelScope| value.clone())
}

///
fn build_sub(bx: &BuildContext, lhs: &AstNode, rhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  let rhe = build_evaluator(bx, rhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => {
        if let Value::Number(rh) = rhv {
          return Value::Number(lh - rh);
        }
      }
      Value::Date(lh) => match rhv {
        Value::Date(rh) => {
          let l = FeelDateTime::new(lh, FeelTime::utc(0, 0, 0, 0));
          let r = FeelDateTime::new(rh, FeelTime::utc(0, 0, 0, 0));
          if let Some(result) = l - r {
            return Value::DaysAndTimeDuration(result);
          }
        }
        Value::DateTime(rh) => {
          let l = FeelDateTime::new(lh, FeelTime::utc(0, 0, 0, 0));
          if let Some(result) = l - rh {
            return Value::DaysAndTimeDuration(result);
          }
        }
        Value::DaysAndTimeDuration(rh) => {
          if let Some(date) = lh - rh {
            return Value::Date(date);
          }
        }
        Value::YearsAndMonthsDuration(rh) => {
          if let Some(date) = lh - rh {
            return Value::Date(date);
          }
        }
        _ => {}
      },
      Value::Time(lh) => match rhv {
        Value::Time(rh) => {
          if let Some(result) = lh - rh {
            return Value::DaysAndTimeDuration(result);
          }
        }
        Value::DaysAndTimeDuration(rh) => {
          return Value::Time(lh - rh);
        }
        _ => {}
      },
      Value::DateTime(lh) => match rhv {
        Value::Date(rh) => {
          let r = FeelDateTime::new(rh, FeelTime::utc(0, 0, 0, 0));
          if let Some(result) = lh - r {
            return Value::DaysAndTimeDuration(result);
          }
        }
        Value::DateTime(rh) => {
          if let Some(result) = lh - rh {
            return Value::DaysAndTimeDuration(result);
          }
        }
        Value::DaysAndTimeDuration(rh) => {
          if let Some(result) = lh - rh {
            return Value::DateTime(result);
          }
        }
        Value::YearsAndMonthsDuration(rh) => {
          if let Some(result) = lh - rh {
            return Value::DateTime(result);
          }
        }
        _ => {}
      },
      Value::DaysAndTimeDuration(lh) => {
        if let Value::DaysAndTimeDuration(rh) = rhv {
          return Value::DaysAndTimeDuration(lh - rh);
        }
      }
      Value::YearsAndMonthsDuration(lh) => {
        if let Value::YearsAndMonthsDuration(rh) = rhv {
          return Value::YearsAndMonthsDuration(lh - rh);
        }
      }
      _ => {}
    }
    value_null!("[subtraction] incompatible types: {} - {}", lhe(scope) as Value, rhe(scope) as Value)
  })
}

///
fn build_unary_ge(bx: &BuildContext, lhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::UnaryGreaterOrEqual(Box::from(lhv))
  })
}

///
fn build_unary_gt(bx: &BuildContext, lhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::UnaryGreater(Box::from(lhv))
  })
}

///
fn build_unary_le(bx: &BuildContext, lhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::UnaryLessOrEqual(Box::from(lhv))
  })
}

///
fn build_unary_lt(bx: &BuildContext, lhs: &AstNode) -> Evaluator {
  let lhe = build_evaluator(bx, lhs);
  Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::UnaryLess(Box::from(lhv))
  })
}

/// Evaluates ternary equality between two values.
pub fn eval_ternary_equality(lhs: &Value, rhs: &Value) -> Option<bool> {
  match lhs {
    Value::Boolean(ls) => match rhs {
      Value::Boolean(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::Number(ls) => match rhs {
      Value::Number(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::String(ls) => match rhs {
      Value::String(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::Context(ls) => match rhs {
      Value::Context(rs) => {
        if ls.keys().len() == rs.keys().len() {
          for (key1, value1) in ls.deref() {
            if let Some(value2) = rs.get_entry(key1) {
              if let Some(equal) = eval_ternary_equality(value1, value2) {
                if !equal {
                  return Some(false); // values in entries are NOT EQUAL
                }
              } else {
                // values in entries can not be compared
                return None;
              }
            } else {
              // there is no such key in the right-side context
              return Some(false);
            }
          }
          // contexts are EQUAL
          return Some(true);
        }
        // contexts have different number of keys, so they are NOT EQUAL
        Some(false)
      }
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::Date(ls) => match rhs {
      Value::Date(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::Time(ls) => match rhs {
      Value::Time(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::DateTime(ls) => match rhs {
      Value::DateTime(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::DaysAndTimeDuration(ls) => match rhs {
      Value::DaysAndTimeDuration(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::YearsAndMonthsDuration(ls) => match rhs {
      Value::YearsAndMonthsDuration(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    ls @ Value::Null(_) => match rhs {
      rs @ Value::Null(_) => {
        if (ls.is_invalid_coercion() && !rs.is_invalid_coercion()) || (!ls.is_invalid_coercion() && rs.is_invalid_coercion()) {
          None
        } else {
          Some(true)
        }
      }
      _ => None,
    },
    Value::List(ls) => match rhs {
      Value::List(rs) => {
        if ls.len() == rs.len() {
          for (l, r) in ls.iter().zip(rs.iter()) {
            if let Some(true) = eval_ternary_equality(l, r) {
            } else {
              return Some(false);
            }
          }
          Some(true)
        } else {
          Some(false)
        }
      }
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::UnaryGreater(end) => match rhs {
      Value::Range(rs, cs, re, ce) => {
        if !*cs && !*ce && re.is_null() {
          eval_ternary_equality(end, rs)
        } else {
          Some(false)
        }
      }
      _ => None,
    },
    Value::UnaryLess(end) => match rhs {
      Value::Range(rs, cs, re, ce) => {
        if !*cs && !*ce && rs.is_null() {
          eval_ternary_equality(end, re)
        } else {
          Some(false)
        }
      }
      _ => None,
    },
    Value::UnaryGreaterOrEqual(end) => match rhs {
      Value::Range(rs, cs, re, ce) => {
        if *cs && !*ce && re.is_null() {
          eval_ternary_equality(end, rs)
        } else {
          Some(false)
        }
      }
      _ => None,
    },
    Value::UnaryLessOrEqual(end) => match rhs {
      Value::Range(rs, cs, re, ce) => {
        if !*cs && *ce && rs.is_null() {
          eval_ternary_equality(end, re)
        } else {
          Some(false)
        }
      }
      _ => None,
    },
    Value::Range(r1s, c1s, r1e, c1e) => match rhs {
      Value::Range(r2s, c2s, r2e, c2e) => {
        if *c1s == *c2s && *c1e == *c2e {
          if let Some(true) = eval_ternary_equality(r1s, r2s) {
            return eval_ternary_equality(r1e, r2e);
          }
        }
        Some(false)
      }
      Value::Null(_) => Some(false),
      _ => None,
    },
    _ => None,
  }
}

///
fn eval_in_list(left: &Value, items: &[Value]) -> Value {
  for item in items {
    match item {
      inner @ Value::Null(_)
      | inner @ Value::String(_)
      | inner @ Value::Number(_)
      | inner @ Value::Boolean(_)
      | inner @ Value::Date(_)
      | inner @ Value::Time(_)
      | inner @ Value::DateTime(_)
      | inner @ Value::YearsAndMonthsDuration(_)
      | inner @ Value::DaysAndTimeDuration(_)
      | inner @ Value::Context(_) => {
        if let Value::Boolean(true) = eval_in_equal(left, inner) {
          return VALUE_TRUE;
        }
      }
      Value::UnaryLess(inner) => {
        if let Value::Boolean(true) = eval_in_unary_less(left, inner.borrow()) {
          return VALUE_TRUE;
        }
      }
      Value::UnaryLessOrEqual(inner) => {
        if let Value::Boolean(true) = eval_in_unary_less_or_equal(left, inner.borrow()) {
          return VALUE_TRUE;
        }
      }
      Value::UnaryGreater(inner) => {
        if let Value::Boolean(true) = eval_in_unary_greater(left, inner.borrow()) {
          return VALUE_TRUE;
        }
      }
      Value::UnaryGreaterOrEqual(inner) => {
        if let Value::Boolean(true) = eval_in_unary_greater_or_equal(left, inner.borrow()) {
          return VALUE_TRUE;
        }
      }
      Value::List(inner) => {
        if let Value::Boolean(true) = eval_in_list(left, inner) {
          return VALUE_TRUE;
        }
      }
      Value::Range(l, l_closed, r, r_closed) => {
        if let Value::Boolean(true) = eval_in_range(left, l, *l_closed, r, *r_closed) {
          return VALUE_TRUE;
        }
      }
      _ => return value_null!("eval_in_list"),
    }
  }
  VALUE_FALSE
}

/// Checks if all elements from `list` are present in `items`.
fn eval_in_list_in_list(l_items: &[Value], r_items: &[Value]) -> Value {
  for item in r_items {
    if let Value::List(rhs) = item {
      let mut available: HashSet<usize> = (0..rhs.len()).collect();
      for l in l_items {
        let mut found = false;
        for (index, r) in rhs.iter().enumerate() {
          if available.contains(&index) {
            if let Value::Boolean(true) = eval_in_equal(l, r) {
              available.remove(&index);
              found = true;
              break;
            }
          }
        }
        if !found {
          return VALUE_FALSE;
        }
      }
      return VALUE_TRUE;
    }
  }
  VALUE_FALSE
}

///
fn eval_in_negated_list(left: &Value, items: &[Value]) -> Value {
  for item in items {
    match item {
      inner @ Value::Null(_)
      | inner @ Value::String(_)
      | inner @ Value::Number(_)
      | inner @ Value::Boolean(_)
      | inner @ Value::Date(_)
      | inner @ Value::Time(_)
      | inner @ Value::DateTime(_)
      | inner @ Value::DaysAndTimeDuration(_)
      | inner @ Value::YearsAndMonthsDuration(_) => {
        if let Value::Boolean(true) = eval_in_equal(left, inner) {
          return Value::Boolean(false);
        }
      }
      Value::UnaryLess(inner) => {
        if let Value::Boolean(true) = eval_in_unary_less(left, inner.borrow()) {
          return Value::Boolean(false);
        }
      }
      Value::UnaryLessOrEqual(inner) => {
        if let Value::Boolean(true) = eval_in_unary_less_or_equal(left, inner.borrow()) {
          return Value::Boolean(false);
        }
      }
      Value::UnaryGreater(inner) => {
        if let Value::Boolean(true) = eval_in_unary_greater(left, inner.borrow()) {
          return Value::Boolean(false);
        }
      }
      Value::UnaryGreaterOrEqual(inner) => {
        if let Value::Boolean(true) = eval_in_unary_greater_or_equal(left, inner.borrow()) {
          return Value::Boolean(false);
        }
      }
      Value::List(inner) => {
        if let Value::Boolean(true) = eval_in_list(left, inner) {
          return Value::Boolean(false);
        }
      }
      Value::Range(l, l_closed, r, r_closed) => {
        if let Value::Boolean(true) = eval_in_range(left, l, *l_closed, r, *r_closed) {
          return Value::Boolean(false);
        }
      }
      other => return value_null!("unexpected type in negated list: {}", other.type_of()),
    }
  }
  Value::Boolean(true)
}

///
fn eval_in_range(lhv: &Value, l: &Value, l_closed: bool, r: &Value, r_closed: bool) -> Value {
  match lhv {
    Value::Number(value) => match l {
      Value::Number(lv) => match r {
        Value::Number(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::String(value) => match l {
      Value::String(lv) => match r {
        Value::String(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::Date(value) => match l {
      Value::Date(lv) => match r {
        Value::Date(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::Time(value) => match l {
      Value::Time(lv) => match r {
        Value::Time(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::DateTime(value) => match l {
      Value::DateTime(lv) => match r {
        Value::DateTime(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::YearsAndMonthsDuration(value) => match l {
      Value::YearsAndMonthsDuration(lv) => match r {
        Value::YearsAndMonthsDuration(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::DaysAndTimeDuration(value) => match l {
      Value::DaysAndTimeDuration(lv) => match r {
        Value::DaysAndTimeDuration(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    _ => value_null!("eval_in_range"),
  }
}

///
fn eval_in_equal(left: &Value, right: &Value) -> Value {
  if let Some(true) = eval_ternary_equality(left, right) {
    VALUE_TRUE
  } else {
    VALUE_FALSE
  }
}

///
fn eval_in_unary_less(left: &Value, right: &Value) -> Value {
  match right {
    Value::Number(r) => {
      if let Value::Number(l) = left {
        return Value::Boolean(*l < *r);
      }
    }
    Value::String(r) => {
      if let Value::String(l) = left {
        return Value::Boolean(l < r);
      }
    }
    Value::Date(r) => {
      if let Value::Date(l) = left {
        return Value::Boolean(l < r);
      }
    }
    Value::Time(r) => {
      if let Value::Time(l) = left {
        return Value::Boolean(l < r);
      }
    }
    Value::DateTime(r) => {
      if let Value::DateTime(l) = left {
        return Value::Boolean(l < r);
      }
    }
    Value::YearsAndMonthsDuration(r) => {
      if let Value::YearsAndMonthsDuration(l) = left {
        return Value::Boolean(l < r);
      }
    }
    Value::DaysAndTimeDuration(r) => {
      if let Value::DaysAndTimeDuration(l) = left {
        return Value::Boolean(l < r);
      }
    }
    _ => {}
  }
  value_null!("eval_in_unary_less")
}

///
fn eval_in_unary_less_or_equal(left: &Value, right: &Value) -> Value {
  match right {
    Value::Number(r) => {
      if let Value::Number(l) = left {
        return Value::Boolean(*l <= *r);
      }
    }
    Value::String(r) => {
      if let Value::String(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    Value::Date(r) => {
      if let Value::Date(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    Value::Time(r) => {
      if let Value::Time(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    Value::DateTime(r) => {
      if let Value::DateTime(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    Value::YearsAndMonthsDuration(r) => {
      if let Value::YearsAndMonthsDuration(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    Value::DaysAndTimeDuration(r) => {
      if let Value::DaysAndTimeDuration(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    _ => {}
  }
  value_null!("eval_in_unary_less_or_equal")
}

///
fn eval_in_unary_greater(left: &Value, right: &Value) -> Value {
  match right {
    Value::Number(r) => {
      if let Value::Number(l) = left {
        return Value::Boolean(*l > *r);
      }
    }
    Value::String(r) => {
      if let Value::String(l) = left {
        return Value::Boolean(l > r);
      }
    }
    Value::Date(r) => {
      if let Value::Date(l) = left {
        return Value::Boolean(l > r);
      }
    }
    Value::Time(r) => {
      if let Value::Time(l) = left {
        return Value::Boolean(l > r);
      }
    }
    Value::DateTime(r) => {
      if let Value::DateTime(l) = left {
        return Value::Boolean(l > r);
      }
    }
    Value::YearsAndMonthsDuration(r) => {
      if let Value::YearsAndMonthsDuration(l) = left {
        return Value::Boolean(l > r);
      }
    }
    Value::DaysAndTimeDuration(r) => {
      if let Value::DaysAndTimeDuration(l) = left {
        return Value::Boolean(l > r);
      }
    }
    _ => {}
  }
  value_null!("eval_in_unary_greater")
}

///
fn eval_in_unary_greater_or_equal(left: &Value, right: &Value) -> Value {
  match right {
    Value::Number(r) => {
      if let Value::Number(l) = left {
        return Value::Boolean(*l >= *r);
      }
    }
    Value::String(r) => {
      if let Value::String(left_value) = left {
        return Value::Boolean(left_value >= r);
      }
    }
    Value::Date(r) => {
      if let Value::Date(l) = left {
        return Value::Boolean(l >= r);
      }
    }
    Value::Time(r) => {
      if let Value::Time(l) = left {
        return Value::Boolean(l >= r);
      }
    }
    Value::DateTime(r) => {
      if let Value::DateTime(l) = left {
        return Value::Boolean(l >= r);
      }
    }
    Value::YearsAndMonthsDuration(r) => {
      if let Value::YearsAndMonthsDuration(l) = left {
        return Value::Boolean(l >= r);
      }
    }
    Value::DaysAndTimeDuration(r) => {
      if let Value::DaysAndTimeDuration(l) = left {
        return Value::Boolean(l >= r);
      }
    }
    _ => {}
  }
  value_null!("eval_in_unary_greater_or_equal")
}

/// Evaluates function definition with positional parameters.
fn eval_function_with_positional_parameters(
  scope: &FeelScope,
  args: &[Value],
  params: &[(Name, FeelType)],
  body: &FunctionBody,
  closure_ctx: FeelContext,
  result_type: FeelType,
) -> Value {
  let mut params_ctx = FeelContext::default();
  if args.len() != params.len() {
    return value_null!("invalid number of arguments");
  }
  for (argument_value, (parameter_name, parameter_type)) in args.iter().zip(params) {
    params_ctx.set_entry(parameter_name, argument_value.coerced(parameter_type))
  }
  eval_function_definition(scope, params_ctx, body, closure_ctx, result_type)
}

/// Evaluates function definition with named parameters.
fn eval_function_with_named_parameters(
  scope: &FeelScope,
  args: &Value,
  params: &[(Name, FeelType)],
  body: &FunctionBody,
  closure_ctx: FeelContext,
  result_type: FeelType,
) -> Value {
  let mut params_ctx = FeelContext::default();
  if let Value::NamedParameters(argument_map) = args {
    if argument_map.len() != params.len() {
      return value_null!("invalid number of arguments");
    }
    for (parameter_name, parameter_type) in params {
      if let Some((argument, _)) = argument_map.get(parameter_name) {
        params_ctx.set_entry(parameter_name, argument.coerced(parameter_type))
      } else {
        return value_null!("parameter with name {} not found in arguments", parameter_name);
      }
    }
  }
  eval_function_definition(scope, params_ctx, body, closure_ctx, result_type)
}

/// Evaluates function definition.
fn eval_function_definition(scope: &FeelScope, params_ctx: FeelContext, body: &FunctionBody, closure_ctx: FeelContext, result_type: FeelType) -> Value {
  scope.push(closure_ctx); // closure_ctx
  scope.push(params_ctx); // params_ctx
  let mut result = body.evaluate(scope);
  if let Value::FunctionDefinition(fd_params, fd_body, fd_external, fd_closure, fd_closure_ctx, fd_result_type) = &result {
    let mut new_closure_ctx = fd_closure_ctx.clone();
    for closure_name in fd_closure.iter() {
      if let Some(closure_value) = scope.search_entry(closure_name) {
        new_closure_ctx.create_entry(closure_name, closure_value);
      }
    }
    result = Value::FunctionDefinition(
      fd_params.to_owned(),
      fd_body.to_owned(),
      fd_external.to_owned(),
      fd_closure.to_owned(),
      new_closure_ctx,
      fd_result_type.to_owned(),
    );
  }
  scope.pop(); // params_ctx
  scope.pop(); // closure_ctx
  result.coerced(&result_type)
}

/// Evaluates external function definition with positional parameters.
fn eval_external_function_with_positional_parameters(scope: &FeelScope, args: &[Value], params: &[(Name, FeelType)], body: &FunctionBody, result_type: FeelType) -> Value {
  if args.len() != params.len() {
    return value_null!("invalid number of arguments");
  }
  eval_external_function_definition(scope, args, body, result_type)
}

/// Evaluates external function definition with named parameters.
fn eval_external_function_with_named_parameters(scope: &FeelScope, args: &Value, params: &[(Name, FeelType)], body: &FunctionBody, result_type: FeelType) -> Value {
  let mut args1 = vec![];
  if let Value::NamedParameters(argument_map) = args {
    if argument_map.len() != params.len() {
      return value_null!("invalid number of arguments");
    }
    for (parameter_name, parameter_type) in params {
      if let Some((argument, _)) = argument_map.get(parameter_name) {
        args1.push(argument.coerced(parameter_type));
      } else {
        return value_null!("parameter with name {} not found in arguments", parameter_name);
      }
    }
  }
  eval_external_function_definition(scope, &args1, body, result_type)
}

/// Evaluates external function definition.
fn eval_external_function_definition(scope: &FeelScope, arguments: &[Value], body: &FunctionBody, result_type: FeelType) -> Value {
  let result = match &body.evaluate(scope) {
    Value::ExternalJavaFunction(class_name, method_signature) => evaluate_external_java_function(class_name, method_signature, arguments),
    Value::ExternalPmmlFunction(document, model_name) => evaluate_external_pmml_function(document, model_name, arguments),
    other => value_null!("expected JAVA or PMML mapping, actual value is {}", other),
  };
  result.coerced(&result_type)
}

fn build_err_msg(err_msg: String) -> Evaluator {
  Box::new(move |_: &FeelScope| value_null!(err_msg))
}

/// Returns an error message for unexpected node in AST.
fn err_msg_unexpected_node(node: &AstNode) -> String {
  format!("unexpected AST node: {node:?}")
}

/// Returns an error message when other AST node was expected.
fn err_msg_expected_node(expected: &str, actual: &AstNode) -> String {
  format!("expected AST node {expected}, actual AST node is {actual:?}")
}

/// Returns an error message when expected only a named or positional parameter.
fn err_msg_expected_positional_or_named_parameter(actual: &AstNode) -> String {
  format!("expected positional or named parameter, actual AST node is {actual:?}")
}

/// Returns an error message when expected only a parameter name.
fn err_expected_parameter_name(actual: &AstNode) -> String {
  format!("expected AST node ParameterName, actual node is {actual:?}")
}

/// Returns an error message when expected only a parameter name.
fn err_invalid_qualified_name(actual: &AstNode) -> String {
  format!("invalid qualified name node: {actual:?}")
}

#[cfg(test)]
mod tests {
  use super::*;
  use dsntk_feel::{scope, FeelType};

  #[test]
  fn test_unimplemented_external_function_kind() {
    let evaluator = Box::new(move |_: &FeelScope| Value::Boolean(false)) as Evaluator;
    let body = FunctionBody::External(Arc::new(evaluator));
    let result = eval_external_function_definition(&scope!(), &[], &body, FeelType::Boolean);
    assert_eq!("null(expected JAVA or PMML mapping, actual value is false)", result.to_string())
  }
}
