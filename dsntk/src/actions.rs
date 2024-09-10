//! # Command-line actions

use crate::examples::*;
use clap::{arg, command, crate_description, crate_version, Arg, ArgAction, ArgMatches, Command};
use dsntk_common::*;
use dsntk_feel::values::Value;
use dsntk_feel::FeelScope;
use dsntk_feel_parser::ast_tree;
use dsntk_model::{DecisionTable, DmnElement, NamedElement};
use once_cell::sync::Lazy;
use std::fs;
use std::path::Path;

/// Automatic color selection flag.
const COLOR_MODE_AUTO: &str = "auto";

/// Always color selection flag.
const COLOR_MODE_ALWAYS: &str = "always";

/// Color off-switching flag.
const COLOR_MODE_NEVER: &str = "never";

/// Default name for context file.
static DEFAULT_CTX: Lazy<String> = Lazy::new(|| "unknown.ctx".to_string());

/// Default name for expression file.
static DEFAULT_FEEL: Lazy<String> = Lazy::new(|| "unknown.feel".to_string());

/// Default name for HTML file.
static DEFAULT_HTML: Lazy<String> = Lazy::new(|| "unknown.html".to_string());

/// Default name for decision table file.
static DEFAULT_DTB: Lazy<String> = Lazy::new(|| "unknown.dtb".to_string());

/// Default name for decision model file.
static DEFAULT_DMN: Lazy<String> = Lazy::new(|| "unknown.dmn".to_string());

/// Default name for invocable.
static DEFAULT_INVOCABLE: Lazy<String> = Lazy::new(|| "unknown".to_string());

/// Default name for color mode.
static DEFAULT_COLOR: Lazy<String> = Lazy::new(|| "auto".to_string());

/// Default name for examples directory.
static DEFAULT_EXAMPLES_DIR: Lazy<String> = Lazy::new(|| ".".to_string());

/// Command-line actions.
enum Action {
  /// Parse FEEL expression.
  ParseFeelExpression(
    /// Name of the file containing parsing context.
    String,
    /// Name of the file containing FEEL expression to be parsed.
    String,
    /// Requested color mode.
    ColorMode,
  ),
  /// Evaluate FEEL expression.
  EvaluateFeelExpression(
    /// Name of the file containing input data.
    String,
    /// Name of the file containing FEEL expression to be evaluated.
    String,
  ),
  /// Test FEEL expression.
  TestFeelExpression(
    /// Name of the file containing tests.
    String,
    /// Name of the file containing FEEL expression to be tested.
    String,
    /// Flag indicating if only test summary should be printed.
    bool,
    /// Requested color mode.
    ColorMode,
  ),
  /// Export FEEL expression to HTML.
  ExportFeelExpression(
    /// Name of the file containing parsing context.
    String,
    /// Name of the file containing FEEL expression to be exported.
    String,
    /// Output HTML file name.
    String,
  ),
  /// Parse decision table.
  ParseDecisionTable(
    /// Name of the file containing decision table definitions (Unicode format).
    String,
  ),
  /// Evaluate decision table.
  EvaluateDecisionTable(
    /// Name of the file containing input data.
    String,
    /// Name of the file containing decision table definitions to be evaluated (Unicode format).
    String,
  ),
  /// Test decision table.
  TestDecisionTable(
    /// Test file name.
    String,
    /// Decision table file name.
    String,
    /// Flag indicating if only test summary will be printed.
    bool,
    /// Requested color mode.
    ColorMode,
  ),
  /// Export decision table.
  ExportDecisionTable(
    /// Decision table file name.
    String,
    /// Output HTML file name.
    String,
  ),
  /// Recognize decision table.
  RecognizeDecisionTable(
    /// Name of the file containing decision table definitions (Unicode format).
    String,
  ),
  /// Parse DMN model.
  ParseDmnModel(
    /// Name of the file containing DMN model.
    String,
    /// Requested color mode.
    ColorMode,
  ),
  /// Evaluate DMN model.
  EvaluateDmnModel(
    /// Name of the file containing input data.
    String,
    /// Name of the file containing DMN model to be evaluated.
    String,
    /// Name of the invocable to be evaluated.
    String,
  ),
  /// Test DMN model.
  TestDmnModel(
    /// Test file name.
    String,
    /// Decision table file name.
    String,
    /// Invocable name.
    String,
    /// Flag indicating if only test summary will be printed.
    bool,
    /// Requested color mode.
    ColorMode,
  ),
  /// Export DMN model.
  ExportDmnModel(
    /// Name of the file containing DMN model.
    String,
    /// Output HTML file name.
    String,
  ),
  /// Starts as a service.
  StartService(
    /// Optional host name.
    Option<String>,
    /// Optional port number
    Option<String>,
    /// Optional directory containing models to be loaded on start.
    Option<String>,
    /// Requested color mode.
    ColorMode,
    /// Flag indicating if more detailed information should be displayed during startup.
    bool,
  ),
  /// Save examples.
  SaveExamples(
    /// Directory where examples are saved.
    String,
  ),
  /// Do nothing, no action was specified.
  DoNothing,
}

