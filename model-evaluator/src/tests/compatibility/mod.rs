//! # Compatibility tests
//!
//! ```text
//! ┌────────────────────────────────────────────────────────────────────────┬────────┬───────┬─────────┐
//! │ TCK directory                                                          │  XML   │ Rust  │  Rust   │
//! │                                                                        │ tests  │ tests │ benches │
//! ├────────────────────────────────────────────────────────────────────────┼────────┼───────┼─────────┤
//! │ compliance-level-2/0001-input-data-string                              │      1 │     1 │       1 │
//! │ compliance-level-2/0002-input-data-number                              │      1 │     1 │       1 │
//! │ compliance-level-2/0003-input-data-string-allowed-values               │      1 │     2 │       2 │
//! │ compliance-level-2/0004-simpletable-U                                  │      3 │     3 │       3 │
//! │ compliance-level-2/0005-simpletable-A                                  │      3 │     3 │       3 │
//! │ compliance-level-2/0006-simpletable-P1                                 │      3 │     3 │       3 │
//! │ compliance-level-2/0007-simpletable-P2                                 │      3 │     3 │       3 │
//! │ compliance-level-2/0008-LX-arithmetic                                  │      3 │     3 │       3 │
//! │ compliance-level-2/0009-invocation-arithmetic                          │      3 │     3 │       3 │
//! │ compliance-level-2/0010-multi-output-U                                 │      3 │     3 │       3 │
//! │ compliance-level-2/0100-feel-constants                                 │      2 │     2 │       2 │
//! │ compliance-level-2/0101-feel-constants                                 │      6 │     6 │       6 │
//! │ compliance-level-2/0102-feel-constants                                 │      4 │     4 │       4 │
//! │ compliance-level-2/0105-feel-math                                      │     33 │    33 │      33 │
//! │ compliance-level-2/0106-feel-ternary-logic                             │     18 │    18 │      18 │
//! │ compliance-level-2/0107-feel-ternary-logic-not                         │      3 │     3 │       3 │
//! │ compliance-level-2/0108-first-hitpolicy                                │      3 │     3 │       3 │
//! │ compliance-level-2/0109-ruleOrder-hitpolicy                            │      3 │     3 │       3 │
//! │ compliance-level-2/0110-outputOrder-hitpolicy                          │      3 │     3 │       3 │
//! │ compliance-level-2/0111-first-hitpolicy-singleoutputcol                │      3 │     3 │       3 │
//! │ compliance-level-2/0112-ruleOrder-hitpolicy-singleinoutcol             │      3 │     3 │       3 │
//! │ compliance-level-2/0113-outputOrder-hitpolicy-singleinoutcol           │      3 │     3 │       3 │
//! │ compliance-level-2/0114-min-collect-hitpolicy                          │      3 │     3 │       3 │
//! │ compliance-level-2/0115-sum-collect-hitpolicy                          │      3 │     3 │       3 │
//! │ compliance-level-2/0116-count-collect-hitpolicy                        │      3 │     3 │       3 │
//! │ compliance-level-2/0117-multi-any-hitpolicy                            │      3 │     3 │       3 │
//! │ compliance-level-2/0118-multi-priority-hitpolicy                       │      3 │     3 │       3 │
//! │ compliance-level-2/0119-multi-collect-hitpolicy                        │      3 │     3 │       3 │
//! │ compliance-level-3/0001-filter                                         │      1 │     1 │       1 │
//! │ compliance-level-3/0002-string-functions                               │      4 │     4 │       4 │
//! │ compliance-level-3/0003-iteration                                      │      1 │     1 │       1 │
//! │ compliance-level-3/0004-lending                                        │     11 │    11 │      11 │
//! │ compliance-level-3/0005-literal-invocation                             │      3 │     3 │       3 │
//! │ compliance-level-3/0006-join                                           │      1 │     1 │       1 │
//! │ compliance-level-3/0007-date-time                                      │     19 │    19 │      19 │
//! │ compliance-level-3/0008-listGen                                        │     10 │    10 │      10 │
//! │ compliance-level-3/0009-append-flatten                                 │     10 │    10 │      10 │
//! │ compliance-level-3/0010-concatenate                                    │      6 │     6 │       6 │
//! │ compliance-level-3/0011-insert-remove                                  │      6 │     6 │       6 │
//! │ compliance-level-3/0012-list-functions                                 │     19 │    19 │      19 │
//! │ compliance-level-3/0013-sort                                           │      3 │     3 │       3 │
//! │ compliance-level-3/0014-loan-comparison                                │      2 │     7 │       7 │
//! │ compliance-level-3/0016-some-every                                     │      8 │     8 │       8 │
//! │ compliance-level-3/0017-tableTests                                     │      4 │     4 │       4 │
//! │ compliance-level-3/0020-vacation-days                                  │      7 │     7 │       7 │
//! │ compliance-level-3/0021-singleton-list                                 │      5 │     6 │       6 │
//! │ compliance-level-3/0030-user-defined-functions                         │      2 │     2 │       2 │
//! │ compliance-level-3/0031-user-defined-functions                         │      3 │     3 │       3 │
//! │ compliance-level-3/0032-conditionals                                   │      6 │     6 │       6 │
//! │ compliance-level-3/0033-for-loops                                      │      4 │     4 │       4 │
//! │ compliance-level-3/0034-drg-scopes                                     │     12 │    12 │      12 │
//! │ compliance-level-3/0035-test-structure-output                          │      3 │     3 │       3 │
//! │ compliance-level-3/0036-dt-variable-input                              │     24 │    24 │      24 │
//! │ compliance-level-3/0037-dt-on-bkm-implicit-params                      │      2 │     2 │       2 │
//! │ compliance-level-3/0038-dt-on-bkm-explicit-params                      │      2 │     2 │       2 │
//! │ compliance-level-3/0039-dt-list-semantics                              │      2 │     2 │       2 │
//! │ compliance-level-3/0040-singlenestedcontext                            │      3 │     3 │       3 │
//! │ compliance-level-3/0041-multiple-nestedcontext                         │      3 │     3 │       3 │
//! │ compliance-level-3/0050-feel-abs-function                              │     17 │    17 │      17 │
//! │ compliance-level-3/0051-feel-sqrt-function                             │     15 │    15 │      15 │
//! │ compliance-level-3/0052-feel-exp-function                              │     15 │    15 │      15 │
//! │ compliance-level-3/0053-feel-log-function                              │     15 │    15 │      15 │
//! │ compliance-level-3/0054-feel-even-function                             │     17 │    17 │      17 │
//! │ compliance-level-3/0055-feel-odd-function                              │     17 │    17 │      17 │
//! │ compliance-level-3/0056-feel-modulo-function                           │     28 │    28 │      28 │
//! │ compliance-level-3/0057-feel-context                                   │     11 │    11 │      11 │
//! │ compliance-level-3/0058-feel-number-function                           │     21 │    21 │      21 │
//! │ compliance-level-3/0059-feel-all-function                              │     19 │    19 │      19 │
//! │ compliance-level-3/0060-feel-any-function                              │     17 │    17 │      17 │
//! │ compliance-level-3/0061-feel-median-function                           │     14 │    14 │      14 │
//! │ compliance-level-3/0062-feel-mode-function                             │     13 │    13 │      13 │
//! │ compliance-level-3/0063-feel-stddev-function                           │     11 │    11 │      11 │
//! │ compliance-level-3/0064-feel-conjunction                               │     19 │    19 │      19 │
//! │ compliance-level-3/0065-feel-disjunction                               │     19 │    19 │      19 │
//! │ compliance-level-3/0066-feel-negation                                  │      6 │     6 │       6 │
//! │ compliance-level-3/0067-feel-split-function                            │      9 │     9 │       9 │
//! │ compliance-level-3/0068-feel-equality                                  │    108 │   109 │     109 │
//! │ compliance-level-3/0069-feel-list                                      │     35 │    35 │      35 │
//! │ compliance-level-3/0070-feel-instance-of                               │    142 │   154 │     154 │
//! │ compliance-level-3/0071-feel-between                                   │     38 │    38 │      38 │
//! │ compliance-level-3/0072-feel-in                                        │    274 │   274 │     274 │
//! │ compliance-level-3/0073-feel-comments                                  │      3 │     3 │       3 │
//! │ compliance-level-3/0074-feel-properties                                │     67 │    67 │      67 │
//! │ compliance-level-3/0075-feel-exponent                                  │     12 │    12 │      12 │
//! │ compliance-level-3/0076-feel-external-java                             │     18 │    18 │      18 │
//! │ compliance-level-3/0077-feel-nan                                       │      1 │     1 │       1 │
//! │ compliance-level-3/0078-feel-infinity                                  │      2 │     2 │       2 │
//! │ compliance-level-3/0080-feel-getvalue-function                         │     14 │    14 │      14 │
//! │ compliance-level-3/0081-feel-getentries-function                       │      9 │     9 │       9 │
//! │ compliance-level-3/0082-feel-coercion                                  │     37 │    37 │      37 │
//! │ compliance-level-3/0083-feel-unicode                                   │     14 │    18 │      11 │      -7
//! │ compliance-level-3/0084-feel-for-loops                                 │     13 │    13 │      10 │      -3
//! │ compliance-level-3/0085-decision-services                              │     17 │    26 │      10 │     -16
//! │ compliance-level-3/0086-import                                         │      2 │     2 │       2 │
//! │ compliance-level-3/0087-chapter-11-example                             │      9 │     9 │       9 │
//! │ compliance-level-3/0088-no-decision-logic                              │      1 │     1 │       1 │
//! │ compliance-level-3/0089-nested-inputdata-imports                       │      1 │     9 │       4 │      -5
//! │ compliance-level-3/0090-feel-paths                                     │      4 │     4 │       4 │
//! │ compliance-level-3/0091-local-hrefs                                    │      1 │     1 │       0 │      -1
//! │ compliance-level-3/0092-feel-lambda                                    │     18 │    19 │       0 │     -19
//! │ compliance-level-3/0093-feel-at-literals                               │     19 │    19 │       0 │     -19
//! │ compliance-level-3/0094-feel-product-function                          │     13 │    13 │       0 │     -13
//! │ compliance-level-3/0095-feel-day-of-year-function                      │     19 │    19 │       0 │     -19
//! │ compliance-level-3/0096-feel-day-of-week-function                      │     12 │    12 │       0 │     -12
//! │ compliance-level-3/0097-feel-month-of-year-function                    │     12 │    12 │       0 │     -12
//! │ compliance-level-3/0098-feel-week-of-year-function                     │     19 │    19 │       0 │     -19
//! │ compliance-level-3/0099-arithmetic-negation                            │     10 │    14 │       0 │     -14
//! │ compliance-level-3/0100-arithmetic                                     │   1087 │  1087 │    1087 │
//! │ compliance-level-3/0103-feel-is-function                               │     50 │    50 │      50 │
//! │ compliance-level-3/1100-feel-decimal-function                          │     10 │    10 │      10 │
//! │ compliance-level-3/1101-feel-floor-function                            │     17 │    17 │       6 │     -11
//! │ compliance-level-3/1102-feel-ceiling-function                          │     17 │    17 │      17 │
//! │ compliance-level-3/1103-feel-substring-function                        │     11 │    11 │      11 │
//! │ compliance-level-3/1104-feel-string-length-function                    │      6 │     6 │       6 │
//! │ compliance-level-3/1105-feel-upper-case-function                       │      8 │     8 │       8 │
//! │ compliance-level-3/1106-feel-lower-case-function                       │      9 │     9 │       9 │
//! │ compliance-level-3/1107-feel-substring-before-function                 │      9 │     9 │       9 │
//! │ compliance-level-3/1108-feel-substring-after-function                  │     10 │    10 │      10 │
//! │ compliance-level-3/1109-feel-replace-function                          │     28 │    28 │      28 │
//! │ compliance-level-3/1110-feel-contains-function                         │     10 │    10 │      10 │
//! │ compliance-level-3/1111-feel-matches-function                          │     40 │    44 │       0 │     -44
//! │ compliance-level-3/1115-feel-date-function                             │     52 │    52 │      52 │
//! │ compliance-level-3/1116-feel-time-function                             │     83 │    83 │      83 │
//! │ compliance-level-3/1117-feel-date-and-time-function                    │     88 │    88 │      88 │
//! │ compliance-level-3/1120-feel-duration-function                         │     50 │    50 │      50 │
//! │ compliance-level-3/1121-feel-years-and-months-duration-function        │     36 │    36 │      36 │
//! │ compliance-level-3/1130-feel-interval                                  │     14 │    14 │       0 │     -14
//! │ compliance-level-3/1131-feel-function-invocation                       │      8 │     8 │       8 │
//! │ compliance-level-3/1140-feel-string-join-function                      │     22 │    22 │       0 │     -22
//! │ compliance-level-3/1141-feel-round-up-function                         │     16 │    16 │      16 │
//! │ compliance-level-3/1142-feel-round-down-function                       │     16 │    16 │      16 │
//! │ compliance-level-3/1143-feel-round-half-up-function                    │     16 │    16 │      16 │
//! │ compliance-level-3/1144-feel-round-half-down-function                  │     16 │    16 │      16 │
//! │ compliance-level-3/1145-feel-context-function                          │     18 │    18 │       0 │     -18
//! │ compliance-level-3/1146-feel-context-put-function                      │     30 │    30 │      30 │
//! │ compliance-level-3/1147-feel-context-merge-function                    │     14 │    14 │       0 │     -14
//! │ compliance-level-3/1148-feel-now-function                              │      2 │     2 │       0 │      -2
//! │ compliance-level-3/1149-feel-today-function                            │      2 │     2 │       0 │      -2
//! │ compliance-level-3/1150-boxed-conditional                              │      3 │     3 │       0 │      -3
//! │ compliance-level-3/1151-boxed-filter                                   │      4 │     6 │       0 │      -6
//! │ compliance-level-3/1152-boxed-for                                      │      2 │     4 │       0 │      -4
//! │ compliance-level-3/1153-boxed-some                                     │      5 │     7 │       0 │      -7
//! │ compliance-level-3/1154-boxed-every                                    │      5 │     7 │       0 │      -7
//! │ non-compliant/0015-all-any                                             │     10 │    10 │       0 │     -10
//! │ non-compliant/0019-flight-rebooking                                    │      1 │     2 │       0 │      -2
//! │ non-compliant/0079-feel-string-function                                │     37 │    37 │      37 │
//! │ non-compliant/0088-recursion                                           │      2 │     1 │       0 │      -2
//! ├────────────────────────────────────────────────────────────────────────┼────────┼───────┼─────────┤
//! │                                                                  Total │   3400 │       │         │
//! └────────────────────────────────────────────────────────────────────────┴────────┴───────┴─────────┘
//! ```

mod level_2;
mod level_3;
mod non_compliant;

use super::*;
