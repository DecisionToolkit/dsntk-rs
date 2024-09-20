//! # Workspace builder

use dsntk_common::{encode_segments, to_rdnn, ColorPalette};
use dsntk_model::{Definitions, DmnElement};
use dsntk_model_evaluator::ModelEvaluator;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

/// Workspace builder.
pub struct WorkspaceBuilder {
  /// Color palette based on color mode.
  colors: ColorPalette,
  /// Flag indicating the level of verbosity.
  verbose: bool,
  /// Total number of model files found.
  file_count: usize,
  /// The number of successfully loaded models.
  loaded_count: usize,
  /// The number of models that failed to load.
  failed_loads_count: usize,
  /// The number of workspaces that failed to load.
  failed_deployments_count: usize,
  /// Map: workspace name -> model definitions.
  workspace_definitions: HashMap<String, Vec<Definitions>>,
  /// Map: workspace name -> namespaces in workspace
  workspace_namespaces: HashMap<String, HashSet<String>>,
  /// Map: workspace name -> (file_name, rdnn)
  workspace_models: HashMap<String, HashMap<String, String>>,
  /// Map: invocable path -> (workspace name, model namespace, model name, invocable name)
  pub(crate) invocables: HashMap<String, (String, String, String, String)>,
  /// Map: workspace name -> model evaluator
  pub(crate) evaluators: HashMap<String, Arc<ModelEvaluator>>,
}

impl WorkspaceBuilder {
  /// Creates a new workspace builder.
  pub fn new(colors: ColorPalette, verbose: bool) -> Self {
    Self {
      colors,
      verbose,
      file_count: 0,
      loaded_count: 0,
      failed_loads_count: 0,
      failed_deployments_count: 0,
      workspace_definitions: Default::default(),
      workspace_namespaces: Default::default(),
      workspace_models: Default::default(),
      invocables: Default::default(),
      evaluators: Default::default(),
    }
  }

  /// Loads decision models from files and builds the workspaces.
  pub fn load_decision_models(&mut self, dir: &Path) {
    // load model files
    for entry_result in WalkDir::new(dir).into_iter() {
      match entry_result {
        Ok(entry) => {
          let path = entry.path();
          if path.is_file() && path.extension().map_or(false, |ext| ext == "dmn") {
            self.file_count += 1;
            let workspace_name = self.workspace_name(dir, path);
            self.load_file(&workspace_name, path);
          }
        }
        Err(reason) => self.err_file_operation(reason.to_string()),
      }
    }
    // build evaluators
    for (workspace_name, loaded_definitions) in &self.workspace_definitions {
      match ModelEvaluator::new(loaded_definitions) {
        Ok(evaluator) => {
          self.evaluators.insert(workspace_name.to_string(), evaluator);
        }
        Err(reason) => {
          self.err_deployment_failure(workspace_name, reason.to_string());
          self.failed_deployments_count += 1;
        }
      }
    }
    // build invocable paths
    for (workspace_name, evaluator) in &self.evaluators {
      for (model_namespace, model_name, invocable_name) in evaluator.invocables().list() {
        let invocable_path = format!(
          "{}{}/{}/{}",
          if !workspace_name.is_empty() { format!("{}/", workspace_name) } else { "".to_string() },
          to_rdnn(&model_namespace).unwrap(),
          model_name,
          invocable_name
        );
        self
          .invocables
          .insert(invocable_path, (workspace_name.clone(), model_namespace, model_name, invocable_name));
      }
    }
  }

  /// Checks if namespaces are duplicated in the workspace.
  fn check_namespace_duplicates(&self, file: &Path, workspace_name: &str, namespace: &str) -> bool {
    if let Some(namespaces) = self.workspace_namespaces.get(workspace_name) {
      if namespaces.contains(namespace) {
        let file_name = self.workspace_models.get(workspace_name).unwrap().get(namespace).unwrap();
        self.err_duplicated_namespace(workspace_name, file, namespace, file_name);
        return false;
      }
    }
    true
  }

