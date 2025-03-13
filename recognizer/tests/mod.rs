use walkdir::WalkDir;

mod markdown;
mod unicode;

/// aaa
#[test]
fn check_decision_tables_tests() {
  let dir_examples_horizontal = "../examples/src/decision_tables/horizontal";
  for entry_result in WalkDir::new(dir_examples_horizontal).into_iter() {
    let entry = entry_result.unwrap();
    let path = entry.path();
    if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
      let file_name = path.strip_prefix(dir_examples_horizontal).unwrap().display().to_string();
      if file_name.starts_with("H_") {
        println!("{}", file_name);
      }
    }
  }
}
