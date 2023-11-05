//! # Examples of valid decision tables

/// **Horizontal decision table**,
/// **no** information item name,
/// **no** output label,
/// **no** input values,
/// **single** output,
/// **no** annotations.
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT LABEL: no
///   INPUT/OUTPUT VALUES: no
///               OUTPUTS: single
///           ANNOTATIONS: no
/// ```
pub const EX_0001: &str = r#"
  ┌───┬────────────┬───────╥──────┐
  │ U │  Customer  │ Order ║      │
  ╞═══╪════════════╪═══════╬══════╡
  │ 1 │ "Business" │  <10  ║ 0.10 │
  ├───┼────────────┼───────╫──────┤
  │ 2 │ "Business" │ >=10  ║ 0.15 │
  ├───┼────────────┼───────╫──────┤
  │ 3 │ "Private"  │   -   ║ 0.05 │
  └───┴────────────┴───────╨──────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal decision table**,
/// **no** information item name,
/// **no** output label,
/// **no** input values,
/// **single** output,
/// **with** annotations.
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0002: &str = r#"
  ┌───┬──────────┬───────╥──────╥─────────────┐
  │ U │ Customer │ Order ║      ║ Description │
  ╞═══╪══════════╪═══════╬══════╬═════════════╡
  │ 1 │"Business"│  <10  ║ 0.10 ║ Small order │
  ├───┼──────────┼───────╫──────╫─────────────┤
  │ 2 │"Business"│ >=10  ║ 0.15 ║ Large order │
  ├───┼──────────┼───────╫──────╫─────────────┤
  │ 3 │"Private" │   -   ║ 0.05 ║ All orders  │
  └───┴──────────┴───────╨──────╨─────────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
/// **no** information item name,
/// **no** output label,
/// **no** input values,
/// **multiple** outputs,
/// **without** annotations.
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0003: &str = r#"
  ┌───┬──────────┬───────╥──────────┬──────────┐
  │ U │ Customer │ Order ║ Discount │ Priority │
  ╞═══╪══════════╪═══════╬══════════╪══════════╡
  │ 1 │"Business"│  <10  ║   0.10   │ "Normal" │
  ├───┼──────────┼───────╫──────────┼──────────┤
  │ 2 │"Business"│ >=10  ║   0.15   │  "High"  │
  ├───┼──────────┼───────╫──────────┼──────────┤
  │ 3 │"Private" │   -   ║   0.05   │  "Low"   │
  └───┴──────────┴───────╨──────────┴──────────┘
% { Customer:"Business",   Order:  -3.23 }, { Discount: 0.10, Priority: "Normal" }
% { Customer:"Business",   Order:   9.00 }, { Discount: 0.10, Priority: "Normal" }
% { Customer:"Business",   Order:  10.00 }, { Discount: 0.15, Priority: "High" }
% { Customer:"Business",   Order: 120.00 }, { Discount: 0.15, Priority: "High" }
% { Customer:"Private",    Order:  -2.34 }, { Discount: 0.05, Priority: "Low" }
% { Customer:"Private",    Order:  10.00 }, { Discount: 0.05, Priority: "Low" }
% { Customer:"Private",    Order: 101.00 }, { Discount: 0.05, Priority: "Low" }
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0004: &str = r#"
  ┌───┬──────────┬───────╥──────────┬──────────╥─────────────┐
  │ U │ Customer │ Order ║ Discount │ Priority ║ Description │
  ╞═══╪══════════╪═══════╬══════════╪══════════╬═════════════╡
  │ 1 │"Business"│  <10  ║   0.10   │ "Normal" ║ Small order │
  ├───┼──────────┼───────╫──────────┼──────────╫─────────────┤
  │ 2 │"Business"│ >=10  ║   0.15   │  "High"  ║ Large order │
  ├───┼──────────┼───────╫──────────┼──────────╫─────────────┤
  │ 3 │"Private" │   -   ║   0.05   │  "Low"   ║ All orders  │
  └───┴──────────┴───────╨──────────┴──────────╨─────────────┘
% { Customer:"Business",   Order:  -3.23 }, { Discount: 0.10, Priority: "Normal" }
% { Customer:"Business",   Order:   9.00 }, { Discount: 0.10, Priority: "Normal" }
% { Customer:"Business",   Order:  10.00 }, { Discount: 0.15, Priority: "High" }
% { Customer:"Business",   Order: 120.00 }, { Discount: 0.15, Priority: "High" }
% { Customer:"Private",    Order:  -2.34 }, { Discount: 0.05, Priority: "Low" }
% { Customer:"Private",    Order:  10.00 }, { Discount: 0.05, Priority: "Low" }
% { Customer:"Private",    Order: 101.00 }, { Discount: 0.05, Priority: "Low" }
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0005: &str = r#"
  ┌───┬───────────┬───────╥──────┐
  │ U │ Customer  │ Order ║      │
  │   ├───────────┼───────╫──────┤
  │   │"Business",│  <10, ║ 0.05,│
  │   │"Private"  │ >=10  ║ 0.10,│
  │   │           │       ║ 0.15 │
  ╞═══╪═══════════╪═══════╬══════╡
  │ 1 │"Business" │  <10  ║ 0.10 │
  ├───┼───────────┼───────╫──────┤
  │ 2 │"Business" │ >=10  ║ 0.15 │
  ├───┼───────────┼───────╫──────┤
  │ 3 │"Private"  │   -   ║ 0.05 │
  └───┴───────────┴───────╨──────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0006: &str = r#"
  ┌───┬───────────┬───────╥──────╥─────────────┐
  │ U │ Customer  │ Order ║      ║ Description │
  │   ├───────────┼───────╫──────╫─────────────┤
  │   │"Business",│  <10, ║ 0.10,║             │
  │   │"Private"  │ >=10  ║ 0.15,║             │
  │   │           │       ║ 0.05 ║             │
  ╞═══╪═══════════╪═══════╬══════╬═════════════╡
  │ 1 │"Business" │  <10  ║ 0.10 ║    Small    │
  ├───┼───────────┼───────╫──────╫─────────────┤
  │ 2 │"Business" │ >=10  ║ 0.15 ║    Large    │
  ├───┼───────────┼───────╫──────╫─────────────┤
  │ 3 │"Private"  │   -   ║ 0.05 ║     All     │
  └───┴───────────┴───────╨──────╨─────────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0007: &str = r#"
  ┌───┬───────────┬───────╥──────────┬──────────┐
  │ U │ Customer  │ Order ║ Discount │ Priority │
  │   ├───────────┼───────╫──────────┼──────────┤
  │   │"Business",│ <10,  ║   0.10,  │"Normal", │
  │   │"Private"  │ >=10  ║   0.15,  │ "High",  │
  │   │           │       ║   0.05   │  "Low"   │
  ╞═══╪═══════════╪═══════╬══════════╪══════════╡
  │ 1 │"Business" │ <10   ║   0.10   │ "Normal" │
  ├───┼───────────┼───────╫──────────┼──────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │  "High"  │
  ├───┼───────────┼───────╫──────────┼──────────┤
  │ 3 │"Private"  │   -   ║   0.05   │  "Low"   │
  └───┴───────────┴───────╨──────────┴──────────┘