  /// Loads decision model from file.
  fn load_file(&mut self, workspace_name: &str, file: &Path) {
    match fs::read_to_string(file) {
      Ok(xml) => match dsntk_model::parse(&xml) {
        Ok(definitions) => {
          let namespace = definitions.namespace().to_string();
          if to_rdnn(&namespace).is_some() {
            if self.check_namespace_duplicates(file, workspace_name, &namespace) {
              self
                .workspace_definitions
                .entry(workspace_name.to_string())
                .and_modify(|loaded_definitions| {
                  loaded_definitions.push(definitions.clone());
                })
                .or_insert(vec![definitions]);
              self
                .workspace_namespaces
                .entry(workspace_name.to_string())
                .and_modify(|loaded_namespaces| {
                  loaded_namespaces.insert(namespace.clone());
                })
                .or_insert({
                  let mut set = HashSet::new();
                  set.insert(namespace.clone());
                  set
                });
              self
                .workspace_models
                .entry(workspace_name.to_string())
                .and_modify(|loaded_models| {
                  loaded_models.insert(namespace.clone(), file.to_string_lossy().to_string());
                })
                .or_insert({
                  let mut map = HashMap::new();
                  map.insert(namespace.clone(), file.to_string_lossy().to_string());
                  map
                });
              self.loaded_count += 1;
            }
          } else {
            self.err_invalid_namespace(workspace_name, file, &namespace);
            self.failed_loads_count += 1;
          }
        }
        Err(reason) => {
          self.err_file_load(workspace_name, file, reason.to_string());
          self.failed_loads_count += 1;
        }
      },
      Err(reason) => {
        self.err_file_load(workspace_name, file, reason.to_string());
        self.failed_loads_count += 1;
      }
    }
  }

  /// Displays loading process summary.
  pub fn display_summary(&self) {
    println!(
      "{1}Found {2} {3}.{0}",
      self.colors.clear(),
      if self.file_count > 0 { self.colors.green() } else { self.colors.red() },
      self.file_count,
      Self::plural("model", self.file_count)
    );
    if self.loaded_count > 0 {
      println!(
        "{1}Loaded {2} {3}.{0}",
        self.colors.clear(),
        self.colors.green(),
        self.loaded_count,
        Self::plural("model", self.loaded_count)
      );
    }
    if self.failed_loads_count > 0 {
      println!(
        "{1}Failed to load {2} {3}.{0}",
        self.colors.clear(),
        self.colors.red(),
        self.failed_loads_count,
        Self::plural("model", self.failed_loads_count)
      );
    }
    let deployed_invocables_count = self.evaluators.values().map(|evaluator| evaluator.invocables().len()).sum();
    println!(
      "{1}Deployed {2} {3}.{0}",
      self.colors.clear(),
      self.colors.green(),
      deployed_invocables_count,
      Self::plural("invocable", deployed_invocables_count)
    );
    if self.failed_deployments_count > 0 {
      println!(
        "{1}Failed to deploy {2} {3}.{0}",
        self.colors.clear(),
        self.colors.red(),
        self.failed_deployments_count,
        Self::plural("workspace", self.failed_deployments_count)
      );
    }
    if self.verbose {
      self.display_deployed_invocables();
    }
  }