/// Executes command-line action.
pub async fn do_action() -> std::io::Result<()> {
  match get_cli_action() {
    Action::ParseFeelExpression(ctx_file_name, feel_file_name, color_mode) => {
      parse_feel_expression(&ctx_file_name, &feel_file_name, color_mode);
      Ok(())
    }
    Action::EvaluateFeelExpression(input_file_name, feel_file_name) => {
      evaluate_feel_expression(&input_file_name, &feel_file_name);
      Ok(())
    }
    Action::TestFeelExpression(test_file_name, feel_file_name, summary_only, color_mode) => {
      test_feel_expression(&test_file_name, &feel_file_name, summary_only, color_mode);
      Ok(())
    }
    Action::ExportFeelExpression(ctx_file_name, feel_file_name, html_file_name) => {
      export_feel_expression(&ctx_file_name, &feel_file_name, &html_file_name);
      Ok(())
    }
    Action::ParseDecisionTable(dectab_file_name) => {
      parse_decision_table(&dectab_file_name);
      Ok(())
    }
    Action::EvaluateDecisionTable(input_file_name, dectab_file_name) => {
      evaluate_decision_table(&input_file_name, &dectab_file_name);
      Ok(())
    }
    Action::TestDecisionTable(test_file_name, dectab_file_name, summary_only, color_mode) => {
      test_decision_table(&test_file_name, &dectab_file_name, summary_only, color_mode);
      Ok(())
    }
    Action::ExportDecisionTable(dectab_file_name, html_file_name) => {
      export_decision_table(&dectab_file_name, &html_file_name);
      Ok(())
    }
    Action::RecognizeDecisionTable(dectab_file_name) => {
      recognize_decision_table(&dectab_file_name);
      Ok(())
    }
    Action::ParseDmnModel(dmn_file_name, color_mode) => {
      parse_dmn_model(&dmn_file_name, color_mode);
      Ok(())
    }
    Action::EvaluateDmnModel(dmn_file_name, ctx_file_name, invocable_name) => {
      evaluate_dmn_model(&dmn_file_name, &ctx_file_name, &invocable_name);
      Ok(())
    }
    Action::TestDmnModel(test_file_name, dmn_file_name, invocable_name, summary_only, color_mode) => {
      test_dmn_model(&test_file_name, &dmn_file_name, &invocable_name, summary_only, color_mode);
      Ok(())
    }
    Action::ExportDmnModel(dmn_file_name, html_file_name) => {
      export_dmn_model(&dmn_file_name, &html_file_name);
      Ok(())
    }
    Action::StartService(opt_host, opt_port, opt_dir, color, verbose) => {
      // start a service (REST server)
      dsntk_server::start_server(opt_host, opt_port, opt_dir, color.into(), verbose).await
    }
    Action::SaveExamples(root_dir) => {
      // save the examples in the specified root directory
      generate_examples(&root_dir)
    }
    Action::DoNothing => {
      // no specific action was requested
      Ok(())
    }
  }
}

