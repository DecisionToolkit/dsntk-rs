pub fn is_special_character(ch: char) -> bool {
  matches!(ch, '.' | '+' | '*' | '^' | '$' | '?' | '[' | ']' | '(' | ')' | '|' | '{' | '}' | '\\')
}
