use dsntk_examples::decision_tables::H_000010;
use dsntk_recognizer::recognize_from_unicode;

const HIT_POLICY: &str = r#"
 ┌───╥─────────────┐
 │ C ║             │
 ╞═══╬═════════════╡
 │ 1 ║  "Monday"   │
 ├───╫─────────────┤
 │ 2 ║  "Tuesday"  │
 ├───╫─────────────┤
 │ 3 ║ "Wednesday" │
 └───╨─────────────┘
"#;

#[test]
fn _0001() {
  let _ = recognize_from_unicode(
    r#"
 ┌───╥─────────────┐
 │ A ║             │
 ╞═══╬═════════════╡
 │ 1 ║  "Monday"   │
 ├───╫─────────────┤
 │ 2 ║  "Tuesday"  │
 ├───╫─────────────┤
 │ 3 ║ "Wednesday" │
 └───╨─────────────┘
"#,
    false,
  );
}
