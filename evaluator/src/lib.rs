mod test_files;

pub use dsntk_feel_evaluator::{evaluate, evaluate_context, evaluate_equals, evaluate_max, evaluate_min, evaluate_sum};
pub use dsntk_model_evaluator::{build_decision_table_evaluator, ModelEvaluator};
pub use test_files::evaluate_test_cases;
