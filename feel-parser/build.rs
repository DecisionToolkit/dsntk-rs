use dsntk_feel_grammar::lalr_rust_tables;

fn main() {
  if std::env::var("CARGO_FEATURE_PARSING_TABLES").is_ok() {
    // generate new LALR parsing tables for FEEL grammar
    // when executed with --features=parsing-tables
    lalr_rust_tables("./src/lalr.rs");
  }
}
