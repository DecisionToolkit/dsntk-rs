use super::*;

from_examples!(DMN_3_1155);
static_context!(CTX, "{}");

fn assert(invocable_name: &str, expected: &str) {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX, expected);
}

#[test]
fn _0001() {
  assert("decision001", "[1, 4, 3]");
}

#[test]
fn _0002() {
  assert("decision002", "[1, 2, 4]");
}

#[test]
fn _0003() {
  assert("decision003", "null");
}

#[test]
fn _0004() {
  assert("decision004", "null");
}

#[test]
fn _0005() {
  assert("decision005", "null");
}

#[test]
fn _0006() {
  assert("decision006", "null([core::list replace] invalid argument type, expected list, actual type is Null)");
}

#[test]
fn _0007() {
  assert("decision007", "null");
}

#[test]
fn _0008() {
  assert("decision008", "[1, 2, null]");
}

#[test]
fn _0009() {
  assert("decision009", "[5, 5, 7, 8]");
}

#[test]
fn _0010() {
  assert("decision010", "null");
}

#[test]
fn _0011() {
  assert("decision011", "[1, 4, 3]");
}

#[test]
fn _0012() {
  assert("decision011_a", "[1, 2, 4]");
}

#[test]
fn _0013() {
  assert("decision012", "[1, 4, 3]");
}

#[test]
fn _0014() {
  assert("decision013", "[1, 4, 3]");
}

#[test]
fn _0015() {
  assert("decision014", "null(expected 3 parameters, actual number of parameters is 4)");
}

#[test]
fn _0016() {
  assert("decision015", "null(expected 3 parameters, actual number of parameters is 2)");
}

#[test]
fn _0017() {
  assert("decision016", "null(expected 3 parameters, actual number of parameters is 4)");
}

#[test]
fn _0018() {
  assert("decision017", "null(list replace: matching function must accept exactly two arguments)");
}

#[test]
fn _0019() {
  assert("decision018", "null(list replace: matching function must accept exactly two arguments)");
}

#[test]
fn _0020() {
  assert("decision019", "null(list replace: matching function must return boolean value)");
}

#[test]
fn _0021() {
  assert("decision020", "[5, 5, 5, 5]");
}

#[test]
fn _0022() {
  assert("decision021", "[5]");
}