/// Parses CLI argument matches.
fn get_matches() -> ArgMatches {
  command!()
    // disable the built-in version flag
    .disable_version_flag(true)
    // handle the version flag in a custom way
    .arg(Arg::new("version").short('V').long("version").help("Print version").action(ArgAction::SetTrue))
    // pfe
    .subcommand(
      Command::new("pfe")
        .about("Parse FEEL Expression")
        .display_order(7)
        .arg(
          arg!(-c --color <WHEN>)
            .help("Control when colored output is used")
            .value_parser([COLOR_MODE_AUTO, COLOR_MODE_ALWAYS, COLOR_MODE_NEVER])
            .action(ArgAction::Set)
            .display_order(1),
        )
        .arg(arg!(<CONTEXT_FILE>).help("File containing context for parsed FEEL expression").required(true).index(1))
        .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be parsed").required(true).index(2)),
    )
    // efe
    .subcommand(
      Command::new("efe")
        .about("Evaluate FEEL Expression")
        .display_order(4)
        .arg(arg!(<INPUT_FILE>).help("File containing input data for evaluated FEEL expression").required(true).index(1))
        .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be evaluated").required(true).index(2)),
    )
    // tfe
    .subcommand(
      Command::new("tfe")
        .about("Test FEEL Expression")
        .display_order(10)
        .arg(
          arg!(-s - -summary)
            .help("Display only summary after completing all tests")
            .action(ArgAction::SetTrue)
            .display_order(1),
        )
        .arg(
          arg!(-c --color <WHEN>)
            .help("Control when colored output is used")
            .value_parser([COLOR_MODE_AUTO, COLOR_MODE_ALWAYS, COLOR_MODE_NEVER])
            .action(ArgAction::Set)
            .display_order(2),
        )
        .arg(arg!(<TEST_FILE>).help("File containing test cases for tested FEEL expression").required(true).index(1))
        .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be tested").required(true).index(2)),
    )
    // xfe
    .subcommand(
      Command::new("xfe")
        .about("eXport FEEL Expression")
        .display_order(13)
        .arg(
          arg!(<INPUT_FILE>)
            .help("File containing input data for expression to be exported to HTML")
            .required(true)
            .index(1),
        )
        .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be exported to HTML").required(true).index(2))
        .arg(arg!(<HTML_FILE>).help("Output HTML file").required(true).index(3)),
    )
    // pdm
    .subcommand(
      Command::new("pdm")
        .about("Parse DMN Model")
        .display_order(5)
        .arg(
          arg!(-c --color <WHEN>)
            .help("Control when colored output is used")
            .value_parser([COLOR_MODE_AUTO, COLOR_MODE_ALWAYS, COLOR_MODE_NEVER])
            .action(ArgAction::Set)
            .display_order(1),
        )
        .arg(arg!(<DMN_FILE>).help("File containing DMN model to be parsed").required(true).index(1)),
    )
    // edm
    .subcommand(
      Command::new("edm")
        .about("Evaluate DMN Model")
        .display_order(2)
        .arg(
          arg!(-i --invocable <NAME>)
            .help("Name of the invocable (decision, bkm, decision service) to be evaluated")
            .action(ArgAction::Set)
            .required(true)
            .display_order(1),
        )
        .arg(arg!(<INPUT_FILE>).help("File containing input data for evaluated DMN model").required(true).index(1))
        .arg(arg!(<DMN_FILE>).help("File containing DMN model to be evaluated").required(true).index(2)),
    )
    // tdm
    .subcommand(
      Command::new("tdm")
        .about("Test DMN Model")
        .display_order(8)
        .arg(
          arg!(-i --invocable <NAME>)
            .help("Name of the invocable to be tested")
            .required(true)
            .action(ArgAction::Set)
            .display_order(1),
        )
        .arg(
          arg!(-s - -summary)
            .help("Display only summary after completing all tests")
            .action(ArgAction::SetTrue)
            .display_order(2),
        )
        .arg(
          arg!(-c --color <WHEN>)
            .help("Control when colored output is used")
            .value_parser([COLOR_MODE_AUTO, COLOR_MODE_ALWAYS, COLOR_MODE_NEVER])
            .action(ArgAction::Set)
            .display_order(3),
        )
        .arg(arg!(<TEST_FILE>).help("File containing test cases for tested DMN model").required(true).index(1))
        .arg(arg!(<DMN_FILE>).help("File containing DMN model to be tested").required(true).index(2)),
    )
    // xdm
    .subcommand(
      Command::new("xdm")
        .about("eXport DMN Model")
        .display_order(11)
        .arg(arg!(<DMN_FILE>).help("File containing DMN model to be exported to HTML").required(true).index(1))
        .arg(arg!(<HTML_FILE>).help("Output HTML file").required(true).index(2)),
    )
    // pdt
    .subcommand(
      Command::new("pdt")
        .about("Parse Decision Table")
        .display_order(6)
        .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be parsed").required(true).index(1)),
    )
    // edt
    .subcommand(
      Command::new("edt")
        .about("Evaluate Decision Table")
        .display_order(3)
        .arg(arg!(<INPUT_FILE>).help("File containing input data for evaluated decision table").required(true).index(1))
        .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be evaluated").required(true).index(2)),
    )
    // tdt
    .subcommand(
      Command::new("tdt")
        .about("Test Decision Table")
        .display_order(9)
        .arg(
          arg!(-s - -summary)
            .help("Display only summary after completing all tests")
            .action(ArgAction::SetTrue)
            .display_order(1),
        )
        .arg(
          arg!(-c --color <WHEN>)
            .help("Control when colored output is used")
            .value_parser([COLOR_MODE_AUTO, COLOR_MODE_ALWAYS, COLOR_MODE_NEVER])
            .action(ArgAction::Set)
            .display_order(2),
        )
        .arg(arg!(<TEST_FILE>).help("File containing test cases for tested decision table").required(true).index(1))
        .arg(arg!(<DECTAB_FILE>).help("File containing FEEL expression to be tested").required(true).index(2)),
    )
    // xdt
    .subcommand(
      Command::new("xdt")
        .about("eXport Decision Table")
        .display_order(12)
        .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be exported to HTML").required(true).index(1))
        .arg(arg!(<HTML_FILE>).help("Output HTML file").required(true).index(2)),
    )
    // rdt
    .subcommand(
      Command::new("rdt")
        .about("Recognize Decision Table")
        .display_order(14)
        .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be recognized").required(true).index(1)),
    )
    // srv
    .subcommand(
      Command::new("srv")
        .about("Run as a service")
        .display_order(1)
        .arg(arg!(-H --host <HOST>).help("Host name").action(ArgAction::Set).display_order(1))
        .arg(arg!(-P --port <PORT>).help("Port number").action(ArgAction::Set).display_order(2))
        .arg(arg!(-D --dir <DIR>).help("Directory where DMN files are searched").action(ArgAction::Set).display_order(3))
        .arg(
          arg!(-v - -verbose)
            .help("Displays model deployment details during startup")
            .action(ArgAction::SetTrue)
            .display_order(4),
        )
        .arg(
          arg!(-c --color <WHEN>)
            .help("Control when colored output is used")
            .value_parser([COLOR_MODE_AUTO, COLOR_MODE_ALWAYS, COLOR_MODE_NEVER])
            .action(ArgAction::Set)
            .display_order(4),
        ),
    )
    // exs
    .subcommand(
      Command::new("exs")
        .about("Save examples")
        .display_order(15)
        .arg(arg!(<DIR>).help("Directory where examples are saved").action(ArgAction::Set).required(true).index(1)),
    )
    .get_matches()
}

