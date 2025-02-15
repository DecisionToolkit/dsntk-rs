use crate::errors::{err_empty_flags, err_invalid_flag, err_invalid_pattern};
use crate::unicode_blocks::replace_block_names;
use crate::utils::is_special_character;
use onig::{Regex, RegexOptions, Syntax};

pub fn is_match(input: &str, pattern: &str) -> dsntk_common::Result<bool> {
  let pattern = fix_pattern(pattern, false, false);
  let Ok(re) = Regex::new(&pattern) else {
    return Err(err_invalid_pattern(&pattern));
  };
  Ok(re.is_match(input))
}

pub fn is_match_with_flags(input: &str, pattern: &str, flags: &str) -> dsntk_common::Result<bool> {
  let flags = flags.trim();
  if flags.is_empty() {
    return Err(err_empty_flags());
  }
  let mut options = RegexOptions::REGEX_OPTION_NONE;
  let mut whitespaces = false;
  let mut themselves = false;
  for ch in flags.chars() {
    match ch {
      'i' => options |= RegexOptions::REGEX_OPTION_IGNORECASE,
      'm' => options |= RegexOptions::REGEX_OPTION_MULTILINE,
      's' => options |= RegexOptions::REGEX_OPTION_SINGLELINE,
      'x' => {
        whitespaces = true;
        options |= RegexOptions::REGEX_OPTION_EXTEND;
      }
      'q' => themselves = true,
      _ => return Err(err_invalid_flag(ch)),
    }
  }
  let pattern = fix_pattern(pattern, whitespaces, themselves);
  let Ok(re) = Regex::with_options(&pattern, options, Syntax::default()) else {
    return Err(err_invalid_pattern(&pattern));
  };
  Ok(re.is_match(input))
}

// Nasty tricks on pattern.
fn fix_pattern(pattern: &str, whitespaces: bool, themselves: bool) -> String {
  let mut pattern = replace_block_names(pattern, whitespaces);
  if whitespaces {
    pattern = pattern.replace("\\ ", "\\").replace("[ ]", "[\\ ]");
  }
  if themselves {
    pattern = pattern
      .chars()
      .map(|ch| if is_special_character(ch) { format!(r#"\{}"#, ch) } else { format!("{}", ch) })
      .collect();
  }
  pattern
}
