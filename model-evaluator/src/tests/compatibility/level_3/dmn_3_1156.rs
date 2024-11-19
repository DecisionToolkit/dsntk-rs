use super::*;

from_examples!(DMN_3_1156);
static_context!(CTX, "{}");

fn assert(invocable_name: &str, expected: &str) {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0001() {
  assert("decision001", "true");
}

#[test]
fn _0002() {
  assert("decision001_a", "true");
}

#[test]
fn _0003() {
  assert("decision001_b", "true");
}

#[test]
fn _0004() {
  assert("decision001_c", "true");
}

#[test]
fn _0005() {
  assert("decision001_d", "true");
}

#[test]
fn _0006() {
  assert("decision001_e", "true");
}

#[test]
fn _0007() {
  assert("decision001_g", "true");
}

#[test]
fn _0008() {
  assert("decision001_h", "true");
}

#[test]
fn _0009() {
  assert("decision001_i", "true");
}

#[test]
fn _0010() {
  assert("decision002", "true");
}

#[test]
fn _0011() {
  assert("decision003_a", "true");
}

#[test]
fn _0012() {
  assert("decision003_b", "true");
}

#[test]
fn _0013() {
  assert("decision003_c", "true");
}

#[test]
fn _0014() {
  assert("decision003_d", "true");
}

#[test]
fn _0015() {
  assert("decision003_e", "true");
}

#[test]
fn _0016() {
  assert("decision004_e", "null(invalid range literal)");
}

#[test]
fn _0017() {
  assert("decision004_f", "null(invalid range literal)");
}

#[test]
fn _0018() {
  assert("decision005_a", "true");
}

#[test]
fn _0019() {
  assert("decision005_b", "true");
}

#[test]
fn _0020() {
  assert("decision005_c", "true");
}

#[test]
fn _0021() {
  assert("decision005_d", "true");
}

#[test]
fn _0022() {
  assert("decision005_e", "true");
}

#[test]
fn _0023() {
  assert("decision005_f", "true");
}

#[test]
fn _0024() {
  assert("decision006", "true");
}

#[test]
fn _0025() {
  assert("decision007_a", "true");
}

#[test]
fn _0026() {
  assert("decision007_b", "null(invalid range literal)");
}

#[test]
fn _0027() {
  assert("decision007_c", "null(invalid range literal)");
}

#[test]
fn _0028() {
  assert("decision008_a", "true");
}

#[test]
fn _0029() {
  assert("decision008_b", "null(invalid range literal)");
}

#[test]
fn _0030() {
  assert("decision008_c", "null(invalid range literal)");
}

#[test]
fn _0031() {
  assert("decision009_a", "true");
}

#[test]
fn _0032() {
  assert("decision009_b", "null(invalid range literal)");
}

#[test]
fn _0033() {
  assert("decision009_c", "null(invalid range literal)");
}

#[test]
fn _0034() {
  assert("decision010_a", "true");
}

#[test]
fn _0035() {
  assert("decision010_b", "null(invalid range literal)");
}

#[test]
fn _0036() {
  assert("decision010_c", "null(invalid range literal)");
}

#[test]
fn _0037() {
  assert("decision011", "true");
}

#[test]
fn _0038() {
  assert("decision012", "null(parameter 'from' not found)");
}

#[test]
fn _0039() {
  assert("decision013_a", "null(expected 1 parameters, actual number of parameters is 2)");
}

#[test]
fn _0040() {
  assert("decision013_b", "null(expected 1 parameters, actual number of parameters is 0)");
}

#[test]
fn _0041() {
  assert("decision014", "null([core::range] invalid argument type, expected string, actual type is range<number>)");
}

#[test]
fn _0042() {
  assert("decision015_a", "null(invalid range literal)");
}

#[test]
fn _0043() {
  assert("decision015_b", "null(invalid range literal)");
}

#[test]
fn _0044() {
  assert("decision016", "null(invalid range literal)");
}

#[test]
fn _0045() {
  assert("decision017", "null(invalid range literal)");
}

#[test]
fn _0046() {
  assert("decision018", "null(invalid range)");
}

#[test]
fn _0047() {
  assert("decision027", "null(invalid range literal)");
}