/// Checks the list of arguments passed from the command line
/// and returns an action related to a valid argument.
fn get_cli_action() -> Action {
  let matches = get_matches();
  // replaces built-in version flag with the custom handler
  if matches.get_flag("version") {
    // displays only the version number, without the name of the crate
    println!("{}", crate_version!());
    return Action::DoNothing;
  }
  match matches.subcommand() {
    // parse FEEL expression subcommand
    Some(("pfe", matches)) => {
      return Action::ParseFeelExpression(
        matches.get_one::<String>("CONTEXT_FILE").unwrap_or(&DEFAULT_CTX).to_string(),
        matches.get_one::<String>("FEEL_FILE").unwrap_or(&DEFAULT_FEEL).to_string(),
        matches.get_one::<String>("color").unwrap_or(&DEFAULT_COLOR).to_string().into(),
      );
    }
    // evaluate FEEL expression subcommand
    Some(("efe", matches)) => {
      return Action::EvaluateFeelExpression(
        matches.get_one::<String>("INPUT_FILE").unwrap_or(&DEFAULT_CTX).to_string(),
        matches.get_one::<String>("FEEL_FILE").unwrap_or(&DEFAULT_FEEL).to_string(),
      );
    }
    // test FEEL expression subcommand
    Some(("tfe", matches)) => {
      return Action::TestFeelExpression(
        matches.get_one::<String>("TEST_FILE").unwrap_or(&DEFAULT_CTX).to_string(),
        matches.get_one::<String>("FEEL_FILE").unwrap_or(&DEFAULT_FEEL).to_string(),
        matches.get_flag("summary"),
        matches.get_one::<String>("color").unwrap_or(&DEFAULT_COLOR).to_string().into(),
      );
    }
    // export FEEL expression subcommand
    Some(("xfe", matches)) => {
      return Action::ExportFeelExpression(
        matches.get_one::<String>("INPUT_FILE").unwrap_or(&DEFAULT_CTX).to_string(),
        matches.get_one::<String>("FEEL_FILE").unwrap_or(&DEFAULT_FEEL).to_string(),
        matches.get_one::<String>("HTML_FILE").unwrap_or(&DEFAULT_HTML).to_string(),
      );
    }
    // parse decision table subcommand
    Some(("pdt", matches)) => {
      return Action::ParseDecisionTable(matches.get_one::<String>("DECTAB_FILE").unwrap_or(&DEFAULT_DTB).to_string());
    }
    // evaluate decision table subcommand
    Some(("edt", matches)) => {
      return Action::EvaluateDecisionTable(
        matches.get_one::<String>("INPUT_FILE").unwrap_or(&DEFAULT_CTX).to_string(),
        matches.get_one::<String>("DECTAB_FILE").unwrap_or(&DEFAULT_DTB).to_string(),
      );
    }
    // test decision table subcommand
    Some(("tdt", matches)) => {
      return Action::TestDecisionTable(
        matches.get_one::<String>("TEST_FILE").unwrap_or(&DEFAULT_CTX).to_string(),
        matches.get_one::<String>("DECTAB_FILE").unwrap_or(&DEFAULT_DTB).to_string(),
        matches.get_flag("summary"),
        matches.get_one::<String>("color").unwrap_or(&DEFAULT_COLOR).to_string().into(),
      );
    }
    // export decision table subcommand
    Some(("xdt", matches)) => {
      return Action::ExportDecisionTable(
        matches.get_one::<String>("DECTAB_FILE").unwrap_or(&DEFAULT_DTB).to_string(),
        matches.get_one::<String>("HTML_FILE").unwrap_or(&DEFAULT_HTML).to_string(),
      );
    }
    // recognize decision table subcommand
    Some(("rdt", matches)) => {
      return Action::RecognizeDecisionTable(matches.get_one::<String>("DECTAB_FILE").unwrap_or(&DEFAULT_DTB).to_string());
    }
    // parse DMN model subcommand
    Some(("pdm", matches)) => {
      return Action::ParseDmnModel(
        matches.get_one::<String>("DMN_FILE").unwrap_or(&DEFAULT_DMN).to_string(),
        matches.get_one::<String>("color").unwrap_or(&DEFAULT_COLOR).to_string().into(),
      );
    }
    // evaluate DMN model subcommand
    Some(("edm", matches)) => {
      return Action::EvaluateDmnModel(
        matches.get_one::<String>("INPUT_FILE").unwrap_or(&DEFAULT_CTX).to_string(),
        matches.get_one::<String>("DMN_FILE").unwrap_or(&DEFAULT_DMN).to_string(),
        matches.get_one::<String>("invocable").unwrap_or(&DEFAULT_INVOCABLE).to_string(),
      );
    }
    // test DMN model subcommand
    Some(("tdm", matches)) => {
      return Action::TestDmnModel(
        matches.get_one::<String>("TEST_FILE").unwrap_or(&DEFAULT_CTX).to_string(),
        matches.get_one::<String>("DMN_FILE").unwrap_or(&DEFAULT_DMN).to_string(),
        matches.get_one::<String>("invocable").unwrap_or(&DEFAULT_INVOCABLE).to_string(),
        matches.get_flag("summary"),
        matches.get_one::<String>("color").unwrap_or(&DEFAULT_COLOR).to_string().into(),
      );
    }
    // export DMN model subcommand
    Some(("xdm", matches)) => {
      return Action::ExportDmnModel(
        matches.get_one::<String>("DMN_FILE").unwrap_or(&DEFAULT_DMN).to_string(),
        matches.get_one::<String>("HTML_FILE").unwrap_or(&DEFAULT_HTML).to_string(),
      );
    }
    // start server subcommand
    Some(("srv", matches)) => {
      return Action::StartService(
        matches.get_one::<String>("host").map(|host| host.to_string()),
        matches.get_one::<String>("port").map(|port| port.to_string()),
        matches.get_one::<String>("dir").map(|dir| dir.to_string()),
        matches.get_one::<String>("color").unwrap_or(&DEFAULT_COLOR).to_string().into(),
        matches.get_flag("verbose"),
      );
    }
    // generate examples
    Some(("exs", matches)) => {
      return Action::SaveExamples(matches.get_one::<String>("DIR").unwrap_or(&DEFAULT_EXAMPLES_DIR).to_string());
    }
    _ => {}
  }
  println!("dsntk {}", crate_version!());
  println!("{}", crate_description!());
  println!("dsntk: missing subcommand");
  println!("Try 'dsntk --help' for more information.");
  Action::DoNothing
}

