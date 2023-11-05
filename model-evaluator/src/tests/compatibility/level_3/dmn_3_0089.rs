use super::*;
use dsntk_examples::*;

static MODEL_EVALUATOR_A: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_A]));
const MODEL_NAMESPACE_A: &str = "http://www.trisotech.com/definitions/_ae5b3c17-1ac3-4e1d-b4f9-2cf861aec6d9";
const MODEL_NAME_A: &str = "Model A";

static MODEL_EVALUATOR_B1: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_B1, DMN_3_0089_MODEL_A]));
const MODEL_NAMESPACE_B1: &str = "http://www.trisotech.com/definitions/_2a1d771a-a899-4fef-abd6-fc894332337c";
const MODEL_NAME_B1: &str = "Model B";

static MODEL_EVALUATOR_B2: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_B2, DMN_3_0089_MODEL_A]));
const MODEL_NAMESPACE_B2: &str = "http://www.trisotech.com/definitions/_9d46ece4-a96c-4cb0-abc0-0ca121ac3768";
const MODEL_NAME_B2: &str = "Model B2";

static MODEL_EVALUATOR_C: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_C, DMN_3_0089_MODEL_B1, DMN_3_0089_MODEL_B2, DMN_3_0089_MODEL_A]));
const MODEL_NAMESPACE_C: &str = "http://www.trisotech.com/definitions/_10435dcd-8774-4575-a338-49dd554a0928";
const MODEL_NAME_C: &str = "Model C";

#[test]
fn _0001() {
  let ctx = context(r#" { Person name: "Jenny" } "#);
  assert_decision(&MODEL_EVALUATOR_A, MODEL_NAMESPACE_A, MODEL_NAME_A, "Greet the Person", &ctx, r#""Hello, Jenny""#);
}

#[test]
fn _0002() {
  let ctx = context(r#" { Person name: "Waldy" } "#);
  assert_decision(
    &MODEL_EVALUATOR_B1,
    MODEL_NAMESPACE_B1,
    MODEL_NAME_B1,
    "Evaluating Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, Waldy""#,
  );
}

#[test]
fn _0003() {
  let ctx = context(r#" { Model A: { Person name: "John" }} "#);
  assert_decision(
    &MODEL_EVALUATOR_B1,
    MODEL_NAMESPACE_B1,
    MODEL_NAME_B1,
    "Evaluating Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, John""#,
  );
}

#[test]
fn _0004() {
  let ctx = context(
    r#"{
      Person name: "Johnny",
      Model A: { Person name: "John" }
    } "#,
  );
  assert_decision(
    &MODEL_EVALUATOR_B1,
    MODEL_NAMESPACE_B1,
    MODEL_NAME_B1,
    "Evaluating Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, John""#,
  );
}

#[test]
fn _0005() {
  let ctx = context(r#" { Person name: "Cecil" } "#);
  assert_decision(
    &MODEL_EVALUATOR_B2,
    MODEL_NAMESPACE_B2,
    MODEL_NAME_B2,
    "Evaluating B2 Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, Cecil""#,
  );
}

#[test]
fn _0006() {
  let ctx = context(r#" { Model A: { Person name: "Peter" }} "#);
  assert_decision(
    &MODEL_EVALUATOR_B2,
    MODEL_NAMESPACE_B2,
    MODEL_NAME_B2,
    "Evaluating B2 Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, Peter""#,
  );
}

#[test]
fn _0007() {
  let ctx = context(
    r#"{
       Person name: "Patricia",
       Model A: { Person name: "Peter" }
     }"#,
  );
  assert_decision(
    &MODEL_EVALUATOR_B2,
    MODEL_NAMESPACE_B2,
    MODEL_NAME_B2,
    "Evaluating B2 Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, Peter""#,
  );
}

#[test]
fn _0008() {
  let ctx = context(
    r#"{
      Model B1: { Model A: { Person name: "Bob" }},
      Model B2: { Model A: { Person name: "John" }}
    }"#,
  );
  assert_decision(
    &MODEL_EVALUATOR_C,
    MODEL_NAMESPACE_C,
    MODEL_NAME_C,
    "Model C Decision based on Bs",
    &ctx,
    r#""B1: Evaluating Say Hello to: Hello, Bob; B2: Evaluating Say Hello to: Hello, John""#,
  );
}

#[test]
fn _0009() {
  let ctx = context(
    r#"{
      Person name: "Janusz Biznesu",
      Model B1: {
        Person name: "Bobby",
        Model A: { Person name: "Bob" }
      },
      Model B2: {
        Person name: "Johnny",
        Model A: { Person name: "John" }
      }
    }"#,
  );
  assert_decision(
    &MODEL_EVALUATOR_C,
    MODEL_NAMESPACE_C,
    MODEL_NAME_C,
    "Model C Decision based on Bs",
    &ctx,
    r#""B1: Evaluating Say Hello to: Hello, Bob; B2: Evaluating Say Hello to: Hello, John""#,
  );
}
