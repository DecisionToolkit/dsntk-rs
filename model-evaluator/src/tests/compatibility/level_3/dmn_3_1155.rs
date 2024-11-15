use super::*;

from_examples!(DMN_3_1155);
static_context!(CTX, "{}");

fn check(invocable_name: &str, expected: &str) {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0001() {
  check("decision001", "[1, 4, 3]");
}

#[test]
fn _0002() {
  check("decision002", "[1, 2, 4]");
}

#[test]
fn _0003() {
  check("decision003", "null");
}

#[test]
fn _0004() {
  check("decision004", "null");
}

#[test]
fn _0005() {
  check("decision005", "null");
}

#[test]
fn _0006() {
  check("decision006", "null([core::list replace] invalid argument type, expected list, actual type is Null)");
}

#[test]
fn _0007() {
  check("decision007", "null");
}

#[test]
fn _0008() {
  check("decision008", "[1, 2, null]");
}

#[test]
fn _0009() {
  check("decision009", "[5, 5, 7, 8]");
}

#[test]
fn _0010() {
  check("decision010", "null");
}

#[test]
fn _0011() {
  check("decision011", "[1, 4, 3]");
}

#[test]
fn _0012() {
  check("decision011_a", "[1, 2, 4]");
}

#[test]
fn _0013() {
  check("decision012", "[1, 4, 3]");
}

#[test]
fn _0014() {
  check("decision013", "[1, 4, 3]");
}

#[test]
fn _0015() {
  check("decision014", "null(expected 3 parameters, actual number of parameters is 4)");
}

#[test]
fn _0016() {
  check("decision015", "null(expected 3 parameters, actual number of parameters is 2)");
}

#[test]
fn _0017() {
  check("decision016", "null(expected 3 parameters, actual number of parameters is 4)");
}

#[test]
fn _0018() {
  check("decision017", "null(list replace: matching function must accept exactly two arguments)");
}

#[test]
fn _0019() {
  check("decision018", "null(list replace: matching function must accept exactly two arguments)");
}

#[test]
fn _0020() {
  check("decision019", "null(list replace: matching function must return boolean value)");
}

#[test]
fn _0021() {
  check("decision020", "[5, 5, 5, 5]");
}

#[test]
fn _0022() {
  check("decision021", "[5]");
}