% { Customer:"Business",   Order:  -3.23 }, { Discount: 0.10, Priority: "Normal" }
% { Customer:"Business",   Order:   9.00 }, { Discount: 0.10, Priority: "Normal" }
% { Customer:"Business",   Order:  10.00 }, { Discount: 0.15, Priority: "High" }
% { Customer:"Business",   Order: 120.00 }, { Discount: 0.15, Priority: "High" }
% { Customer:"Private",    Order:  -2.34 }, { Discount: 0.05, Priority: "Low" }
% { Customer:"Private",    Order:  10.00 }, { Discount: 0.05, Priority: "Low" }
% { Customer:"Private",    Order: 101.00 }, { Discount: 0.05, Priority: "Low" }
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0008: &str = r#"
  ┌───┬────────────┬───────╥──────────┬───────────╥─────────────┬─────────────┐
  │ U │  Customer  │ Order ║ Discount │ Priority  ║ Description │  Reference  │
  │   ├────────────┼───────╫──────────┼───────────╫─────────────┼─────────────┤
  │   │ "Business",│ <10,  ║   0.10,  │ "Normal", ║             │             │
  │   │ "Private"  │ >=10  ║   0.15,  │  "High",  ║             │             │
  │   │            │       ║   0.05   │  "Low"    ║             │             │
  ╞═══╪════════════╪═══════╬══════════╪═══════════╬═════════════╪═════════════╡
  │ 1 │ "Business" │ <10   ║   0.10   │ "Normal"  ║    Small    │    Ref 1    │
  ├───┼────────────┼───────╫──────────┼───────────╫─────────────┼─────────────┤
  │ 2 │ "Business" │ >=10  ║   0.15   │  "High"   ║    Large    │    Ref 2    │
  ├───┼────────────┼───────╫──────────┼───────────╫─────────────┼─────────────┤
  │ 3 │ "Private"  │   -   ║   0.05   │  "Low"    ║     All     │    Ref 3    │
  └───┴────────────┴───────╨──────────┴───────────╨─────────────┴─────────────┘
