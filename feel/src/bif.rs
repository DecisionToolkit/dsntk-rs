//! Definitions of built-in functions.

use crate::errors::*;
use crate::FeelType;
use dsntk_common::DsntkError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bif {
  Abs,
  After,
  All,
  Any,
  Append,
  Before,
  Ceiling,
  Coincides,
  Concatenate,
  Contains,
  Count,
  Date,
  DateAndTime,
  DayOfWeek,
  DayOfYear,
  Decimal,
  DistinctValues,
  Duration,
  During,
  EndsWith,
  Even,
  Exp,
  FinishedBy,
  Finishes,
  Flatten,
  Floor,
  GetEntries,
  GetValue,
  Includes,
  IndexOf,
  InsertBefore,
  Is,
  ListContains,
  Log,
  LoweCase,
  Matches,
  Max,
  Mean,
  Median,
  Meets,
  MetBy,
  Min,
  Mode,
  Modulo,
  MonthOfYear,
  Not,
  Number,
  Odd,
  Overlaps,
  OverlapsAfter,
  OverlapsBefore,
  Product,
  Remove,
  Replace,
  Reverse,
  Sort,
  Split,
  Sqrt,
  StartedBy,
  Starts,
  StartsWith,
  Stddev,
  String,
  StringLength,
  Sublist,
  Substring,
  SubstringAfter,
  SubstringBefore,
  Sum,
  Time,
  Union,
  UpperCase,
  WeekOfYear,
  YearsAndMonthsDuration,
}

impl FromStr for Bif {
  type Err = DsntkError;
  /// Converts a string into corresponding enumeration variant of [Bif].
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "abs" => Ok(Self::Abs),
      "after" => Ok(Self::After),
      "all" => Ok(Self::All),
      "any" => Ok(Self::Any),
      "append" => Ok(Self::Append),
      "before" => Ok(Self::Before),
      "ceiling" => Ok(Self::Ceiling),
      "coincides" => Ok(Self::Coincides),
      "concatenate" => Ok(Self::Concatenate),
      "contains" => Ok(Self::Contains),
      "count" => Ok(Self::Count),
      "date" => Ok(Self::Date),
      "date and time" => Ok(Self::DateAndTime),
      "day of week" => Ok(Self::DayOfWeek),
      "day of year" => Ok(Self::DayOfYear),
      "decimal" => Ok(Self::Decimal),
      "distinct values" => Ok(Self::DistinctValues),
      "duration" => Ok(Self::Duration),
      "during" => Ok(Self::During),
      "ends with" => Ok(Self::EndsWith),
      "even" => Ok(Self::Even),
      "exp" => Ok(Self::Exp),
      "finished by" => Ok(Self::FinishedBy),
      "finishes" => Ok(Self::Finishes),
      "flatten" => Ok(Self::Flatten),
      "floor" => Ok(Self::Floor),
      "get entries" => Ok(Self::GetEntries),
      "get value" => Ok(Self::GetValue),
      "includes" => Ok(Self::Includes),
      "index of" => Ok(Self::IndexOf),
      "insert before" => Ok(Self::InsertBefore),
      "is" => Ok(Self::Is),
      "list contains" => Ok(Self::ListContains),
      "log" => Ok(Self::Log),
      "lower case" => Ok(Self::LoweCase),
      "matches" => Ok(Self::Matches),
      "max" => Ok(Self::Max),
      "mean" => Ok(Self::Mean),
      "median" => Ok(Self::Median),
      "meets" => Ok(Self::Meets),
      "met by" => Ok(Self::MetBy),
      "min" => Ok(Self::Min),
      "mode" => Ok(Self::Mode),
      "modulo" => Ok(Self::Modulo),
      "month of year" => Ok(Self::MonthOfYear),
      "not" => Ok(Self::Not),
      "number" => Ok(Self::Number),
      "odd" => Ok(Self::Odd),
      "overlaps" => Ok(Self::Overlaps),
      "overlaps after" => Ok(Self::OverlapsAfter),
      "overlaps before" => Ok(Self::OverlapsBefore),
      "product" => Ok(Self::Product),
      "remove" => Ok(Self::Remove),
      "replace" => Ok(Self::Replace),
      "reverse" => Ok(Self::Reverse),
      "sort" => Ok(Self::Sort),
      "split" => Ok(Self::Split),
      "sqrt" => Ok(Self::Sqrt),
      "started by" => Ok(Self::StartedBy),
      "starts" => Ok(Self::Starts),
      "starts with" => Ok(Self::StartsWith),
      "stddev" => Ok(Self::Stddev),
      "string" => Ok(Self::String),
      "string length" => Ok(Self::StringLength),
      "sublist" => Ok(Self::Sublist),
      "substring" => Ok(Self::Substring),
      "substring after" => Ok(Self::SubstringAfter),
      "substring before" => Ok(Self::SubstringBefore),
      "sum" => Ok(Self::Sum),
      "time" => Ok(Self::Time),
      "union" => Ok(Self::Union),
      "upper case" => Ok(Self::UpperCase),
      "week of year" => Ok(Self::WeekOfYear),
      "years and months duration" => Ok(Self::YearsAndMonthsDuration),
      _ => Err(err_unknown_function_name(s)),
    }
  }
}

impl Bif {
  /// Returns a [FeelType] of [Bif].
  pub fn feel_type(&self) -> FeelType {
    match self {
      Bif::Abs => FeelType::Function(vec![FeelType::Number], Box::new(FeelType::Number)),
      Bif::Sqrt => FeelType::Function(vec![FeelType::Number], Box::new(FeelType::Number)),
      _ => {
        // TODO Implement the rest of bif types
        FeelType::Any
      }
    }
  }
}

/// Returns `true` when the specified name is a built-in function name.
pub fn is_built_in_function_name(name: &str) -> bool {
  Bif::from_str(name).is_ok()
}

/// Returns `true` when the specified name is a one of the following
/// built-in function name (date and time literals):
/// - `date`,
/// - `time`,
/// - `date and time`,
/// - `duration`.
pub fn is_built_in_date_time_function_name(name: &str) -> bool {
  if let Ok(built_in_function) = Bif::from_str(name) {
    matches!(built_in_function, Bif::Date | Bif::Time | Bif::DateAndTime | Bif::Duration)
  } else {
    false
  }
}
