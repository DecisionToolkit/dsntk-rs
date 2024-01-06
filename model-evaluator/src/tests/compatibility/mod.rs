//! # Compatibility tests
//!
//! ```text
//! ┌────────────────────────────────────────────────────────────────────────┬────────┬───────┬─────────┐
//! │ TCK test directory                                                     │  XML   │ Rust  │ Rust    │
//! │                                                                        │ tests  │ tests │ benches │
//! ├────────────────────────────────────────────────────────────────────────┼────────┼───────┼─────────┤
//! │ compliance-level-2/0001-input-data-string                              │      1 │     1 │       1 │
//! │ compliance-level-2/0002-input-data-number                              │      1 │     1 │       1 │
//! │ compliance-level-2/0003-input-data-string-allowed-values               │      1 │ level_2/dmn_2_0003.rs                     2            2     0
//! │ compliance-level-2/0004-simpletable-U                                  │      3 │ level_2/dmn_2_0004.rs                     3            3     0
//! │ compliance-level-2/0005-simpletable-A                                  │      3 │ level_2/dmn_2_0005.rs                     3            3     0
//! │ compliance-level-2/0006-simpletable-P1                                 │      3 │ level_2/dmn_2_0006.rs                     3            3     0
//! │ compliance-level-2/0007-simpletable-P2                                 │      3 │ level_2/dmn_2_0007.rs                     3            3     0
//! │ compliance-level-2/0008-LX-arithmetic                                  │      3 │ level_2/dmn_2_0008.rs                     3            3     0
//! │ compliance-level-2/0009-invocation-arithmetic                          │      3 │ level_2/dmn_2_0009.rs                     3            3     0
//! │ compliance-level-2/0010-multi-output-U                                 │      3 │ level_2/dmn_2_0010.rs                     3            3     0
//! │ compliance-level-2/0100-feel-constants                                 │      2 │ level_2/dmn_2_0100.rs                     2            2     0
//! │ compliance-level-2/0101-feel-constants                                 │      6 │ level_2/dmn_2_0101.rs                     6            6     0
//! │ compliance-level-2/0102-feel-constants                                 │      4 │ level_2/dmn_2_0102.rs                     4            4     0
//! │ compliance-level-2/0105-feel-math                                      │     33 │ level_2/dmn_2_0105.rs                    33           33     0
//! │ compliance-level-2/0106-feel-ternary-logic                             │     18 │ level_2/dmn_2_0106.rs                    18           18     0
//! │ compliance-level-2/0107-feel-ternary-logic-not                         │      3 │ level_2/dmn_2_0107.rs                     3            3     0
//! │ compliance-level-2/0108-first-hitpolicy                                │      3 │ level_2/dmn_2_0108.rs                     3            3     0
//! │ compliance-level-2/0109-ruleOrder-hitpolicy                            │      3 │ level_2/dmn_2_0109.rs                     3            3     0
//! │ compliance-level-2/0110-outputOrder-hitpolicy                          │      3 │ level_2/dmn_2_0110.rs                     3            3     0
//! │ compliance-level-2/0111-first-hitpolicy-singleoutputcol                │      3 │ level_2/dmn_2_0111.rs                     3            3     0
//! │ compliance-level-2/0112-ruleOrder-hitpolicy-singleinoutcol             │      3 │ level_2/dmn_2_0112.rs                     3            3     0
//! │ compliance-level-2/0113-outputOrder-hitpolicy-singleinoutcol           │      3 │ level_2/dmn_2_0113.rs                     3            3     0
//! │ compliance-level-2/0114-min-collect-hitpolicy                          │      3 │ level_2/dmn_2_0114.rs                     3            3     0
//! │ compliance-level-2/0115-sum-collect-hitpolicy                          │      3 │ level_2/dmn_2_0115.rs                     3            3     0
//! │ compliance-level-2/0116-count-collect-hitpolicy                        │      3 │ level_2/dmn_2_0116.rs                     3            3     0
//! │ compliance-level-2/0117-multi-any-hitpolicy                            │      3 │ level_2/dmn_2_0117.rs                     3            3     0
//! │ compliance-level-2/0118-multi-priority-hitpolicy                       │      3 │ level_2/dmn_2_0118.rs                     3            3     0
//! │ compliance-level-2/0119-multi-collect-hitpolicy                        │      3 │ level_2/dmn_2_0119.rs                     3            3     0
//! │ compliance-level-3/0001-filter                                         │      1 │ level_3/dmn_3_0001.rs                     1            1     0
//! │ compliance-level-3/0002-string-functions                               │      4 │ level_3/dmn_3_0002.rs                     4            4     0
//! │ compliance-level-3/0003-iteration                                      │      1 │ level_3/dmn_3_0003.rs                     1            1     0
//! │ compliance-level-3/0004-lending                                        │     11 │ level_3/dmn_3_0004.rs                    11           11     0
//! │ compliance-level-3/0005-literal-invocation                             │      3 │ level_3/dmn_3_0005.rs                     3            3     0
//! │ compliance-level-3/0006-join                                           │      1 │ level_3/dmn_3_0006.rs                     1            1     0
//! │ compliance-level-3/0007-date-time                                      │     19 │ level_3/dmn_3_0007.rs                    19           19     0
//! │ compliance-level-3/0008-listGen                                        │     10 │ level_3/dmn_3_0008.rs                    10           10     0
//! │ compliance-level-3/0009-append-flatten                                 │     10 │ level_3/dmn_3_0009.rs                    10           10     0
//! │ compliance-level-3/0010-concatenate                                    │      6 │ level_3/dmn_3_0010.rs                     6            6     0
//! │ compliance-level-3/0011-insert-remove                                  │      6 │ level_3/dmn_3_0011.rs                     6            6     0
//! │ compliance-level-3/0012-list-functions                                 │     19 │ level_3/dmn_3_0012.rs                    19           19     0
//! │ compliance-level-3/0013-sort                                           │      3 │ level_3/dmn_3_0013.rs                     3            3     0
//! │ compliance-level-3/0014-loan-comparison                                │      2 │ level_3/dmn_3_0014.rs                     7            7     0
//! │ compliance-level-3/0016-some-every                                     │      8 │ level_3/dmn_3_0016.rs                     8            8     0
//! │ compliance-level-3/0017-tableTests                                     │      4 │ level_3/dmn_3_0017.rs                     4            4     0
//! │ compliance-level-3/0020-vacation-days                                  │      7 │ level_3/dmn_3_0020.rs                     7            7     0
//! │ compliance-level-3/0021-singleton-list                                 │      5 │ level_3/dmn_3_0021.rs                     6            6     0
//! │ compliance-level-3/0030-user-defined-functions                         │      2 │ level_3/dmn_3_0030.rs                     2            2     0
//! │ compliance-level-3/0031-user-defined-functions                         │      3 │ level_3/dmn_3_0031.rs                     3            3     0
//! │ compliance-level-3/0032-conditionals                                   │      6 │ level_3/dmn_3_0032.rs                     6            6     0
//! │ compliance-level-3/0033-for-loops                                      │      4 │ level_3/dmn_3_0033.rs                     4            4     0
//! │ compliance-level-3/0034-drg-scopes                                     │     12 │ level_3/dmn_3_0034.rs                    12           12     0
//! │ compliance-level-3/0035-test-structure-output                          │      3 │ level_3/dmn_3_0035.rs                     3            3     0
//! │ compliance-level-3/0036-dt-variable-input                              │     24 │ level_3/dmn_3_0036.rs                    24           24     0
//! │ compliance-level-3/0037-dt-on-bkm-implicit-params                      │      2 │ level_3/dmn_3_0037.rs                     2            2     0
//! │ compliance-level-3/0038-dt-on-bkm-explicit-params                      │      2 │ level_3/dmn_3_0038.rs                     2            2     0
//! │ compliance-level-3/0039-dt-list-semantics                              │      2 │ level_3/dmn_3_0039.rs                     2            2     0
//! │ compliance-level-3/0040-singlenestedcontext                            │      3 │ level_3/dmn_3_0040.rs                     3            3     0
//! │ compliance-level-3/0041-multiple-nestedcontext                         │      3 │ level_3/dmn_3_0041.rs                     3            3     0
//! │ compliance-level-3/0050-feel-abs-function                              │     17 │ level_3/dmn_3_0050.rs                    17           17     0
//! │ compliance-level-3/0051-feel-sqrt-function                             │     15 │ level_3/dmn_3_0051.rs                    15           15     0
//! │ compliance-level-3/0052-feel-exp-function                              │     15 │ level_3/dmn_3_0052.rs                    15           15     0
//! │ compliance-level-3/0053-feel-log-function                              │     15 │ level_3/dmn_3_0053.rs                    15           15     0
//! │ compliance-level-3/0054-feel-even-function                             │     17 │ level_3/dmn_3_0054.rs                    17           17     0
//! │ compliance-level-3/0055-feel-odd-function                              │     17 │ level_3/dmn_3_0055.rs                    17           17     0
//! │ compliance-level-3/0056-feel-modulo-function                           │     28 │ level_3/dmn_3_0056.rs                    28           28     0
//! │ compliance-level-3/0057-feel-context                                   │     11 │ level_3/dmn_3_0057.rs                    11           11     0
//! │ compliance-level-3/0058-feel-number-function                           │     21 │ level_3/dmn_3_0058.rs                    21           21     0
//! │ compliance-level-3/0059-feel-all-function                              │     19 │ level_3/dmn_3_0059.rs                    19           19     0
//! │ compliance-level-3/0060-feel-any-function                              │     17 │ level_3/dmn_3_0060.rs                    17           17     0
//! │ compliance-level-3/0061-feel-median-function                           │     14 │ level_3/dmn_3_0061.rs                    14           14     0
//! │ compliance-level-3/0062-feel-mode-function                             │     13 │ level_3/dmn_3_0062.rs                    13           13     0
//! │ compliance-level-3/0063-feel-stddev-function                           │     11 │ level_3/dmn_3_0063.rs                    11           11     0
//! │ compliance-level-3/0064-feel-conjunction                               │     19 │ level_3/dmn_3_0064.rs                    19           19     0
//! │ compliance-level-3/0065-feel-disjunction                               │     19 │ level_3/dmn_3_0065.rs                    19           19     0
//! │ compliance-level-3/0066-feel-negation                                  │      6 │ level_3/dmn_3_0066.rs                     6            6     0
//! │ compliance-level-3/0067-feel-split-function                            │      9 │ level_3/dmn_3_0067.rs                     9            9     0
//! │ compliance-level-3/0068-feel-equality                                  │    108 │ level_3/dmn_3_0068.rs                   109           72 *   37
//! │ compliance-level-3/0069-feel-list                                      │     35 │ level_3/dmn_3_0069.rs                    35           27 *   8
//! │ compliance-level-3/0070-feel-instance-of                               │    142 │ level_3/dmn_3_0070.rs                   154          154     0
//! │ compliance-level-3/0071-feel-between                                   │     38 │ level_3/dmn_3_0071.rs                    38           35 *   3
//! │ compliance-level-3/0072-feel-in                                        │    274 │ level_3/dmn_3_0072.rs                   274          274     0
//! │ compliance-level-3/0073-feel-comments                                  │      3 │ level_3/dmn_3_0073.rs                     3            3     0
//! │ compliance-level-3/0074-feel-properties                                │     67 │ level_3/dmn_3_0074.rs                    67           43 *   24
//! │ compliance-level-3/0075-feel-exponent                                  │     12 │ level_3/dmn_3_0075.rs                    12           12     0
//! │ compliance-level-3/0076-feel-external-java                             │     18 │ level_3/dmn_3_0076.rs                    18           18     0
//! │ compliance-level-3/0077-feel-nan                                       │      1 │ level_3/dmn_3_0077.rs                     1            1     0
//! │ compliance-level-3/0078-feel-infinity                                  │      2 │ level_3/dmn_3_0078.rs                     2            2     0
//! │ compliance-level-3/0080-feel-getvalue-function                         │     14 │ level_3/dmn_3_0080.rs                    14           12 *   2
//! │ compliance-level-3/0081-feel-getentries-function                       │      9 │ level_3/dmn_3_0081.rs                     9            9     0
//! │ compliance-level-3/0082-feel-coercion                                  │     36 │ level_3/dmn_3_0082.rs                    34           33 *   1
//! │ compliance-level-3/0083-feel-unicode                                   │     14 │ level_3/dmn_3_0083.rs                    18           11 *   7
//! │ compliance-level-3/0084-feel-for-loops                                 │     13 │ level_3/dmn_3_0084.rs                    13           10 *   3
//! │ compliance-level-3/0085-decision-services                              │     17 │ level_3/dmn_3_0085.rs                    26           10 *   16
//! │ compliance-level-3/0086-import                                         │      2 │ level_3/dmn_3_0086.rs                     2            2     0
//! │ compliance-level-3/0087-chapter-11-example                             │      9 │ level_3/dmn_3_0087.rs                     9            9     0
//! │ compliance-level-3/0088-no-decision-logic                              │      1 │ level_3/dmn_3_0088.rs                     1            1     0
//! │ compliance-level-3/0089-nested-inputdata-imports                       │      1 │ level_3/dmn_3_0089.rs                     9            4 *   5
//! │ compliance-level-3/0090-feel-paths                                     │      4 │ level_3/dmn_3_0090.rs                     4            4     0
//! │ compliance-level-3/0091-local-hrefs                                    │      1 │ level_3/dmn_3_0091.rs                     1            0 *   1
//! │ compliance-level-3/0092-feel-lambda                                    │     18 │ level_3/dmn_3_0092.rs                    19            0 *   19
//! │ compliance-level-3/0093-feel-at-literals                               │     19 │ level_3/dmn_3_0093.rs                    19            0 *   19
//! │ compliance-level-3/0094-feel-product-function                          │     13 │ level_3/dmn_3_0094.rs                    13            0 *   13
//! │ compliance-level-3/0095-feel-day-of-year-function                      │     19 │ level_3/dmn_3_0095.rs                    19            0 *   19
//! │ compliance-level-3/0096-feel-day-of-week-function                      │     12 │ level_3/dmn_3_0096.rs                    12            0 *   12
//! │ compliance-level-3/0097-feel-month-of-year-function                    │     12 │ level_3/dmn_3_0097.rs                    12            0 *   12
//! │ compliance-level-3/0098-feel-week-of-year-function                     │     19 │ level_3/dmn_3_0098.rs                    19            0 *   19
//! │ compliance-level-3/0099-arithmetic-negation                            │     10 │ level_3/dmn_3_0099.rs                    14            0 *   14
//! │ compliance-level-3/0100-arithmetic                                     │   1087 │ level_3/dmn_3_0100.rs                  1087         1087     0
//! │ compliance-level-3/0103-feel-is-function                               │     50 │ level_3/dmn_3_0103.rs                    50           50     0
//! │ compliance-level-3/1100-feel-decimal-function                          │     10 │ level_3/dmn_3_1100.rs                    10           10     0
//! │ compliance-level-3/1101-feel-floor-function                            │     17 │ level_3/dmn_3_1101.rs                    17            6 *   11
//! │ compliance-level-3/1102-feel-ceiling-function                          │     17 │ level_3/dmn_3_1102.rs                     6            6     0
//! │ compliance-level-3/1103-feel-substring-function                        │     11 │ level_3/dmn_3_1103.rs                    11           11     0
//! │ compliance-level-3/1104-feel-string-length-function                    │      6 │ level_3/dmn_3_1104.rs                     6            6     0
//! │ compliance-level-3/1105-feel-upper-case-function                       │      8 │ level_3/dmn_3_1105.rs                     8            8     0
//! │ compliance-level-3/1106-feel-lower-case-function                       │      9 │ level_3/dmn_3_1106.rs                     9            9     0
//! │ compliance-level-3/1107-feel-substring-before-function                 │      9 │ level_3/dmn_3_1107.rs                     9            9     0
//! │ compliance-level-3/1108-feel-substring-after-function                  │     10 │ level_3/dmn_3_1108.rs                    10           10     0
//! │ compliance-level-3/1109-feel-replace-function                          │     28 │ level_3/dmn_3_1109.rs                    28           28     0
//! │ compliance-level-3/1110-feel-contains-function                         │     10 │ level_3/dmn_3_1110.rs                    10           10     0
//! │ compliance-level-3/1111-feel-matches-function                          │     40 │ level_3/dmn_3_1111.rs                    44            0 *   44
//! │ compliance-level-3/1115-feel-date-function                             │     52 │ level_3/dmn_3_1115.rs                    52           52     0
//! │ compliance-level-3/1116-feel-time-function                             │     83 │ level_3/dmn_3_1116.rs                    83           83     0
//! │ compliance-level-3/1117-feel-date-and-time-function                    │     88 │ level_3/dmn_3_1117.rs                    88           88     0
//! │ compliance-level-3/1120-feel-duration-function                         │     50 │ level_3/dmn_3_1120.rs                    50           50     0
//! │ compliance-level-3/1121-feel-years-and-months-duration-function        │     36 │ level_3/dmn_3_1121.rs                    36           36     0
//! │ compliance-level-3/1130-feel-interval                                  │     14 │ level_3/dmn_3_1130.rs                    14            0 *   14
//! │ compliance-level-3/1131-feel-function-invocation                       │      8 │ level_3/dmn_3_1140.rs                    22            0 *   22
//! │ compliance-level-3/1140-feel-string-join-function                      │     22 │ level_3/dmn_3_1141.rs                     1            0 *   1
//! │ compliance-level-3/1141-feel-round-up-function                         │     16 │ level_3/dmn_3_1142.rs                     1            0 *   1
//! │ compliance-level-3/1142-feel-round-down-function                       │     16 │ level_3/dmn_3_1143.rs                     1            0 *   1
//! │ compliance-level-3/1143-feel-round-half-up-function                    │     16 │ level_3/dmn_3_1144.rs                     1            0 *   1
//! │ compliance-level-3/1144-feel-round-half-down-function                  │     16 │ level_3/dmn_3_1145.rs                    18            0 *   18
//! │ compliance-level-3/1145-feel-context-function                          │     18 │ level_3/dmn_3_1146.rs                    30           30     0
//! │ compliance-level-3/1146-feel-context-put-function                      │     30 │ level_3/dmn_3_1147.rs                    14            0 *   14
//! │ compliance-level-3/1147-feel-context-merge-function                    │     14 │ level_3/dmn_3_1148.rs                     2            0 *   2
//! │ compliance-level-3/1148-feel-now-function                              │      2 │ level_3/dmn_3_1149.rs                     2            0 *   2
//! │ compliance-level-3/1149-feel-today-function                            │      2 │ level_3/dmn_3_1150.rs                     3            0 *   3
//! │ compliance-level-3/1150-boxed-conditional                              │      3 │ level_3/dmn_3_1151.rs                     6            0 *   6
//! │ compliance-level-3/1151-boxed-filter                                   │      4 │ level_3/dmn_3_1152.rs                     4            0 *   4
//! │ compliance-level-3/1152-boxed-for                                      │      2 │ level_3/dmn_3_1153.rs                     7            0 *   7
//! │ compliance-level-3/1153-boxed-some                                     │      5 │ level_3/dmn_3_1154.rs                     7            0 *   7
//! │ compliance-level-3/1154-boxed-every                                    │      5 │ non_compliant/dmn_n_0015.rs              10            0 *   10
//! │ non-compliant/0015-all-any                                             │     10 │ non_compliant/dmn_n_0019.rs               2            0 *   2
//! │ non-compliant/0019-flight-rebooking                                    │      1 │ non_compliant/dmn_n_0079.rs              37           37     0
//! │ non-compliant/0079-feel-string-function                                │     37 │ non_compliant/dmn_n_0088.rs               1            0 *   1
//! ├────────────────────────────────────────────────────────────────────────┼────────┤
//! │                                                                  Total │   3400 │
//! └────────────────────────────────────────────────────────────────────────┴────────┘
//! ```

mod level_2;
mod level_3;
mod non_compliant;

use super::*;