% { Customer: "Business",   Order:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer: "Business",   Order:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer: "Business",   Order:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer: "Business",   Order: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer: "Private",    Order:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Private",    Order:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Private",    Order: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Government", Order: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0009: &str = r#"
  ┌───┬──────────┬───────╥──────────┐
  │ U │ Customer │ Order ║ Discount │
  ╞═══╪══════════╪═══════╬══════════╡
  │ 1 │"Business"│ <10   ║   0.10   │
  ├───┼──────────┼───────╫──────────┤
  │ 2 │"Business"│ >=10  ║   0.15   │
  ├───┼──────────┼───────╫──────────┤
  │ 3 │"Private" │   -   ║   0.05   │
  └───┴──────────┴───────╨──────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0010: &str = r#"
  ┌───┬──────────┬───────╥──────────╥─────────────┐
  │ U │ Customer │ Order ║ Discount ║ Description │
  ╞═══╪══════════╪═══════╬══════════╬═════════════╡
  │ 1 │"Business"│ <10   ║   0.10   ║ Small       │
  ├───┼──────────┼───────╫──────────╫─────────────┤
  │ 2 │"Business"│ >=10  ║   0.15   ║ Large       │
  ├───┼──────────┼───────╫──────────╫─────────────┤
  │ 3 │"Private" │   -   ║   0.05   ║ All         │
  └───┴──────────┴───────╨──────────╨─────────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0011: &str = r#"
  ┌───┬──────────┬───────╥─────────────────────┐
  │ U │ Customer │ Order ║    Order options    │
  │   │   type   │ size  ╟──────────┬──────────┤
  │   │          │       ║ Discount │ Priority │
  ╞═══╪══════════╪═══════╬══════════╪══════════╡
  │ 1 │"Business"│ <10   ║   0.10   │ "Normal" │
  ├───┼──────────┼───────╫──────────┼──────────┤
  │ 2 │"Business"│ >=10  ║   0.15   │  "High"  │
  ├───┼──────────┼───────╫──────────┼──────────┤
  │ 3 │"Private" │   -   ║   0.05   │  "Low"   │
  └───┴──────────┴───────╨──────────┴──────────┘
% { Customer type: "Business",   Order size:  -3.23 }, { Discount: 0.10, Priority: "Normal" }
% { Customer type: "Business",   Order size:   9.00 }, { Discount: 0.10, Priority: "Normal" }
% { Customer type: "Business",   Order size:  10.00 }, { Discount: 0.15, Priority: "High" }
% { Customer type: "Business",   Order size: 120.00 }, { Discount: 0.15, Priority: "High" }
% { Customer type: "Private",    Order size:  -2.34 }, { Discount: 0.05, Priority: "Low" }
% { Customer type: "Private",    Order size:  10.00 }, { Discount: 0.05, Priority: "Low" }
% { Customer type: "Private",    Order size: 101.00 }, { Discount: 0.05, Priority: "Low" }
% { Customer type: "Government", Order size: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0012: &str = r#"
  ┌───┬──────────┬───────╥─────────────────────╥─────────────┬───────────┐
  │ U │ Customer │ Order ║    Order options    ║             │           │
  │   │          │       ╟──────────┬──────────╢ Description │ Reference │
  │   │   type   │ size  ║ Discount │ Priority ║             │           │
  ╞═══╪══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
  │ 1 │"Business"│  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
  ├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 2 │"Business"│ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
  ├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 3 │"Private" │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
  └───┴──────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
% { Customer type: "Business",   Order size:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Business",   Order size: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Private",    Order size:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Government", Order size: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0013: &str = r#"
  ┌───┬───────────┬───────╥──────────┐
  │ U │ Customer  │ Order ║ Discount │
  │   ├───────────┼───────╫──────────┤
  │   │"Business",│  <10, ║   0.10,  │
  │   │"Private"  │ >=10  ║   0.15,  │
  │   │           │       ║   0.05   │
  ╞═══╪═══════════╪═══════╬══════════╡
  │ 1 │"Business" │  <10  ║   0.10   │
  ├───┼───────────┼───────╫──────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │
  ├───┼───────────┼───────╫──────────┤
  │ 3 │"Private"  │   -   ║   0.05   │
  └───┴───────────┴───────╨──────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0014: &str = r#"
  ┌───┬───────────┬───────╥──────────╥─────────────┬───────────┐
  │ U │ Customer  │ Order ║ Discount ║ Description │ Reference │
  │   ├───────────┼───────╫──────────╫─────────────┼───────────┤
  │   │"Business",│  <10, ║   0.10,  ║             │           │
  │   │"Private"  │ >=10  ║   0.15,  ║             │           │
  │   │           │       ║   0.05   ║             │           │
  ╞═══╪═══════════╪═══════╬══════════╬═════════════╪═══════════╡
  │ 1 │"Business" │  <10  ║   0.10   ║ Small order │   Ref 1   │
  ├───┼───────────┼───────╫──────────╫─────────────┼───────────┤
  │ 2 │"Business" │ >=10  ║   0.15   ║ Large order │   Ref 2   │
  ├───┼───────────┼───────╫──────────╫─────────────┼───────────┤
  │ 3 │"Private"  │   -   ║   0.05   ║ All orders  │   Ref 3   │
  └───┴───────────┴───────╨──────────╨─────────────┴───────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0015: &str = r#"
  ┌───┬───────────┬───────╥─────────────────────┐
  │ U │           │       ║    Order options    │
  │   │ Customer  │ Order ╟──────────┬──────────┤
  │   │   type    │ size  ║ Discount │ Priority │
  │   ├───────────┼───────╫──────────┼──────────┤
  │   │"Business",│  <10, ║   0.10,  │"Normal", │
  │   │"Private"  │ >=10  ║   0.15,  │ "High",  │
  │   │           │       ║   0.05   │ "Low"    │
  ╞═══╪═══════════╪═══════╬══════════╪══════════╡
  │ 1 │"Business" │  <10  ║   0.10   │ "Normal" │
  ├───┼───────────┼───────╫──────────┼──────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │  "High"  │
  ├───┼───────────┼───────╫──────────┼──────────┤
  │ 3 │"Private"  │   -   ║   0.05   │  "Low"   │
  └───┴───────────┴───────╨──────────┴──────────┘
% { Customer type: "Business",   Order size:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Business",   Order size: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Private",    Order size:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Government", Order size: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0016: &str = r#"
  ┌───┬───────────┬───────╥─────────────────────╥─────────────┬───────────┐
  │ U │           │       ║    Order options    ║             │           │
  │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
  │   │   type    │ size  ║ Discount │ Priority ║             │           │
  │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │   │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
  │   │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
  │   │           │       ║   0.05   │ "Low"    ║             │           │
  ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
  │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
  ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
  ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
  └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
% { Customer type: "Business",   Order size:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Business",   Order size: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Private",    Order size:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Government", Order size: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0017: &str = r#"
  ┌─────────────────────────────┐
  │ Discount                    │
  ├───┬──────────┬───────╥──────┤
  │ U │ Customer │ Order ║      │
  ╞═══╪══════════╪═══════╬══════╡
  │ 1 │"Business"│  <10  ║ 0.10 │
  ├───┼──────────┼───────╫──────┤
  │ 2 │"Business"│ >=10  ║ 0.15 │
  ├───┼──────────┼───────╫──────┤
  │ 3 │"Private" │   -   ║ 0.05 │
  └───┴──────────┴───────╨──────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0018: &str = r#"
  ┌──────────────┐
  │   Discount   │
  ├───┬──────────┼───────╥──────╥─────────────┐
  │ U │ Customer │ Order ║      ║ Description │
  ╞═══╪══════════╪═══════╬══════╬═════════════╡
  │ 1 │"Business"│  <10  ║ 0.10 ║ Small order │
  ├───┼──────────┼───────╫──────╫─────────────┤
  │ 2 │"Business"│ >=10  ║ 0.15 ║ Large order │
  ├───┼──────────┼───────╫──────╫─────────────┤
  │ 3 │"Private" │   -   ║ 0.05 ║ All orders  │
  └───┴──────────┴───────╨──────╨─────────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0019: &str = r#"
  ┌───────────────────────┐
  │             Order     │
  │             options   │
  ├───┬──────────┬───────╥┴─────────┬──────────┐
  │ U │ Customer │ Order ║ Discount │ Priority │
  ╞═══╪══════════╪═══════╬══════════╪══════════╡
  │ 1 │"Business"│  <10  ║   0.10   │ "Normal" │
  ├───┼──────────┼───────╫──────────┼──────────┤
  │ 2 │"Business"│ >=10  ║   0.15   │  "High"  │
  ├───┼──────────┼───────╫──────────┼──────────┤
  │ 3 │"Private" │   -   ║   0.05   │  "Low"   │
  └───┴──────────┴───────╨──────────┴──────────┘
% { Customer: "Business",   Order:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer: "Business",   Order:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer: "Business",   Order:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer: "Business",   Order: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer: "Private",    Order:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Private",    Order:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Private",    Order: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Government", Order: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0020: &str = r#"
  ┌───────────────────────┐
  │             Order     │
  │             options   │
  ├───┬──────────┬───────╥┴─────────┬──────────╥─────────────┬───────────┐
  │ U │ Customer │ Order ║ Discount │ Priority ║ Description │ Reference │
  ╞═══╪══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
  │ 1 │"Business"│  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
  ├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 2 │"Business"│ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
  ├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 3 │"Private" │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
  └───┴──────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
% { Customer: "Business",   Order:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer: "Business",   Order:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer: "Business",   Order:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer: "Business",   Order: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer: "Private",    Order:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Private",    Order:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Private",    Order: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Government", Order: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0021: &str = r#"
  ┌───────────────┐
  │   Discount    │
  ├───┬───────────┼───────╥──────┐
  │ U │ Customer  │ Order ║      │
  │   ├───────────┼───────╫──────┤
  │   │"Business",│  <10, ║ 0.10,│
  │   │"Private"  │ >=10  ║ 0.15,│
  │   │           │       ║ 0.05 │
  ╞═══╪═══════════╪═══════╬══════╡
  │ 1 │"Business" │  <10  ║ 0.10 │
  ├───┼───────────┼───────╫──────┤
  │ 2 │"Business" │ >=10  ║ 0.15 │
  ├───┼───────────┼───────╫──────┤
  │ 3 │"Private"  │   -   ║ 0.05 │
  └───┴───────────┴───────╨──────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0022: &str = r#"
  ┌───────────────┐
  │   Discount    │
  ├───┬───────────┼───────╥──────╥─────────────┐
  │ U │ Customer  │ Order ║      ║ Description │
  │   ├───────────┼───────╫──────╫─────────────┤
  │   │"Business",│  <10, ║ 0.10,║             │
  │   │"Private"  │ >=10  ║ 0.15,║             │
  │   │           │       ║ 0.05 ║             │
  ╞═══╪═══════════╪═══════╬══════╬═════════════╡
  │ 1 │"Business" │  <10  ║ 0.10 ║ Small order │
  ├───┼───────────┼───────╫──────╫─────────────┤
  │ 2 │"Business" │ >=10  ║ 0.15 ║ Large order │
  ├───┼───────────┼───────╫──────╫─────────────┤
  │ 3 │"Private"  │   -   ║ 0.05 ║ All orders  │
  └───┴───────────┴───────╨──────╨─────────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0023: &str = r#"
  ┌─────────────────────────────────────────────┐
  │ Order properties                            │
  ├───┬───────────┬───────╥──────────┬──────────┤
  │ U │ Customer  │ Order ║ Discount │ Priority │
  │   ├───────────┼───────╫──────────┼──────────┤
  │   │"Business",│ <10,  ║   0.10,  │"Normal", │
  │   │"Private"  │ >=10  ║   0.15,  │ "High",  │
  │   │           │       ║   0.05   │  "Low"   │
  ╞═══╪═══════════╪═══════╬══════════╪══════════╡
  │ 1 │"Business" │ <10   ║   0.10   │ "Normal" │
  ├───┼───────────┼───────╫──────────┼──────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │  "High"  │
  ├───┼───────────┼───────╫──────────┼──────────┤
  │ 3 │"Private"  │   -   ║   0.05   │  "Low"   │
  └───┴───────────┴───────╨──────────┴──────────┘
% { Customer: "Business",   Order:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer: "Business",   Order:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer: "Business",   Order:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer: "Business",   Order: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer: "Private",    Order:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Private",    Order:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Private",    Order: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Government", Order: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0024: &str = r#"
  ┌────────────────────────────────────────────────┐
  │ Order properties                               │
  ├───┬───────────┬───────╥──────────┬──────────╥──┴────────┐
  │ U │ Customer  │ Order ║ Discount │ Priority ║ Reference │
  │   ├───────────┼───────╫──────────┼──────────╫───────────┤
  │   │"Business",│ <10,  ║   0.10,  │"Normal", ║           │
  │   │"Private"  │ >=10  ║   0.15,  │ "High",  ║           │
  │   │           │       ║   0.05   │ "Low"    ║           │
  ╞═══╪═══════════╪═══════╬══════════╪══════════╬═══════════╡
  │ 1 │"Business" │ <10   ║   0.10   │ "Normal" ║ Ref 4     │
  ├───┼───────────┼───────╫──────────┼──────────╫───────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Ref 3     │
  ├───┼───────────┼───────╫──────────┼──────────╫───────────┤
  │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ Ref 2     │
  └───┴───────────┴───────╨──────────┴──────────╨───────────┘
% { Customer: "Business",   Order:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer: "Business",   Order:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer: "Business",   Order:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer: "Business",   Order: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer: "Private",    Order:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Private",    Order:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Private",    Order: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer: "Government", Order: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0025: &str = r#"
  ┌────────────────────┐
  │ Discount           │
  ├───┬──────────┬─────┴─╥──────────┐
  │ U │ Customer │ Order ║ Discount │
  ╞═══╪══════════╪═══════╬══════════╡
  │ 1 │"Business"│  <10  ║   0.10   │
  ├───┼──────────┼───────╫──────────┤
  │ 2 │"Business"│ >=10  ║   0.15   │
  ├───┼──────────┼───────╫──────────┤
  │ 3 │"Private" │   -   ║   0.05   │
  └───┴──────────┴───────╨──────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0026: &str = r#"
  ┌────────────────────┐
  │ Discount           │
  ├───┬──────────┬─────┴─╥──────────╥─────────────┐
  │ U │ Customer │ Order ║ Discount ║ Description │
  ╞═══╪══════════╪═══════╬══════════╬═════════════╡
  │ 1 │"Business"│  <10  ║   0.10   ║ Small order │
  ├───┼──────────┼───────╫──────────╫─────────────┤
  │ 2 │"Business"│ >=10  ║   0.15   ║ Large order │
  ├───┼──────────┼───────╫──────────╫─────────────┤
  │ 3 │"Private" │   -   ║   0.05   ║ All orders  │
  └───┴──────────┴───────╨──────────╨─────────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0027: &str = r#"
  ┌──────────────────┐
  │ Order properties │
  ├───┬──────────┬───┴───╥─────────────────────┐
  │ U │ Customer │ Order ║  Order properties   │
  │   │   type   │ size  ╟──────────┬──────────┤
  │   │          │       ║ Discount │ Priority │
  ╞═══╪══════════╪═══════╬══════════╪══════════╡
  │ 1 │"Business"│  <10  ║   0.10   │ "Normal" │
  ├───┼──────────┼───────╫──────────┼──────────┤
  │ 2 │"Business"│ >=10  ║   0.15   │  "High"  │
  ├───┼──────────┼───────╫──────────┼──────────┤
  │ 3 │"Private" │   -   ║   0.05   │  "Low"   │
  └───┴──────────┴───────╨──────────┴──────────┘
% { Customer type: "Business",   Order size:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Business",   Order size: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Private",    Order size:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Government", Order size: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0028: &str = r#"
  ┌──────────────────┐
  │ Order properties │
  ├───┬──────────┬───┴───╥─────────────────────╥─────────────┬───────────┐
  │ U │ Customer │ Order ║  Order properties   ║             │           │
  │   │   type   │ size  ╟──────────┬──────────╢ Description │ Reference │
  │   │          │       ║ Discount │ Priority ║             │           │
  ╞═══╪══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
  │ 1 │"Business"│  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
  ├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 2 │"Business"│ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
  ├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 3 │"Private" │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
  └───┴──────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
% { Customer type: "Business",   Order size:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Business",   Order size: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Private",    Order size:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Government", Order size: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0029: &str = r#"
  ┌──────────────────────────────────┐
  │ Discount                         │
  ├───┬───────────┬───────╥──────────┤
  │ U │ Customer  │ Order ║ Discount │
  │   ├───────────┼───────╫──────────┤
  │   │"Business",│  <10, ║   0.10,  │
  │   │"Private"  │ >=10  ║   0.15,  │
  │   │           │       ║   0.05   │
  ╞═══╪═══════════╪═══════╬══════════╡
  │ 1 │"Business" │  <10  ║   0.10   │
  ├───┼───────────┼───────╫──────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │
  ├───┼───────────┼───────╫──────────┤
  │ 3 │"Private"  │   -   ║   0.05   │
  └───┴───────────┴───────╨──────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0030: &str = r#"
  ┌─────────────────────────────────────┐
  │ Discount                            │
  ├───┬───────────┬───────╥──────────╥──┴────────┐
  │ U │ Customer  │ Order ║ Discount ║ Reference │
  │   ├───────────┼───────╫──────────╫───────────┤
  │   │"Business",│  <10, ║   0.10,  ║           │
  │   │"Private"  │ >=10  ║   0.15,  ║           │
  │   │           │       ║   0.05   ║           │
  ╞═══╪═══════════╪═══════╬══════════╬═══════════╡
  │ 1 │"Business" │  <10  ║   0.10   ║     Ref 0 │
  ├───┼───────────┼───────╫──────────╫───────────┤
  │ 2 │"Business" │ >=10  ║   0.15   ║     Ref 1 │
  ├───┼───────────┼───────╫──────────╫───────────┤
  │ 3 │"Private"  │   -   ║   0.05   ║     Ref 2 │
  └───┴───────────┴───────╨──────────╨───────────┘
% { Customer:"Business",   Order:  -3.23 }, 0.10
% { Customer:"Business",   Order:   9.00 }, 0.10
% { Customer:"Business",   Order:  10.00 }, 0.15
% { Customer:"Business",   Order: 120.00 }, 0.15
% { Customer:"Private",    Order:  -2.34 }, 0.05
% { Customer:"Private",    Order:  10.00 }, 0.05
% { Customer:"Private",    Order: 101.00 }, 0.05
% { Customer:"Government", Order:  10.00 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0031: &str = r#"
  ┌─────────────────────────────────────┐
  │ Order options                       │
  ├───┬───────────┬───────╥─────────────┴───────┐
  │ U │           │       ║    Order options    │
  │   │ Customer  │ Order ╟──────────┬──────────┤
  │   │   type    │ size  ║ Discount │ Priority │
  │   ├───────────┼───────╫──────────┼──────────┤
  │   │"Business",│  <10, ║   0.10,  │"Normal", │
  │   │"Private"  │ >=10  ║   0.15,  │ "High",  │
  │   │           │       ║   0.05   │ "Low"    │
  ╞═══╪═══════════╪═══════╬══════════╪══════════╡
  │ 1 │"Business" │  <10  ║   0.10   │ "Normal" │
  ├───┼───────────┼───────╫──────────┼──────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │ "High"   │
  ├───┼───────────┼───────╫──────────┼──────────┤
  │ 3 │"Private"  │   -   ║   0.05   │ "Low"    │
  └───┴───────────┴───────╨──────────┴──────────┘
% { Customer type: "Business",   Order size:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Business",   Order size: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Private",    Order size:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Government", Order size: 300.01 }, null
"#;

/// **Horizontal** decision table,
///
/// ```text
///           ORIENTATION: horizontal
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0032: &str = r#"
  ┌─────────────────────────────────────┐
  │ Order options                       │
  ├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
  │ U │           │       ║    Order options    ║             │           │
  │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
  │   │   type    │ size  ║ Discount │ Priority ║             │           │
  │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │   │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
  │   │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
  │   │           │       ║   0.05   │ "Low"    ║             │           │
  ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
  │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
  ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
  ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 3 │"Private"  │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
  └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
% { Customer type: "Business",   Order size:  -3.23 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:   9.00 }, { "Discount": 0.10, "Priority": "Normal" }
% { Customer type: "Business",   Order size:  10.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Business",   Order size: 120.00 }, { "Discount": 0.15, "Priority": "High" }
% { Customer type: "Private",    Order size:  -2.34 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size:  10.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Private",    Order size: 101.00 }, { "Discount": 0.05, "Priority": "Low" }
% { Customer type: "Government", Order size: 300.01 }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0033: &str = r#"
  ┌─────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age   ║     <25       │ [25..60] │      >60      │
  ├─────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │                 ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├─────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │        U        ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0034: &str = r#"
  ┌─────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age   ║     <25       │ [25..60] │      >60      │
  ├─────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │                 ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ╞═════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Reference       ║ Ref0 │  Ref1  │   Ref2   │  Ref3  │ Ref4 │
  ├─────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │        U        ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0035: &str = r#"
  ┌───────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Special Discount      ║  10  │    7   │     6    │    4   │  0   │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 7}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 4}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0036: &str = r#"
  ┌───────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Special Discount      ║  10  │    4   │     5    │    3   │  0   │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance ║ No   │   No   │    No    │   No   │ Yes  │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 4}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 3}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0037: &str = r#"
  ┌───────────────────────┬─────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         │ <25, [25..60], >60  ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │                       │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0038: &str = r#"
  ┌───────────────────────┬─────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │                       │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance │                     ║ No   │   No   │    No    │   No   │ Yes  │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             │                     ║  R0  │   R1   │    R2    │   R3   │  R4  │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0039: &str = r#"
  ┌───────────────────────┬─────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Special Discount      │     0,3,5,6,10      ║  10  │    6   │     5    │    3   │  0   │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║   1  │    2   │     3    │    4   │  5   │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 3}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0040: &str = r#"
  ┌───────────────────────┬─────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Special Discount      │     0, 5, 10        ║  10  │    5   │     5    │    5   │  0   │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance │                     ║ No   │   No   │    No    │   No   │ Yes  │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             │                     ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0041: &str = r#"
  ┌───────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0042: &str = r#"
  ┌───────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance ║ No   │   No   │    No    │   No   │ Yes  │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0043: &str = r#"
  ┌─────────────────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age                   ║     <25       │ [25..60] │      >60      │
  ├─────────────────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history                 ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════╤═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Sell    │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  │         ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ options │ Special Discount      ║  10  │    5   │     5    │   5    │  0   │
  ├─────────┴───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │                U                ║   1  │    2   │     3    │   4    │  5   │
  └─────────────────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0044: &str = r#"
  ┌─────────────────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age                   ║     <25       │ [25..60] │      >60      │
  ├─────────────────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history                 ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════╤═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Sell    │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  │         ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ options │ Special Discount      ║  10  │    5   │     5    │    5   │  0   │
  ╞═════════╧═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance           ║ No   │   No   │    No    │   No   │ Yes  │
  ├─────────────────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference                       ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├─────────────────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ U                               ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0045: &str = r#"
  ┌───────────────────────┬─────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0046: &str = r#"
  ┌───────────────────────┬─────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         │ <25, [25..60], >60  ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance │                     ║ No   │   No   │    No    │   No   │ Yes  │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             │                     ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0047: &str = r#"
  ┌─────────────────────────────────┬─────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age                   │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├─────────────────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history                 │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════╤═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Sell    │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  │         ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ options │ Special Discount      │     0, 5, 10        ║  10  │    5   │     5    │    5   │  0   │
  ├─────────┴───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ U                               │                     ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0048: &str = r#"
  ┌─────────────────────────────────┬─────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age                   │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├─────────────────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history                 │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════╤═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Sell    │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  │         ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ options │ Special Discount      │     0, 5, 10        ║  10  │    5   │     5    │    5   │  0   │
  ╞═════════╧═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance           │                     ║ No   │   No   │    No    │   No   │ Yes  │
  ├─────────────────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference                       │                     ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├─────────────────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ U                               │                     ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0049: &str = r#"
  ┌──────────────────────────────┐
  │ Applicant risk rating        │
  ├───────────────────────╥──────┴────────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │                       ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0050: &str = r#"
  ┌──────────────────────────────┐
  │ Applicant risk rating        │
  ├───────────────────────╥──────┴────────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │                       ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance ║ No   │   No   │    No    │   No   │ Yes  │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0051: &str = r#"
  ┌──────────────────────────────┐
  │ Sell options                 │
  ├───────────────────────╥──────┴────────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Special Discount      ║  10  │    5   │     5    │    5   │  0   │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0052: &str = r#"
  ┌──────────────────────────────────────────────────┐
  │ Sell options                                     │
  ├───────────────────────╥───────────────┬──────────┼───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Special Discount      ║  10  │    5   │     5    │    5   │  0   │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance ║  No  │   No   │    No    │   No   │ Yes  │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0053: &str = r#"
  ┌────────────────────────────────────────────────────────────────────────┐
  │ Applicant risk rating                                                  │
  ├───────────────────────┬─────────────────────╥───────────────┬──────────┼───────────────┐
  │ Applicant age         │ <25, [25..60], >60  ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │                       │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0054: &str = r#"
  ┌──────────────────────────────────────────────────────────────────────────────────────┐
  │ Applicant risk rating                                                                │
  ├───────────────────────┬─────────────────────╥───────────────┬──────────┬─────────────┴─┐
  │ Applicant age         │ <25, [25..60], >60  ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │                       │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance │                     ║ No   │   No   │    No    │   No   │ Yes  │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             │                     ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0055: &str = r#"
  ┌────────────────────────────────────────────────────────────────────────┐
  │ Sell options                                                           │
  ├───────────────────────┬─────────────────────╥───────────────┬──────────┼───────────────┐
  │ Applicant age         │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Special Discount      │     0, 5, 10        ║  10  │    5   │     5    │    5   │  0   │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║   1  │    2   │     3    │    4   │  5   │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0056: &str = r#"
  ┌────────────────────────────────────────────────────────────────────────┐
  │ Sell options                                                           │
  ├───────────────────────┬─────────────────────╥───────────────┬──────────┼───────────────┐
  │ Applicant age         │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Special Discount      │     0, 5, 10        ║  10  │    5   │     5    │    5   │  0   │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance │                     ║ No   │   No   │    No    │   No   │ Yes  │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             │                     ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0057: &str = r#"
  ┌──────────────────────────────────┐
  │ Applicant risk rating            │
  ├───────────────────────╥──────────┴────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0058: &str = r#"
  ┌──────────────────────────────────┐
  │ Applicant risk rating            │
  ├───────────────────────╥──────────┴────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance ║ No   │   No   │    No    │   No   │ Yes  │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             ║ Ref0 │  Ref1  │   Ref2   │  Ref3  │ Ref4 │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0059: &str = r#"
  ┌────────────────────────────────────────────────────────────┐
  │ Sell options                                               │
  ├─────────────────────────────────╥───────────────┬──────────┼───────────────┐
  │ Applicant age                   ║     <25       │ [25..60] │      >60      │
  ├─────────────────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history                 ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════╤═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Sell    │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  │         ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ options │ Special Discount      ║  10  │    5   │     5    │    5   │  0   │
  ├─────────┴───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ U                               ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0060: &str = r#"
  ┌────────────────────────────────────────────────────────────┐
  │ Sell options                                               │
  ├─────────────────────────────────╥───────────────┬──────────┼───────────────┐
  │ Applicant age                   ║     <25       │ [25..60] │      >60      │
  ├─────────────────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history                 ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════╤═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Sell    │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  │         ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ options │ Special Discount      ║  10  │    5   │     5    │    5   │  0   │
  ╞═════════╧═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance           ║ No   │   No   │    No    │   No   │ Yes  │
  ├─────────────────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference                       ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├─────────────────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ U                               ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0061: &str = r#"
  ┌──────────────────────────────┐
  │ Applicant risk rating        │
  ├───────────────────────┬──────┴──────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
pub const EX_0062: &str = r#"
  ┌────────────────────────────────────────────────────────────────────────────────────────┐
  │ Applicant risk rating                                                                  │
  ├───────────────────────┬─────────────────────╥───────────────┬──────────┬───────────────┤
  │ Applicant age         │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├───────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ╞═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance │                     ║ No   │   No   │    No    │   No   │ Yes  │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference             │                     ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           │                     ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, "Low"
% { Applicant age: 24, Medical history: "bad"  }, "Medium"
% { Applicant age: 25, Medical history: "good" }, "Medium"
% { Applicant age: 25, Medical history: "bad"  }, "Medium"
% { Applicant age: 60, Medical history: "good" }, "Medium"
% { Applicant age: 60, Medical history: "bad"  }, "Medium"
% { Applicant age: 61, Medical history: "good" }, "Medium"
% { Applicant age: 61, Medical history: "bad"  }, "High"
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0063: &str = r#"
┌──────────────────────────────────────────────────────────────────────────────────┐
│ Sell options                                                                     │
├─────────────────────────────────┬─────────────────────╥───────────────┬──────────┼───────────────┐
│ Applicant age                   │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
├─────────────────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
│ Medical history                 │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
╞═════════╤═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
│ Sell    │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
│         ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
│ options │ Special Discount      │     0, 5, 10        ║  10  │    5   │     5    │    5   │  0   │
├─────────┴───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
│ U                               │                     ║  1   │    2   │     3    │   4    │   5  │
└─────────────────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Vertical** decision table,
///
/// ```text
///           ORIENTATION: **vertical**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
pub const EX_0064: &str = r#"
  ┌──────────────────────────────────────────────────────────────────────────────────┐
  │ Sell options                                                                     │
  ├─────────────────────────────────┬─────────────────────╥───────────────┬──────────┼───────────────┐
  │ Applicant age                   │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├─────────────────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history                 │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════╤═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Sell    │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  │         ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ options │ Special Discount      │     0, 5, 10        ║  10  │    5   │     5    │    5   │  0   │
  ╞═════════╧═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance           │                     ║ No   │   No   │    No    │   No   │ Yes  │
  ├─────────────────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference                       │                     ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├─────────────────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ U                               │                     ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 5}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0065: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0066: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0067: &str = r#"
  ┌────────────────────╥─────────────────────────────────┐
  │                    ║            Customer             │
  ├────────────────────╫──────────┬─────────┬────────────┤
  │   Discount,        ║          │         │            │
  │   Priority         ║"Business"│"Private"│"Government"│
  ╞═════════════╤══════╬══════════╪═════════╪════════════╡
  │             │  <10 ║   0.05,  │    0,   │    0.15,   │
  │             │      ║ "Normal" │  "Low"  │   "High"   │
  │ Order size  ├──────╫──────────┼─────────┼────────────┤
  │             │ >=10 ║   0.10,  │    0,   │    0.15,   │
  │             │      ║  "High"  │ "Normal"│   "High"   │
  └─────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0068: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0069: &str = r#"
  ┌──────────────────────────╥─────────────────────────────────┐
  │                          ║            Customer             │
  │                          ╟─────────────────────────────────┤
  │                          ║"Business","Private","Government"│
  ├──────────────────────────╫──────────┬─────────┬────────────┤
  │      0.05, 0, 0.15       ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╤══════╬══════════╪═════════╪════════════╡
  │            │ <10, │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size │      ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0070: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0071: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0072: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0073: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0074: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0075: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
///
/// Postponed until annotations for crosstab are in specification.
// /// ```
pub const EX_0076: &str = r#"
// "#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0077: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0078: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0079: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: no
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0080: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0081: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0082: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0083: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0084: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0085: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0086: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0087: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: no
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0088: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0089: &str = r#"
  ┌─────────────────────────────────────────────────────┐
  │ Discount                                            │
  ├───────────────────╥─────────────────────────────────┤
  │                   ║            Customer             │
  │     Discount      ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0090: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0091: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: no
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0092: &str = r#" "#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: no
/// ```
pub const EX_0093: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: single
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0094: &str = r#""#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: no
/// ```
pub const EX_0095: &str = r#"
  ┌───────────────────╥─────────────────────────────────┐
  │                   ║            Customer             │
  │                   ╟──────────┬─────────┬────────────┤
  │                   ║"Business"│"Private"│"Government"│
  ╞════════════╤══════╬══════════╪═════════╪════════════╡
  │            │  <10 ║   0.05   │    0    │    0.15    │
  │ Order size ├──────╫──────────┼─────────┼────────────┤
  │            │ >=10 ║   0.10   │    0    │    0.15    │
  └────────────┴──────╨──────────┴─────────┴────────────┘
"#;

/// **Crosstab** decision table,
///
/// ```text
///           ORIENTATION: **crosstab**
/// INFORMATION ITEM NAME: yes
///          OUTPUT-LABEL: yes
///   INPUT-OUTPUT-VALUES: yes
///                OUTPUT: multiple
///           ANNOTATIONS: yes
/// ```
/// Postponed until annotations for crosstab are in specification.
pub const EX_0096: &str = r#""#;

/// **Crosstab** decision table,
/// **with** information item name,
/// **with** output label,
/// **no** input values,
/// **single** output,
/// **no** annotations,
/// **three inputs**.
///
/// ```text
///           ORIENTATION: crosstab
/// INFORMATION ITEM NAME: yes
///          OUTPUT LABEL: yes
///   INPUT/OUTPUT VALUES: no
///               OUTPUTS: single
///           ANNOTATIONS: no
///                 OTHER: three inputs
/// ```
pub const EX_0097: &str = r#"
  ┌─────────────────────────────────────────────────────────────┐
  │ Discount                                                    │
  ├───────────────────╥─────────────────────────────────────────┤
  │                   ║          Customer, Delivery             │
  │                   ╟──────────┬─────────────────┬────────────┤
  │     Discount      ║"Business"│    "Private"    │"Government"│
  │                   ╟──────────┼──────────┬──────┼────────────┤
  │                   ║    -     │ same day │ slow │     -      │
  ╞════════════╤══════╬══════════╪══════════╪══════╪════════════╡
  │            │ <10  ║   0.05   │    0     │ 0.05 │    0.15    │
  │ Order size ├──────╫──────────┼──────────┼──────┼────────────┤
  │            │ >=10 ║   0.10   │    0     │ 0.05 │    0.15    │
  └────────────┴──────╨──────────┴──────────┴──────┴────────────┘
"#;

/// General horizontal decision table.
pub const EX_0100: &str = r#"
  ┌───────────────────────────┐
  │ information item name     │
  ├───┬────────────────────┬──┴─────────────────╥────────────────────┐
  │ C │ input expression 1 │ input expression 2 ║    output label    │
  │   ├────────────────────┼────────────────────╫────────────────────┤
  │   │ input value 1a,    │ input value 2a,    ║ output value 1a,   │
  │   │   input value 1b   │   input value 2b   ║   output value 1b  │
  ╞═══╪════════════════════╪════════════════════╬════════════════════╡
  │ 1 │                    │  input entry 2.1   ║  output entry 1.1  │
  ├───┤  input entry 1.1   ├────────────────────╫────────────────────┤
  │ 2 │                    │  input entry 2.2   ║  output entry 1.2  │
  ├───┼────────────────────┼────────────────────╫────────────────────┤
  │ 3 │  input entry 1.2   │         -          ║  output entry 1.3  │
  ├───┼────────────────────┼────────────────────╫────────────────────┤
  │ 4 │  input entry 1.3   │  input entry 2.3   ║  output entry 1.4  │
  └───┴────────────────────┴────────────────────╨────────────────────┘
"#;

/// General horizontal decision table with multiple outputs.
pub const EX_0101: &str = r#"
  ┌───────────────────────────────────────────────────────────────────────────────────────┐
  │ information item name                                                                 │
  ├───┬────────────────────┬────────────────────╥─────────────────────────────────────────┤
  │ U │                    │                    ║               output label              │
  │   │ input expression 1 │ input expression 2 ╟────────────────────┬────────────────────┤
  │   │                    │                    ║ output component 1 │ output component 1 │
  │   ├────────────────────┼────────────────────╫────────────────────┼────────────────────┤
  │   │ input value 1a,    │ input value 2a,    ║ output value 1a,   │ output value 2a,   │
  │   │   input value 1b   │   input value 2b   ║   output value 1b  │   output value 2b  │
  ╞═══╪════════════════════╪════════════════════╬════════════════════╪════════════════════╡
  │ 1 │                    │  input entry 2.1   ║  output entry 1.1  │  output entry 2.1  │
  ├───┤  input entry 1.1   ├────────────────────╫────────────────────┼────────────────────┤
  │ 2 │                    │  input entry 2.2   ║  output entry 1.2  │  output entry 2.2  │
  ├───┼────────────────────┼────────────────────╫────────────────────┼────────────────────┤
  │ 3 │                    │  input entry 2.3   ║  output entry 1.3  │  output entry 2.3  │
  ├───┤  input entry 1.2   ├────────────────────╫────────────────────┼────────────────────┤
  │ 4 │                    │  input entry 2.4   ║  output entry 1.4  │  output entry 2.4  │
  ├───┼────────────────────┼────────────────────╫────────────────────┼────────────────────┤
  │ 5 │  input entry 1.4   │  input entry 2.5   ║  output entry 1.5  │  output entry 2.5  │
  └───┴────────────────────┴────────────────────╨────────────────────┴────────────────────┘
"#;

/// General horizontal decision table with annotations.
pub const EX_0102: &str = r#"
  ┌───────────────────────────┐
  │   information item name   │
  ├───┬────────────────────┬──┴─────────────────╥────────────────────╥──────────────────────┬──────────────────────┐
  │ U │ input expression 1 │ input expression 2 ║    output label    ║     annotation 1     │     annotation 2     │
  │   ├────────────────────┼────────────────────╫────────────────────╫──────────────────────┼──────────────────────┤
  │   │ input value 1a,    │ input value 2a,    ║ output value 1a,   ║                      │                      │
  │   │   input value 1b   │   input value 2b   ║   output value 1b  ║                      │                      │
  ╞═══╪════════════════════╪════════════════════╬════════════════════╬══════════════════════╪══════════════════════╡
  │ 1 │                    │  input entry 2.1   ║  output entry 1.1  ║ annotation entry 1.1 │ annotation entry 2.1 │
  ├───┤  input entry 1.1   ├────────────────────╫────────────────────╫──────────────────────┼──────────────────────┤
  │ 2 │                    │  input entry 2.2   ║  output entry 1.2  ║ annotation entry 1.2 │ annotation entry 2.2 │
  ├───┼────────────────────┼────────────────────╫────────────────────╫──────────────────────┼──────────────────────┤
  │ 3 │  input entry 1.2   │         -          ║  output entry 1.3  ║ annotation entry 1.3 │ annotation entry 2.3 │
  ├───┼────────────────────┼────────────────────╫────────────────────╫──────────────────────┼──────────────────────┤
  │ 4 │  input entry 1.3   │  input entry 2.3   ║  output entry 1.4  ║ annotation entry 1.4 │ annotation entry 2.4 │
  └───┴────────────────────┴────────────────────╨────────────────────╨──────────────────────┴──────────────────────┘
"#;

/// General vertical decision table.
pub const EX_0103: &str = r#"
  ┌───────────────────────────┐
  │   information item name   │
  ├────────────────────┬──────┴─────────────╥─────────────────────────────────────┬──────────────────┐
  │ input expression 1 │ input value 1a,    ║            input entry 1.1          │ input entry 1.2  │
  │                    │ input value 1b     ║                                     │                  │
  ├────────────────────┼────────────────────╫──────────────────┬──────────────────┼──────────────────┤
  │ input expression 2 │ input value 2a,    ║ input entry 2.1  │ input entry 2.2  │         -        │
  │                    │ input value 2b     ║                  │                  │                  │
  ╞════════════════════╪════════════════════╬══════════════════╪══════════════════╪══════════════════╡
  │ output label       │ output value 1a,   ║ output entry 1.1 │ output entry 1.2 │ output entry 1.3 │
  │                    │ output value 1b    ║                  │                  │                  │
  ├────────────────────┼────────────────────╫──────────────────┼──────────────────┼──────────────────┤
  │         U          │                    ║         1        │        2         │         3        │
  └────────────────────┴────────────────────╨──────────────────┴──────────────────┴──────────────────┘
"#;

/// General vertical decision table with multiple outputs.
pub const EX_0104: &str = r#"
  ┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
  │ information item name                                                                            │
  ├──────────────────────┬──────────────────╥─────────────────────────────────────┬──────────────────┤
  │ input expression 1   │ input value 1a,  ║            input entry 1.1          │ input entry 1.2  │
  │                      │  input value 1b  ║                                     │                  │
  ├──────────────────────┼──────────────────╫──────────────────┬──────────────────┼──────────────────┤
  │ input expression 2   │ input value 2a,  ║ input entry 2.1  │ input entry 2.2  │         -        │
  │                      │  input value 2b  ║                  │                  │                  │
  ╞════════╤═════════════╪══════════════════╬══════════════════╪══════════════════╪══════════════════╡
  │        │   output    │ output value 1a, ║ output entry 1.1 │ output entry 1.2 │ output entry 1.3 │
  │ output │ component 1 │  output value 1b ║                  │                  │                  │
  │ label  ├─────────────┼──────────────────╫──────────────────┼──────────────────┼──────────────────┤
  │        │   output    │ output value 2a, ║ output entry 2.1 │ output entry 2.2 │ output entry 2.3 │
  │        │ component 2 │  output value 2b ║                  │                  │                  │
  ├────────┴─────────────┼──────────────────╫──────────────────┼──────────────────┼──────────────────┤
  │          U           │                  ║         1        │        2         │         3        │
  └──────────────────────┴──────────────────╨──────────────────┴──────────────────┴──────────────────┘
"#;

/// General vertical decision table with annotations.
pub const EX_0105: &str = r#"
  ┌───────────────────────────┐
  │   information item name   │
  ├────────────────────┬──────┴─────────────╥─────────────────────────────────────────────┬──────────────────────┐
  │ input expression 1 │ input value 1a,    ║               input entry 1.1               │    input entry 1.1   │
  │                    │  input value 1b    ║                                             │                      │
  ├────────────────────┼────────────────────╫──────────────────────┬──────────────────────┼──────────────────────┤
  │ input expression 2 │ input value 2a,    ║   input entry 2.1    │   input entry 2.2    │           -          │
  │                    │  input value 2b    ║                      │                      │                      │
  ╞════════════════════╪════════════════════╬══════════════════════╪══════════════════════╪══════════════════════╡
  │    output label    │ output value 1a,   ║   output entry 1.1   │   output entry 1.2   │   output entry 1.3   │
  │                    │  output value 1b   ║                      │                      │                      │
  ╞════════════════════╪════════════════════╬══════════════════════╪══════════════════════╪══════════════════════╡
  │    annotation 1    │                    ║ annotation entry 1.1 │ annotation entry 1.2 │ annotation entry 1.3 │
  ├────────────────────┼────────────────────╫──────────────────────┼──────────────────────┼──────────────────────┤
  │    annotation 2    │                    ║ annotation entry 2.1 │ annotation entry 2.2 │ annotation entry 2.3 │
  ├────────────────────┼────────────────────╫──────────────────────┼──────────────────────┼──────────────────────┤
  │         U          │                    ║           1          │           2          │           3          │
  └────────────────────┴────────────────────╨──────────────────────┴──────────────────────┴──────────────────────┘
"#;

/// General crosstab decision table.
pub const EX_0106: &str = r#"
  ┌────────────────────────────────────────────────────────────────┐
  │ information item name                                          │
  ├──────────────────────────────────╥─────────────────────────────┤
  │                                  ║      input expression 1     │
  │           output label           ╟──────────────┬──────────────┤
  │                                  ║ input entry  │ input entry  │
  │                                  ║      1.1     │      1.2     │
  ╞════════════════════╤═════════════╬══════════════╪══════════════╡
  │                    │ input entry ║ output entry │ output entry │
  │                    │     2.1     ║      1.1     │      1.3     │
  │ input expression 2 ├─────────────╫──────────────┼──────────────┤
  │                    │ input entry ║ output entry │ output entry │
  │                    │     2.2     ║      1.2     │      1.4     │
  └────────────────────┴─────────────╨──────────────┴──────────────┘
"#;

/// General crosstab decision table with multiple outputs.
pub const EX_0107: &str = r#"
  ┌───────────────────────────────────────────────────────────────┐
  │ information item name                                         │
  ├───────────────────────╥───────────────────────────────────────┤
  │     output label      ║          input expression 1           │
  ├───────────────────────╫───────────────────┬───────────────────┤
  │  output component 1,  ║  input entry 1a   │  input entry 1b   │
  │  output component 2   ║                   │                   │
  ╞════════════╤══════════╬═══════════════════╪═══════════════════╡
  │            │  input   ║ output entry 1.1, │ output entry 1.3, │
  │   input    │ entry 2a ║ output entry 2.1  │ output entry 2.3  │
  │ expression ├──────────╫───────────────────┼───────────────────┤
  │     2      │  input   ║ output entry 1.2, │ output entry 1.4, │
  │            │ entry 2b ║ output entry 2.2  │ output entry 2.4  │
  └────────────┴──────────╨───────────────────┴───────────────────┘
"#;
