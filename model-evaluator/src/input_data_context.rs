//! # Builder for input data context evaluators

use crate::item_definition_context::ItemDefinitionContextEvaluator;
use crate::model_definitions::{DefDefinitions, DefInputData, DefKey};
use crate::type_ref::type_ref_to_feel_type;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::FeelType;
use std::collections::HashMap;

/// Type of closure that evaluates input data context.
type InputDataContextEvaluatorFn = Box<dyn Fn(&mut FeelContext, &ItemDefinitionContextEvaluator) -> FeelType + Send + Sync>;

/// Input data context evaluator.
#[derive(Default)]
pub struct InputDataContextEvaluator {
  evaluators: HashMap<DefKey, InputDataContextEvaluatorFn>,
}

impl InputDataContextEvaluator {
  /// Creates a new input data context evaluator based on provided definitions.
  pub fn new(definitions: &DefDefinitions) -> Self {
    let mut evaluators = HashMap::new();
    for input_data in definitions.input_data() {
      let input_data_namespace = input_data.namespace();
      let input_data_id = input_data.id();
      let evaluator = input_data_context_evaluator(input_data);
      let def_key = DefKey::new(input_data_namespace, input_data_id);
      evaluators.insert(def_key, evaluator);
    }
    Self { evaluators }
  }

  /// Evaluates input data context with specified identifier.
  pub fn eval(&self, def_key: &DefKey, ctx: &mut FeelContext, item_definition_context_evaluator: &ItemDefinitionContextEvaluator) -> FeelType {
    if let Some(evaluator) = self.evaluators.get(def_key) {
      evaluator(ctx, item_definition_context_evaluator)
    } else {
      FeelType::Any
    }
  }
}

///
pub fn input_data_context_evaluator(input_data: &DefInputData) -> InputDataContextEvaluatorFn {
  let namespace = input_data.variable().namespace().to_string();
  let type_ref = input_data.variable().type_ref().to_string();
  let name = input_data.variable().name().clone();
  if let Some(simple_type_ref) = type_ref_to_feel_type(&type_ref) {
    Box::new(move |ctx: &mut FeelContext, _: &ItemDefinitionContextEvaluator| {
      ctx.set_entry(&name, Value::FeelType(simple_type_ref.clone()));
      simple_type_ref.clone()
    })
  } else {
    Box::new(move |ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| evaluator.eval(&DefKey::new(&namespace, &type_ref), &name, ctx))
  }
}

#[cfg(test)]
mod tests {
  use crate::input_data_context::InputDataContextEvaluator;
  use crate::item_definition_context::ItemDefinitionContextEvaluator;
  use crate::model_definitions::{DefDefinitions, DefKey};
  use dsntk_examples::input_data::*;
  use dsntk_feel::context::FeelContext;
  use dsntk_feel::FeelType;

  const NAMESPACE: &str = "https://dsntk.io";

  /// Utility function for building input data context evaluator from definitions,
  /// and item definition context evaluator from definitions.
  fn build_evaluators(xml: &str) -> (InputDataContextEvaluator, ItemDefinitionContextEvaluator) {
    let definitions = dsntk_model::parse(xml).unwrap();
    let mut def_definitions = DefDefinitions::default();
    def_definitions.add_model(&definitions);
    let input_data_context_evaluator = InputDataContextEvaluator::new(&def_definitions);
    let item_definition_context_evaluator = ItemDefinitionContextEvaluator::new(&def_definitions).unwrap();
    (input_data_context_evaluator, item_definition_context_evaluator)
  }

  #[test]
  fn _0001_1() {
    let (evaluator, item_definition_context_evaluator) = build_evaluators(DMN_0001);
    let expected_type = FeelType::String;
    let mut ctx = FeelContext::default();
    let actual_type = evaluator.eval(
      &DefKey::new(NAMESPACE, "_cba86e4d-e91c-46a2-9176-e9adf88e15db"),
      &mut ctx,
      &item_definition_context_evaluator,
    );
    assert_eq!(expected_type, actual_type);
    assert_eq!("{Full Name: type(string)}", ctx.to_string());
  }

  /*
    #[test]
    fn _0001_2() {
      let definitions = &dsntk_model::parse(DMN_0001).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Full Name : 50.0 }"#;
      let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Full", "Name"]), Value::Null(None))),
        input_data_evaluators.eval("_cba86e4d-e91c-46a2-9176-e9adf88e15db", &Value::Context(context), &item_definitions_evaluators)
      );
    }

    #[test]
    fn _0002_1() {
      let definitions = &dsntk_model::parse(DMN_0002).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Monthly Salary : 12000.00 }"#;
      let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Monthly", "Salary"]), Value::Number(12000.0))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
      let context_str = r#"{ Monthly Salary : 8135.35 }"#;
      let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Monthly", "Salary"]), Value::Number(8135.35))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
    }

    #[test]
    fn _0002_2() {
      let definitions = &dsntk_model::parse(DMN_0002).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Monthly Salary : "12000.00" }"#;
      let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Monthly", "Salary"]), Value::Null(None))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
    }

    #[test]
    fn _0003_1() {
      let definitions = &dsntk_model::parse(DMN_0003).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Is Affordable : true }"#;
      let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Is", "Affordable"]), Value::Boolean(true))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
      let context_str = r#"{ Is Affordable : false }"#;
      let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Is", "Affordable"]), Value::Boolean(false))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
    }

    #[test]
    fn _0003_2() {
      let definitions = &dsntk_model::parse(DMN_0003).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Is Affordable : "no" }"#;
      let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Is", "Affordable"]), Value::Null(None))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
    }

    #[test]
    fn _0103_1() {
      let definitions = &dsntk_model::parse(DMN_0103).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Employment Status : "EMPLOYED" }"#;
      let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      let name = Name::new(&["Employment", "Status"]);
      assert_eq!(
        Some((name, Value::String("EMPLOYED".to_string()))),
        input_data_evaluators.eval("_acfd4e1d-da0a-4842-aa35-ea50dd36fb01", &Value::Context(context), &item_definitions_evaluators)
      );
    }

  */
}