  /// Displays the names of the deployed invocables (encoded URLs).
  fn display_deployed_invocables(&self) {
    let mut invocable_paths = self.invocables.keys().cloned().collect::<Vec<String>>();
    invocable_paths.sort();
    let invocable_count = invocable_paths.len();
    if invocable_count > 0 {
      println!("{1}\nDeployed invocables:{0}", self.colors.clear(), self.colors.yellow());
    }
    for key in invocable_paths {
      if let Some((workspace_name, model_namespace, model_name, invocable_name)) = self.invocables.get(&key) {
        print!("  ");
        // workspace name containing the directory structure, URL encoded
        let segment_1 = encode_segments(workspace_name);
        if !segment_1.is_empty() {
          print!("{1}{2}{0}/", self.colors.clear(), self.colors.magenta(), segment_1);
        }
        // model namespace converted to DNN, URL encoded
        let rdnn = to_rdnn(model_namespace).unwrap_or_default();
        let segment_2 = encode_segments(&rdnn);
        if !segment_2.is_empty() {
          print!("{1}{2}{0}/", self.colors.clear(), self.colors.cyan(), segment_2);
        }
        // model name, URL encoded
        let segment_3 = encode_segments(model_name);
        if !segment_3.is_empty() {
          print!("{1}{2}{0}/", self.colors.clear(), self.colors.magenta(), segment_3);
        }
        // invocable name, URL encoded
        let segment_4 = encode_segments(invocable_name);
        if !segment_4.is_empty() {
          print!("{1}{2}{0}", self.colors.clear(), self.colors.cyan(), segment_4);
        }
        println!();
      }
    }
    if invocable_count > 0 {
      println!();
    }
  }

  /// Returns workspace name created from parent and child paths.
  fn workspace_name(&self, parent_path: &Path, child_path: &Path) -> String {
    let canonical_child_path = child_path.canonicalize().unwrap();
    let mut workspace_name = canonical_child_path.parent().unwrap();
    if let Some(parent_path) = parent_path.canonicalize().unwrap().parent() {
      workspace_name = workspace_name.strip_prefix(parent_path).unwrap();
    }
    workspace_name
      .to_string_lossy()
      .replace('\\', "/")
      .trim_start_matches('/')
      .trim_end_matches('/')
      .to_string()
  }

  /// Returns a noun in plural form, depending on specified numeric value.
  fn plural(noun: &str, number: usize) -> String {
    if number == 1 {
      noun.to_string()
    } else {
      format!("{}s", noun)
    }
  }

  /// Prints file loading error details.
  fn err_file_load(&self, workspace_name: &str, file: &Path, reason: String) {
    eprintln!(
      "[{1}error{0}][{2}{3}{0}] {1}{4}{0}",
      self.colors.clear(),
      self.colors.red(),
      self.colors.blue(),
      self.join_names(workspace_name, file),
      reason
    );
  }

  /// Prints duplicated namespace error details.
  fn err_duplicated_namespace(&self, workspace_name: &str, file: &Path, namespace: &str, file_name: &str) {
    eprintln!(
      "[{1}error{0}][{2}{3}{0}] {1}duplicated namespace '{4}' in file {5}{0}",
      self.colors.clear(),
      self.colors.red(),
      self.colors.blue(),
      self.join_names(workspace_name, file),
      namespace,
      self.join_names(workspace_name, Path::new(file_name)),
    );
  }

  /// Prints invalid namespace error details.
  fn err_invalid_namespace(&self, workspace_name: &str, file: &Path, namespace: &str) {
    eprintln!(
      "[{1}error{0}][{2}{3}{0}] {1}invalid namespace: '{4}'{0}",
      self.colors.clear(),
      self.colors.red(),
      self.colors.blue(),
      self.join_names(workspace_name, file),
      namespace,
    );
  }

  /// Prints deployment error details.
  fn err_deployment_failure(&self, workspace_name: &str, reason: String) {
    eprintln!(
      "[{1}error{0}][{2}{3}{0}] {1}deployment failed with reason: {4}{0}",
      self.colors.clear(),
      self.colors.red(),
      self.colors.blue(),
      if workspace_name.is_empty() { "." } else { workspace_name },
      reason
    );
  }

  /// Prints file operation error details.
  fn err_file_operation(&self, reason: String) {
    eprintln!("[{1}error{0}] {1}{2}{0}", self.colors.clear(), self.colors.red(), reason);
  }

  /// Joins the workspace name with the name of the file.
  fn join_names(&self, workspace_name: &str, file: &Path) -> String {
    format!("{}/{}", workspace_name, file.file_name().unwrap().to_string_lossy())
  }
}
