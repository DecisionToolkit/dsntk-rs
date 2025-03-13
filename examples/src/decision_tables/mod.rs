//! # Examples of decision tables defined as Unicode or Markdown text
//!
//! File naming convention for horizontal decision tables (rules as rows):
//!
//! ```text
//! ┌────────────── table orientation: h - horizontal (rules as rows)
//! │ ┌──────────── information item name: 0 - absent, 1 - present
//! │ │┌─────────── output label: 0 - absent, 1 - present
//! │ ││┌────────── allowed values: 0 - absent, 1 - present
//! │ │││┌───────── number of input clauses: 0, 1, 2,...
//! │ ││││┌──────── number of output clauses: 1, 2,...
//! │ │││││┌─────── number of annotation clauses: 0, 1, 2,...
//! H_000010
//! ```
//!
//! File naming convention for vertical decision tables (rules as column):
//!
//! ```text
//! ┌────────────── table orientation: v - vertical (rules as columns)
//! │ ┌──────────── information item name: 0 - absent, 1 - present
//! │ │┌─────────── output label: 0 - absent, 1 - present
//! │ ││┌────────── allowed values: 0 - absent, 1 - present
//! │ │││┌───────── number of input clauses: 0, 1, 2,...
//! │ ││││┌──────── number of output clauses: 1, 2,...
//! │ │││││┌─────── number of annotation clauses: 0, 1, 2,...
//! V_000010
//! ```

mod crosstab;
mod horizontal;
mod vertical;

pub use crosstab::*;
pub use horizontal::*;
pub use vertical::*;
