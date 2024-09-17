//! # Built-in examples

// --- example decision model ---

/// Input context for example DMN™ decision model.
pub const EXAMPLE_DM_CTX: &str = include_str!("dm/dm.ctx");

/// Example DMN™ decision model.
pub const EXAMPLE_DM: &str = include_str!("dm/dm.dmn");

// --- example decision table ---

/// Input context for example decision table.
pub const EXAMPLE_DT_CTX: &str = include_str!("dt/dt.ctx");

/// Example decision table in Unicode format.
pub const EXAMPLE_DT: &str = include_str!("dt/dt.dtb");

// --- example FEEL expression ---

/// Input context for example FEEL expression.
pub const EXAMPLE_FE_CTX: &str = include_str!("fe/fe.ctx");

/// Example FEEL expression.
pub const EXAMPLE_FE: &str = include_str!("fe/fe.feel");
