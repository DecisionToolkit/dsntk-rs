use super::*;

from_examples!(DMN_3_0074);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "context_001", &ctx, r#""foo""#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_001", &ctx, r#"2018"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_002", &ctx, r#"12"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_003", &ctx, r#"10"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "date_004", &ctx, r#"1"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_001", &ctx, r#"2018"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_002", &ctx, r#"12"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_003", &ctx, r#"10"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_004", &ctx, r#"1"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_005", &ctx, r#"10"#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_005_a", &ctx, r#"0"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_006", &ctx, r#"30"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_006_a", &ctx, r#"0"#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_007", &ctx, r#"1"#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_007_a", &ctx, r#"0"#);
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_008", &ctx, r#"PT5H"#);
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "dateTime_008_a",
    &ctx,
    r#"null(could not retrieve time offset for date and time)"#,
  );
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dateTime_009", &ctx, r#""Etc/UTC""#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "dateTime_009_a",
    &ctx,
    r#"null(could not retrieve timezone for date and time)"#,
  );
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_001", &ctx, r#"10"#);
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_002", &ctx, r#"30"#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_003", &ctx, r#"1"#);
}

#[test]
fn _0023() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "time_004", &ctx, r#"PT5H"#);
}

#[test]
fn _0024() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "time_004_a",
    &ctx,
    r#"null(could not retrieve time offset for time)"#,
  );
}

#[test]
fn _0025() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "time_005_a",
    &ctx,
    r#"null(could not retrieve timezone for time)"#,
  );
}

#[test]
fn _0026() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_001", &ctx, r#"1"#);
}

#[test]
fn _0027() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_001_a", &ctx, r#"0"#);
}

#[test]
fn _0028() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_002", &ctx, r#"2"#);
}

#[test]
fn _0029() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "ym_duration_002_a", &ctx, r#"0"#);
}

#[test]
fn _0030() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "ym_duration_003",
    &ctx,
    r#"null(no such property in years and months duration: days)"#,
  );
}

#[test]
fn _0031() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "ym_duration_004",
    &ctx,
    r#"null(no such property in years and months duration: hours)"#,
  );
}

#[test]
fn _0032() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "ym_duration_005",
    &ctx,
    r#"null(no such property in years and months duration: minutes)"#,
  );
}

#[test]
fn _0033() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "ym_duration_006",
    &ctx,
    r#"null(no such property in years and months duration: seconds)"#,
  );
}

#[test]
fn _0034() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "dt_duration_001",
    &ctx,
    r#"null(no such property in days and time duration: years)"#,
  );
}

#[test]
fn _0035() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "dt_duration_002",
    &ctx,
    r#"null(no such property in days and time duration: months)"#,
  );
}

#[test]
fn _0036() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_003", &ctx, r#"1"#);
}

#[test]
fn _0037() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_003_a", &ctx, r#"0"#);
}

#[test]
fn _0038() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_004", &ctx, r#"2"#);
}

#[test]
fn _0039() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_004_a", &ctx, r#"0"#);
}

#[test]
fn _0040() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_005", &ctx, r#"2"#);
}

#[test]
fn _0041() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_005_a", &ctx, r#"0"#);
}

#[test]
fn _0042() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_006", &ctx, r#"2"#);
}

#[test]
fn _0043() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "dt_duration_006_a", &ctx, r#"0"#);
}

#[test]
fn _0044() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_001", &ctx, r#"1"#);
}

#[test]
fn _0045() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_002", &ctx, r#"10"#);
}

#[test]
fn _0046() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_003", &ctx, r#"true"#);
}

#[test]
fn _0047() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_004", &ctx, r#"false"#);
}

#[test]
fn _0048() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_005", &ctx, r#"false"#);
}

#[test]
fn _0049() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_006", &ctx, r#"true"#);
}

#[test]
fn _0050() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_007", &ctx, r#"false"#);
}

#[test]
fn _0051() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_008", &ctx, r#"false"#);
}

#[test]
fn _0052() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "range_009",
    &ctx,
    r#"null(no such property in unary less: start)"#,
  );
}

#[test]
fn _0053() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "range_010",
    &ctx,
    r#"null(no such property in unary less or equal: start)"#,
  );
}

#[test]
fn _0054() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_011", &ctx, r#"10"#);
}

#[test]
fn _0055() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_012", &ctx, r#"10"#);
}

#[test]
fn _0056() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_013", &ctx, r#"10"#);
}

#[test]
fn _0057() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_014", &ctx, r#"10"#);
}

#[test]
fn _0058() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "range_015",
    &ctx,
    r#"null(no such property in unary greater: end)"#,
  );
}

#[test]
fn _0059() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    &MODEL_NAME,
    "range_016",
    &ctx,
    r#"null(no such property in unary greater or equal: end)"#,
  );
}

#[test]
fn _0060() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_017", &ctx, r#"false"#);
}

#[test]
fn _0061() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_018", &ctx, r#"false"#);
}

#[test]
fn _0062() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_019", &ctx, r#"false"#);
}

#[test]
fn _0063() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_020", &ctx, r#"true"#);
}

#[test]
fn _0064() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_021", &ctx, r#"false"#);
}

#[test]
fn _0065() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_022", &ctx, r#"true"#);
}

#[test]
fn _0066() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_023", &ctx, r#"false"#);
}

#[test]
fn _0067() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, "range_024", &ctx, r#"false"#);
}