/// Parses `FEEL` expression loaded from file and prints the parsed `AST` to standard output.
fn parse_feel_expression(ctx_file_name: &str, feel_file_name: &str, color_mode: ColorMode) {
  match fs::read_to_string(feel_file_name) {
    Ok(feel_expression) => match fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dsntk_evaluator::evaluate_context(&FeelScope::default(), &context_definition) {
        Ok(ctx) => match dsntk_feel_parser::parse_expression(&ctx.into(), &feel_expression, false) {
          Ok(ast_root_node) => {
            println!("    AST:{}", ast_tree(&ast_root_node, &color_mode).trim_end());
          }
          Err(reason) => eprintln!("parsing expression failed with reason: {reason}"),
        },
        Err(reason) => eprintln!("evaluating context failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading context file `{ctx_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading expression file `{feel_file_name}` failed with reason: {reason}"),
  }
}

/// Evaluates `FEEL` expression loaded from file and prints the result to standard output.
fn evaluate_feel_expression(ctx_file_name: &str, feel_file_name: &str) {
  match fs::read_to_string(feel_file_name) {
    Ok(textual_expression) => match fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dsntk_evaluator::evaluate_context(&FeelScope::default(), &context_definition) {
        Ok(ctx) => match dsntk_feel_parser::parse_expression(&ctx.clone().into(), &textual_expression, false) {
          Ok(ast_root_node) => {
            println!("{}", dsntk_evaluator::evaluate(&ctx.into(), &ast_root_node));
          }
          Err(reason) => eprintln!("parsing expression failed with reason: {reason}"),
        },
        Err(reason) => eprintln!("evaluating context failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading context file `{ctx_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading expression file `{feel_file_name}` failed with reason: {reason}"),
  }
}

/// Tests `FEEL` expression loaded from file and prints the test result to standard output.
fn test_feel_expression(test_file_name: &str, feel_file_name: &str, summary_only: bool, color_mode: ColorMode) {
  match fs::read_to_string(feel_file_name) {
    Ok(feel_file_content) => match fs::read_to_string(test_file_name) {
      Ok(test_file_content) => match dsntk_evaluator::evaluate_test_cases(&test_file_content) {
        Ok(test_cases) => {
          let mut passed = 0_usize;
          let mut failed = 0_usize;
          for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
            let scope = input_data.clone().into();
            match dsntk_feel_parser::parse_expression(&scope, &feel_file_content, false) {
              Ok(node) => {
                let actual = dsntk_evaluator::evaluate(&scope, &node);
                display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only, color_mode);
              }
              Err(reason) => eprintln!("parsing expression failed with reason: {reason}"),
            }
          }
          display_test_summary(passed, failed, summary_only, color_mode);
        }
        Err(reason) => eprintln!("evaluation of test cases failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading test file `{test_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading expression file `{feel_file_name}` failed with reason: {reason}"),
  }
}

/// Exports `FEEL` expression loaded from file to HTML output file.
fn export_feel_expression(_ctx_file_name: &str, _feel_file_name: &str, html_file_name: &str) {
  let _ = fs::write(html_file_name, "not implemented yet\n");
}

/// Parses decision table loaded from text file.
fn parse_decision_table(dectab_file_name: &str) {
  match fs::read_to_string(dectab_file_name) {
    Ok(text) => match dsntk_recognizer::recognize(&text, true) {
      Ok(_) => {}
      Err(reason) => eprintln!("ERROR: {reason}"),
    },
    Err(reason) => eprintln!("loading decision table file `{dectab_file_name}` failed with reason: {reason}"),
  }
}

/// Evaluates context and decision table loaded from files.
fn evaluate_decision_table(input_file_name: &str, dectab_file_name: &str) {
  let input_file_content = match fs::read_to_string(input_file_name) {
    Ok(input_file_content) => input_file_content,
    Err(reason) => {
      eprintln!("loading input file `{input_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let input_data = match dsntk_evaluator::evaluate_context(&FeelScope::default(), &input_file_content) {
    Ok(input_data) => input_data,
    Err(reason) => {
      eprintln!("evaluating input data failed with reason: {reason}");
      return;
    }
  };
  let dtb_file_content = match fs::read_to_string(dectab_file_name) {
    Ok(dtb_file_content) => dtb_file_content,
    Err(reason) => {
      eprintln!("loading input file `{dectab_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let recognized_decision_table = match dsntk_recognizer::recognize(&dtb_file_content, false) {
    Ok(decision_table) => decision_table,
    Err(reason) => {
      eprintln!("building decision table failed with reason: {reason}");
      return;
    }
  };
  let scope = input_data.into();
  let evaluator = match dsntk_evaluator::build_decision_table_evaluator(&scope, &recognized_decision_table.into()) {
    Ok(evaluator) => evaluator,
    Err(reason) => {
      eprintln!("building decision table evaluator failed with reason: {reason}");
      return;
    }
  };
  let result = evaluator(&scope);
  println!("{}", result.jsonify());
}

/// Tests decision table loaded from file.
fn test_decision_table(test_file_name: &str, dectab_file_name: &str, summary_only: bool, color_mode: ColorMode) {
  let dtb_file_content = match fs::read_to_string(dectab_file_name) {
    Ok(dtb_file_content) => dtb_file_content,
    Err(reason) => {
      eprintln!("loading decision table file `{dectab_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let decision_table: DecisionTable = match dsntk_recognizer::recognize(&dtb_file_content, false) {
    Ok(decision_table) => decision_table.into(),
    Err(reason) => {
      eprintln!("building decision table failed with reason: {reason}");
      return;
    }
  };
  let test_file_content = match fs::read_to_string(test_file_name) {
    Ok(test_file_content) => test_file_content,
    Err(reason) => {
      eprintln!("loading test file `{test_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let test_cases = match dsntk_evaluator::evaluate_test_cases(&test_file_content) {
    Ok(test_cases) => test_cases,
    Err(reason) => {
      eprintln!("evaluating test file failed with reason: {reason}");
      return;
    }
  };
  let mut passed = 0_usize;
  let mut failed = 0_usize;
  for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
    let scope = input_data.clone().into();
    let evaluator = match dsntk_evaluator::build_decision_table_evaluator(&scope, &decision_table) {
      Ok(evaluator) => evaluator,
      Err(reason) => {
        eprintln!("building decision table evaluator failed with reason: {reason}");
        return;
      }
    };
    let actual = evaluator(&scope);
    display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only, color_mode);
  }
  display_test_summary(passed, failed, summary_only, color_mode);
}

/// Exports decision table loaded from text file to HTML output file.
fn export_decision_table(dectab_file_name: &str, html_file_name: &str) {
  match fs::read_to_string(dectab_file_name) {
    Ok(text) => match dsntk_recognizer::recognize(&text, false) {
      Ok(recognized_decision_table) => {
        let html_output = dsntk_gendoc::decision_table_to_html(&recognized_decision_table.into());
        if let Err(reason) = fs::write(html_file_name, html_output) {
          println!("writing output HTML file `{html_file_name}` failed with reason: {reason}")
        }
      }
      Err(reason) => eprintln!("ERROR: {reason}"),
    },
    Err(reason) => eprintln!("loading decision table file `{dectab_file_name}` failed with reason: {reason}"),
  }
}

/// Recognizes the decision table loaded from text file
/// and generates DMN model containing recognized decision table.
fn recognize_decision_table(dtb_file_name: &str) {
  match fs::read_to_string(dtb_file_name) {
    Ok(text) => match dsntk_recognizer::recognize(&text, false) {
      Ok(_decision_table) => {
        println!("Recognized.");
        //TODO Generate DMN model with recognized decision table to be ready to deploy on server.
      }
      Err(reason) => eprintln!("ERROR: {reason}"),
    },
    Err(reason) => eprintln!("loading decision table file `{dtb_file_name}` failed with reason: {reason}"),
  }
}

/// Parses DMN model loaded from XML file and prints ASCII report.
fn parse_dmn_model(dmn_file_name: &str, color_mode: ColorMode) {
  match fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => match &dsntk_model::parse(&dmn_file_content) {
      Ok(definitions) => {
        dsntk_gendoc::print_model(definitions, color_mode);
      }
      Err(reason) => eprintln!("parsing model file failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason:?}"),
  }
}

/// Evaluates DMN model loaded from XML file.
fn evaluate_dmn_model(input_file_name: &str, dmn_file_name: &str, invocable_name: &str) {
  match fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => match fs::read_to_string(input_file_name) {
      Ok(input_file_content) => match dsntk_evaluator::evaluate_context(&FeelScope::default(), &input_file_content) {
        Ok(input_data) => match dsntk_model::parse(&dmn_file_content) {
          Ok(definitions) => {
            let model_namespace = definitions.namespace().to_string();
            let model_name = definitions.name().to_string();
            match dsntk_evaluator::ModelEvaluator::new(&[definitions]) {
              Ok(model_evaluator) => {
                let result = model_evaluator.evaluate_invocable(&model_namespace, &model_name, invocable_name, &input_data);
                println!("{}", result.jsonify())
              }
              Err(reason) => eprintln!("building model evaluator failed with reason: {reason}"),
            }
          }
          Err(reason) => eprintln!("parsing model failed with reason: {reason}"),
        },
        Err(reason) => eprintln!("evaluating input data failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading input data file `{input_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason}"),
  }
}

/// Tests DMN model loaded from XML file.
fn test_dmn_model(test_file_name: &str, dmn_file_name: &str, invocable_name: &str, summary_only: bool, color_mode: ColorMode) {
  let dmn_file_content = match fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => dmn_file_content,
    Err(reason) => {
      eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let definitions = match dsntk_model::parse(&dmn_file_content) {
    Ok(definitions) => definitions,
    Err(reason) => {
      eprintln!("parsing model file failed with reason: {reason}");
      return;
    }
  };
  let model_namespace = definitions.namespace().to_string();
  let model_name = definitions.name().to_string();
  let model_evaluator = match dsntk_evaluator::ModelEvaluator::new(&[definitions]) {
    Ok(model_evaluator) => model_evaluator,
    Err(reason) => {
      eprintln!("building model evaluator failed with reason: {reason}");
      return;
    }
  };
  let test_file_content = match fs::read_to_string(test_file_name) {
    Ok(test_file_content) => test_file_content,
    Err(reason) => {
      eprintln!("loading test file `{test_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let test_cases = match dsntk_evaluator::evaluate_test_cases(&test_file_content) {
    Ok(test_cases) => test_cases,
    Err(reason) => {
      eprintln!("evaluating test file failed with reason: {reason}");
      return;
    }
  };
  let mut passed = 0_usize;
  let mut failed = 0_usize;
  for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
    let actual = model_evaluator.evaluate_invocable(&model_namespace, &model_name, invocable_name, input_data);
    display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only, color_mode);
  }
  display_test_summary(passed, failed, summary_only, color_mode);
}

/// Exports DMN model loaded from `XML` file to `HTML` output file.
fn export_dmn_model(dmn_file_name: &str, html_file_name: &str) {
  match fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => match dsntk_model::parse(&dmn_file_content) {
      Ok(definitions) => {
        let html_output = dsntk_gendoc::dmn_model_to_html(&definitions);
        if let Err(reason) = fs::write(html_file_name, html_output) {
          println!("writing output HTML file `{html_file_name}` failed with reason: {reason}")
        }
      }
      Err(reason) => eprintln!("ERROR: {reason}"),
    },
    Err(reason) => eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason}"),
  }
}

/// Generates examples in current directory.
fn generate_examples(root_dir: &str) -> std::io::Result<()> {
  let create_dir = |root_dir: &str, child_dir: &str| -> std::io::Result<()> {
    let root_path = Path::new(root_dir);
    let child_path = Path::new(child_dir);
    let path = root_path.join(child_path);
    fs::create_dir_all(path)?;
    Ok(())
  };
  let write_file = |root_dir: &str, child_dir, contents| -> std::io::Result<()> {
    let root_path = Path::new(root_dir);
    let child_path = Path::new(child_dir);
    let path = root_path.join(child_path);
    fs::write(path, contents)?;
    Ok(())
  };
  create_dir(root_dir, "e1")?;
  write_file(root_dir, "e1/e1.ctx", E1_CTX)?;
  write_file(root_dir, "e1/e1.feel", E1_FEEL)?;
  create_dir(root_dir, "e2")?;
  write_file(root_dir, "e2/e2.ctx", E2_CTX)?;
  write_file(root_dir, "e2/e2.dmn", E2_DMN)?;
  create_dir(root_dir, "e3")?;
  write_file(root_dir, "e3/e3.ctx", E3_CTX)?;
  write_file(root_dir, "e3/e3.dtb", E3_DTB)?;
  Ok(())
}

/// Utility function for displaying test case result.
fn display_test_case_result(actual: &Value, expected: &Value, test_no: &usize, passed: &mut usize, failed: &mut usize, summary_only: bool, color_mode: ColorMode) {
  let color_red = color_red!(color_mode);
  let color_green = color_green!(color_mode);
  let color_reset = color_reset!(color_mode);
  if dsntk_evaluator::evaluate_equals(actual, expected) {
    *passed += 1;
    if !summary_only {
      println!("test {} ... {}ok{}", test_no + 1, color_green, color_reset);
    }
  } else {
    *failed += 1;
    if !summary_only {
      println!("test {} ... {color_red}FAILED{color_reset}", test_no + 1);
      println!("    expected: {color_green}{expected}{color_reset}");
      println!("      actual: {color_red}{actual}{color_reset}");
    }
  }
}

/// Utility function for displaying test summary.
fn display_test_summary(passed: usize, failed: usize, summary_only: bool, color_mode: ColorMode) {
  let color_red = color_red!(color_mode);
  let color_green = color_green!(color_mode);
  let color_reset = color_reset!(color_mode);
  if failed > 0 {
    if summary_only {
      println!("test result: {color_red}FAILED{color_reset}. {passed} passed; {failed} failed.\n");
    } else {
      println!("\ntest result: {color_red}FAILED{color_reset}. {passed} passed; {failed} failed.\n");
    }
  } else if summary_only {
    println!("test result: {color_green}ok{color_reset}. {passed} passed; {failed} failed.\n");
  } else {
    println!("\ntest result: {color_green}ok{color_reset}. {passed} passed; {failed} failed.\n");
  }
}
