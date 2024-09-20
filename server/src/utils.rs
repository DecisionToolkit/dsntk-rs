use std::env;
use std::path::{Path, PathBuf};

/// Returns canonical current directory.
pub fn current_dir() -> Option<PathBuf> {
  let Ok(current_dir) = env::current_dir() else {
    eprintln!("failed to retrieve current directory from operating system");
    return None;
  };
  let Ok(current_dir_path) = current_dir.canonicalize() else {
    eprintln!("failed to canonicalize the current directory");
    return None;
  };
  Some(current_dir_path)
}

pub fn paths_from_variable(variable: &str) -> Vec<PathBuf> {
  let Ok(dir_variable) = env::var(variable) else {
    return vec![];
  };
  if dir_variable.is_empty() {
    return vec![];
  }
  dir_variable
    .split(list_separator())
    .filter_map(|dir| {
      let path = Path::new(&dir);
      if path.exists() && path.is_dir() {
        if let Ok(canonical_path) = path.canonicalize() {
          Some(canonical_path)
        } else {
          eprintln!(
            "failed to canonicalize directory specified in environment variable '{}': [{}]",
            variable,
            path.to_string_lossy()
          );
          None
        }
      } else {
        eprintln!("invalid directory specified in environment variable '{}': [{}]", variable, dir);
        None
      }
    })
    .collect()
}

pub fn paths_from_arguments(args: Vec<String>) -> Vec<PathBuf> {
  if args.is_empty() {
    return vec![];
  }
  args
    .iter()
    .filter_map(|dir| {
      let path = Path::new(&dir);
      if path.exists() && path.is_dir() {
        if let Ok(canonical_path) = path.canonicalize() {
          Some(canonical_path)
        } else {
          eprintln!("failed to canonicalize directory specified as command line argument: [{}]", path.to_string_lossy());
          None
        }
      } else {
        eprintln!("invalid directory specified as command line argument: [{}]", dir);
        None
      }
    })
    .collect()
}

/// Returns list separator.
#[cfg(not(target_os = "windows"))]
fn list_separator() -> char {
  ':'
}

/// Returns list separator.
#[cfg(target_os = "windows")]
fn list_separator() -> char {
  ';'
}

#[cfg(test)]
mod tests {
  use super::*;

  const DSNTK_DIR_TEST: &str = "DSNTK_DIR_TEST";

  #[test]
  fn getting_current_dir_should_work() {
    assert!(current_dir().unwrap().to_string_lossy().ends_with("/dsntk-rs/server"));
  }

  #[test]
  fn getting_paths_from_non_existing_variable_should_work() {
    assert_eq!(Vec::<PathBuf>::new(), paths_from_variable("non_existing_variable_name"));
  }

  #[test]
  fn getting_paths_from_empty_variable_should_work() {
    env::set_var(DSNTK_DIR_TEST, "");
    assert_eq!(Vec::<PathBuf>::new(), paths_from_variable(DSNTK_DIR_TEST));
  }

  #[test]
  fn getting_single_path_from_variable_should_work() {
    env::set_var(DSNTK_DIR_TEST, current_dir().unwrap());
    assert!(paths_from_variable(DSNTK_DIR_TEST).first().unwrap().to_string_lossy().ends_with("/dsntk-rs/server"));
  }

  #[test]
  fn getting_multiple_paths_from_variable_should_work() {
    env::set_var(
      DSNTK_DIR_TEST,
      format!(
        "{}{}{}",
        current_dir().unwrap().to_string_lossy(),
        list_separator(),
        current_dir().unwrap().to_string_lossy()
      ),
    );
    let paths = paths_from_variable(DSNTK_DIR_TEST);
    assert_eq!(2, paths.len());
    assert!(paths.first().unwrap().to_string_lossy().ends_with("/dsntk-rs/server"));
    assert!(paths.get(1).unwrap().to_string_lossy().ends_with("/dsntk-rs/server"));
  }

  #[test]
  fn getting_multiple_paths_from_variable_should_work_1() {
    env::set_var(DSNTK_DIR_TEST, format!("{}{}{}", current_dir().unwrap().to_string_lossy(), list_separator(), ""));
    let paths = paths_from_variable(DSNTK_DIR_TEST);
    assert_eq!(1, paths.len());
    assert!(paths.first().unwrap().to_string_lossy().ends_with("/dsntk-rs/server"));
  }

  #[test]
  fn getting_paths_from_empty_command_line_arguments_should_work() {
    let args = vec![];
    let paths = paths_from_arguments(args);
    assert_eq!(0, paths.len());
  }

  #[test]
  fn getting_single_path_from_command_line_arguments_should_work() {
    let args = vec![current_dir().unwrap().to_string_lossy().to_string()];
    let paths = paths_from_arguments(args);
    assert_eq!(1, paths.len());
    assert!(paths.first().unwrap().to_string_lossy().ends_with("/dsntk-rs/server"));
  }

  #[test]
  fn getting_multiple_paths_from_command_line_arguments_should_work() {
    let args = vec![current_dir().unwrap().to_string_lossy().to_string(), current_dir().unwrap().to_string_lossy().to_string()];
    let paths = paths_from_arguments(args);
    assert_eq!(2, paths.len());
    assert!(paths.first().unwrap().to_string_lossy().ends_with("/dsntk-rs/server"));
    assert!(paths.get(1).unwrap().to_string_lossy().ends_with("/dsntk-rs/server"));
  }
}
