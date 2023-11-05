#![feature(test)]

extern crate test;

use dsntk_feel::{scope, FeelScope};
use dsntk_feel_parser::parse_expression;
use test::Bencher;

#[bench]
fn feel_parser_addition_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"1+2"#;
  b.iter(|| parse_expression(&scope, input, false));
}
