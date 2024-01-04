//! Implementation of the lexer for `FEEL` grammar.

use crate::errors::*;
use crate::lalr::TokenType;
use crate::scope::ParsingScope;
use dsntk_common::Result;
use dsntk_feel::Name;

/// Definition of a single space character.
const WS: char = ' ';

/// Buffer size for lexer input.
const BUF_SIZE: usize = 12;

/// Definition of decimal separator.
const DECIMAL_SEPARATOR: char = '.';

/// Semantic value associated with token type.
#[derive(Debug, Clone)]
pub enum TokenValue {
  YyEmpty,
  YyEof,
  YyError,
  YyUndef,
  YyState(usize),
  And,
  At,
  Between,
  BetweenAnd,
  Boolean(bool),
  BuiltInTypeName(Name),
  Colon,
  Comma,
  Context,
  Div,
  Dot,
  Ellipsis,
  Else,
  Eq,
  Every,
  Exp,
  External,
  For,
  Function,
  LeftBrace,
  LeftBracket,
  LeftParen,
  Le,
  Lt,
  Ge,
  Gt,
  If,
  In,
  Instance,
  List,
  Minus,
  Mul,
  Not,
  Name(Name),
  NameDateTime(Name),
  Nq,
  Null,
  Numeric(String, String, char, String),
  Of,
  Or,
  Plus,
  Range,
  Return,
  RightArrow,
  RightBrace,
  RightBracket,
  RightParen,
  Satisfies,
  Some,
  StartExpression,
  StartBoxedExpression,
  StartRangeLiteral,
  StartSimpleExpression,
  StartSimpleExpressions,
  StartSimpleValue,
  StartTextualExpression,
  StartTextualExpressions,
  StartUnaryTests,
  String(String),
  Then,
}

/// FEEL lexer.
pub struct Lexer<'lexer> {
  /// Parsing scope.
  scope: &'lexer ParsingScope,
  /// Starting token type, returned before the first token.
  start_token_type: Option<TokenType>,
  /// Input characters.
  input: Vec<char>,
  /// Current cursor position in input vector.
  position: usize,
  /// Flag indicating if the unary tests rule is the starting point.
  /// This flag is used to identify `not` keyword,
  /// which otherwise would be recognized as a name.
  /// Token `not` is a keyword at the very beginning of the unary tests rule,
  /// in all other contexts it is just a name.
  unary_tests: bool,
  /// Flag indicating if the `between` keyword was encountered.
  /// When this flag is set, the next `and` token is returned as `band` keyword,
  /// otherwise it is returned as `and`. This allows to disambiguate the `and`
  /// operator used in between clause from conjunction.
  /// After consuming first `and` as `band` this flag is cleared by the lexer.
  between: bool,
  /// ???
  type_name: bool,
  /// ???
  till_in: bool,
}

/// FEEL lexer implementation.
impl<'lexer> Lexer<'lexer> {
  /// Creates a new lexer for specified input text.
  pub fn new(scope: &'lexer ParsingScope, start_token_type: TokenType, input: &str) -> Self {
    Self {
      scope,
      start_token_type: Some(start_token_type),
      input: input.chars().collect(),
      position: 0,
      unary_tests: false,
      between: false,
      type_name: false,
      till_in: false,
    }
  }

  ///
  pub fn set_unary_tests(&mut self) {
    self.unary_tests = true;
  }

  ///
  pub fn set_between(&mut self) {
    self.between = true;
  }

  ///
  pub fn set_type_name(&mut self) {
    self.type_name = true;
  }

  ///
  pub fn set_till_in(&mut self) {
    self.till_in = true;
  }

  ///
  pub fn push_to_scope(&mut self) {
    self.scope.push_default();
  }

  ///
  pub fn pop_from_scope(&mut self) {
    self.scope.pop();
  }

  ///
  pub fn add_name_to_scope(&mut self, name: &Name) {
    self.scope.set_entry_name(name.to_owned());
  }

  /// Returns the next token from input.
  pub fn next_token(&mut self) -> Result<(TokenType, TokenValue)> {
    if let Some(start_token_type) = self.start_token_type.clone() {
      self.start_token_type = None;
      return Ok(match start_token_type {
        tt @ TokenType::StartExpression => (tt, TokenValue::StartExpression),
        tt @ TokenType::StartBoxedExpression => (tt, TokenValue::StartBoxedExpression),
        tt @ TokenType::StartContext => (tt, TokenValue::StartBoxedExpression),
        tt @ TokenType::StartRangeLiteral => (tt, TokenValue::StartRangeLiteral),
        tt @ TokenType::StartSimpleExpression => (tt, TokenValue::StartSimpleExpression),
        tt @ TokenType::StartSimpleExpressions => (tt, TokenValue::StartSimpleExpressions),
        tt @ TokenType::StartSimpleValue => (tt, TokenValue::StartSimpleValue),
        tt @ TokenType::StartTextualExpression => (tt, TokenValue::StartTextualExpression),
        tt @ TokenType::StartTextualExpressions => (tt, TokenValue::StartTextualExpressions),
        tt @ TokenType::StartUnaryTests => (tt, TokenValue::StartUnaryTests),
        _ => (TokenType::YyError, TokenValue::YyError),
      });
    }
    let result = self.read_next_token();
    self.unary_tests = false;
    result
  }

  /// Reads the next token starting from current position.
  fn read_next_token(&mut self) -> Result<(TokenType, TokenValue)> {
    let chars = self.read_input();
    match chars {
      ['s', 'a', 't', 'i', 's', 'f', 'i', 'e', 's', WS, _, _] => {
        self.position += 9;
        Ok((TokenType::Satisfies, TokenValue::Satisfies))
      }
      ['e', 'x', 't', 'e', 'r', 'n', 'a', 'l', WS, _, _, _] => {
        self.position += 8;
        Ok((TokenType::External, TokenValue::External))
      }
      ['f', 'u', 'n', 'c', 't', 'i', 'o', 'n', _, _, _, _] if self.is_function_separator(8) => {
        self.position += 8;
        Ok((TokenType::Function, TokenValue::Function))
      }
      ['i', 'n', 's', 't', 'a', 'n', 'c', 'e', WS, _, _, _] => {
        self.position += 8;
        Ok((TokenType::Instance, TokenValue::Instance))
      }
      ['b', 'e', 't', 'w', 'e', 'e', 'n', WS, _, _, _, _] => {
        self.position += 7;
        Ok((TokenType::Between, TokenValue::Between))
      }
      ['c', 'o', 'n', 't', 'e', 'x', 't', _h, _, _, _, _] if self.is_context_separator(7) => {
        self.position += 7;
        Ok((TokenType::Context, TokenValue::Context))
      }
      ['r', 'e', 't', 'u', 'r', 'n', WS, _, _, _, _, _] => {
        self.position += 6;
        Ok((TokenType::Return, TokenValue::Return))
      }
      ['e', 'v', 'e', 'r', 'y', WS, _, _, _, _, _, _] => {
        self.position += 5;
        Ok((TokenType::Every, TokenValue::Every))
      }
      ['f', 'a', 'l', 's', 'e', ch, _, _, _, _, _, _] if is_separator(ch) => {
        self.position += 5;
        Ok((TokenType::Boolean, TokenValue::Boolean(false)))
      }
      ['r', 'a', 'n', 'g', 'e', _h, _, _, _, _, _, _] if self.is_range_separator(5) => {
        self.position += 5;
        Ok((TokenType::Range, TokenValue::Range))
      }
      ['n', 'u', 'l', 'l', ch, _, _, _, _, _, _, _] if is_separator(ch) => {
        self.position += 4;
        Ok((TokenType::Null, TokenValue::Null))
      }
      ['e', 'l', 's', 'e', WS, _, _, _, _, _, _, _] => {
        self.position += 4;
        Ok((TokenType::Else, TokenValue::Else))
      }
      ['l', 'i', 's', 't', _, _, _, _, _, _, _, _] if self.is_list_separator(4) => {
        self.position += 4;
        Ok((TokenType::List, TokenValue::List))
      }
      ['s', 'o', 'm', 'e', WS, _, _, _, _, _, _, _] => {
        self.position += 4;
        Ok((TokenType::Some, TokenValue::Some))
      }
      ['t', 'h', 'e', 'n', WS, _, _, _, _, _, _, _] => {
        self.position += 4;
        Ok((TokenType::Then, TokenValue::Then))
      }
      ['t', 'r', 'u', 'e', ch, _, _, _, _, _, _, _] if is_separator(ch) => {
        self.position += 4;
        Ok((TokenType::Boolean, TokenValue::Boolean(true)))
      }
      ['a', 'n', 'd', WS, _, _, _, _, _, _, _, _] if !self.between => {
        self.position += 3;
        Ok((TokenType::And, TokenValue::And))
      }
      ['a', 'n', 'd', WS, _, _, _, _, _, _, _, _] if self.between => {
        self.between = false;
        self.position += 3;
        Ok((TokenType::BetweenAnd, TokenValue::BetweenAnd))
      }
      ['f', 'o', 'r', WS, _, _, _, _, _, _, _, _] => {
        self.position += 3;
        Ok((TokenType::For, TokenValue::For))
      }
      ['n', 'o', 't', ch, _, _, _, _, _, _, _, _] if self.unary_tests && is_keyword_not_separator(ch) => {
        self.position += 3;
        Ok((TokenType::Not, TokenValue::Not))
      }
      ['i', 'f', WS, _, _, _, _, _, _, _, _, _] => {
        self.position += 2;
        Ok((TokenType::If, TokenValue::If))
      }
      ['i', 'n', WS, _, _, _, _, _, _, _, _, _] => {
        self.position += 2;
        Ok((TokenType::In, TokenValue::In))
      }
      ['o', 'f', WS, _, _, _, _, _, _, _, _, _] => {
        self.position += 2;
        Ok((TokenType::Of, TokenValue::Of))
      }
      ['o', 'r', WS, _, _, _, _, _, _, _, _, _] => {
        self.position += 2;
        Ok((TokenType::Or, TokenValue::Or))
      }
      ['.', '.', _, _, _, _, _, _, _, _, _, _] => {
        self.position += 2;
        Ok((TokenType::Ellipsis, TokenValue::Ellipsis))
      }
      ['*', '*', _, _, _, _, _, _, _, _, _, _] => {
        self.position += 2;
        Ok((TokenType::Exp, TokenValue::Exp))
      }
      ['!', '=', _, _, _, _, _, _, _, _, _, _] => {
        self.position += 2;
        Ok((TokenType::Ne, TokenValue::Nq))
      }
      ['<', '=', _, _, _, _, _, _, _, _, _, _] => {
        self.position += 2;
        Ok((TokenType::Le, TokenValue::Le))
      }
      ['>', '=', _, _, _, _, _, _, _, _, _, _] => {
        self.position += 2;
        Ok((TokenType::Ge, TokenValue::Ge))
      }
      ['-', '>', _, _, _, _, _, _, _, _, _, _] => {
        self.position += 2;
        Ok((TokenType::RightArrow, TokenValue::RightArrow))
      }
      ['.', ch, _, _, _, _, _, _, _, _, _, _] if is_digit(ch) => self.consume_number(),
      ['.', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::Dot, TokenValue::Dot))
      }
      [',', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::Comma, TokenValue::Comma))
      }
      [':', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::Colon, TokenValue::Colon))
      }
      ['"', _, _, _, _, _, _, _, _, _, _, _] => self.consume_string(),
      ['+', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::Plus, TokenValue::Plus))
      }
      ['-', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::Minus, TokenValue::Minus))
      }
      ['*', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::Mul, TokenValue::Mul))
      }
      ['/', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::Div, TokenValue::Div))
      }
      ['=', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::Eq, TokenValue::Eq))
      }
      ['<', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::Lt, TokenValue::Lt))
      }
      ['>', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::Gt, TokenValue::Gt))
      }
      ['(', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::LeftParen, TokenValue::LeftParen))
      }
      [')', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::RightParen, TokenValue::RightParen))
      }
      ['[', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::LeftBracket, TokenValue::LeftBracket))
      }
      [']', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::RightBracket, TokenValue::RightBracket))
      }
      ['{', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::LeftBrace, TokenValue::LeftBrace))
      }
      ['}', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::RightBrace, TokenValue::RightBrace))
      }
      ['@', _, _, _, _, _, _, _, _, _, _, _] => {
        self.position += 1;
        Ok((TokenType::At, TokenValue::At))
      }
      [ch, _, _, _, _, _, _, _, _, _, _, _] if is_digit(ch) => self.consume_number(),
      [ch, _, _, _, _, _, _, _, _, _, _, _] if is_name_start_char(ch) => self.consume_name(),
      [WS, WS, WS, WS, WS, WS, WS, WS, WS, WS, WS, WS] => Ok((TokenType::YyEof, TokenValue::YyEof)),
      _ => Ok((TokenType::YyUndef, TokenValue::YyUndef)),
    }
  }

  /// Reads characters from input.
  fn read_input(&mut self) -> [char; BUF_SIZE] {
    self.consume_whitespace();
    loop {
      let consumed_comment = self.consume_comment();
      self.consume_whitespace();
      if !consumed_comment {
        break;
      }
    }
    let mut buffer: [char; BUF_SIZE] = [WS; BUF_SIZE];
    for (offset, value) in buffer.iter_mut().enumerate() {
      if let Some(ch) = self.char_at(offset) {
        if !is_whitespace(ch) {
          *value = ch
        };
      }
    }
    buffer
  }

  /// Consumes all whitespace characters starting from the current position.
  /// After consuming a whitespace character the current position is advanced.
  fn consume_whitespace(&mut self) {
    while let Some(ch) = self.char_at(0) {
      if is_whitespace(ch) {
        self.position += 1;
      } else {
        break;
      }
    }
  }

  /// Consumes single comment starting from the current position.
  /// After consuming a comment, the current position is advanced
  /// to the next character after comment.
  fn consume_comment(&mut self) -> bool {
    let pair = (self.char_at(0), self.char_at(1));
    match pair {
      (Some('/'), Some('/')) => {
        self.position += 2;
        while let Some(ch) = self.char_at(0) {
          if ch == '\n' {
            return true;
          }
          self.position += 1;
        }
      }
      (Some('/'), Some('*')) => {
        self.position += 2;
        while let Some(ch) = self.char_at(0) {
          if ch == '*' {
            if let Some('/') = self.char_at(1) {
              self.position += 2;
              return true;
            }
          }
          self.position += 1;
        }
      }
      _ => {}
    }
    false
  }

  /// Consumes the string literal.
  fn consume_string(&mut self) -> Result<(TokenType, TokenValue)> {
    let mut string = "".to_string();
    self.position += 1;
    loop {
      let first = self.char_at(0);
      let second = self.char_at(1);
      match (first, second) {
        (Some('\\'), Some('\'')) => {
          string.push('\'');
          self.position += 2;
        }
        (Some('\\'), Some('"')) => {
          string.push('"');
          self.position += 2;
        }
        (Some('\\'), Some('\\')) => {
          string.push('\\');
          self.position += 2;
        }
        (Some('\\'), Some('n')) => {
          string.push('\n');
          self.position += 2;
        }
        (Some('\\'), Some('r')) => {
          string.push('\r');
          self.position += 2;
        }
        (Some('\\'), Some('t')) => {
          string.push('\t');
          self.position += 2;
        }
        (Some('\\'), Some('u')) => {
          string.push(self.consume_unicode()?);
        }
        (Some('\\'), Some('U')) => {
          string.push(self.consume_unicode()?);
        }
        (Some('"'), _) => {
          self.position += 1;
          break;
        }
        (Some(ch), _) if is_vertical_space(ch) => {
          return Ok((TokenType::YyUndef, TokenValue::YyUndef));
        }
        (Some(ch), _) => {
          string.push(ch);
          self.position += 1;
        }
        _ => return Ok((TokenType::YyEof, TokenValue::YyEof)),
      }
    }
    Ok((TokenType::String, TokenValue::String(string)))
  }

  /// Consumes a number available on input starting from the current position.
  /// When the number is consumed, the current position is incremented accordingly.
  fn consume_number(&mut self) -> Result<(TokenType, TokenValue)> {
    // storage for digits before decimal separator
    let mut digits_before = String::new();
    // storage for digits after decimal separator
    let mut digits_after = String::new();
    // storage for the sign of the exponent ('+' is the default)
    let mut exponent_sign: char = '+';
    // storage for digits of the exponent
    let mut digits_exponent = String::new();
    //------------------------------------------------------------------------------------------------------------------
    // Parse number using a state machine.
    //------------------------------------------------------------------------------------------------------------------
    let mut state = 1;
    loop {
      let Some(ch) = self.char_at(0) else { break };
      match state {
        1 => match ch {
          '0'..='9' => {
            // consume a digit before decimal separator
            digits_before.push(ch);
            self.position += 1;
          }
          DECIMAL_SEPARATOR if self.is_digit_at(1) => {
            // consume decimal separator
            self.position += 1;
            state = 2
          }
          'e' | 'E' => {
            // consume exponent letter
            self.position += 1;
            state = 3
          }
          _ => break,
        },
        2 => match ch {
          '0'..='9' => {
            // consume a digit after decimal separator
            digits_after.push(ch);
            self.position += 1;
          }
          'e' | 'E' => {
            // consume exponent letter
            self.position += 1;
            state = 3
          }
          _ => break,
        },
        3 => match ch {
          '+' => {
            // consume plus - this is the default exponent sign
            self.position += 1;
            state = 4;
          }
          '-' => {
            // consume minus - change the exponent sign
            exponent_sign = '-';
            self.position += 1;
            state = 4;
          }
          '0'..='9' => {
            // consume the first digit of the exponent
            digits_exponent.push(ch);
            self.position += 1;
            state = 4;
          }
          _ => break,
        },
        4 => match ch {
          '0'..='9' => {
            // consume digits of the exponent
            digits_exponent.push(ch);
            self.position += 1;
            state = 4;
          }
          _ => break,
        },
        _ => break,
      }
    }
    if digits_before.is_empty() {
      digits_before.push('0');
    }
    Ok((TokenType::Numeric, TokenValue::Numeric(digits_before, digits_after, exponent_sign, digits_exponent)))
  }

  /// Consumes a name.
  fn consume_name(&mut self) -> Result<(TokenType, TokenValue)> {
    // vector of all name parts
    let mut parts = vec![];
    // currently parsed name part
    let mut current_part = String::new();
    // positions of consumed characters
    let mut consumed_positions = vec![];

    //------------------------------------------------------------------------------------------------------------------
    // The current character on input is already a name start character,
    // so just consume it and advance the input position.
    //------------------------------------------------------------------------------------------------------------------
    let mut ch = self.peek_character();
    current_part.push(ch);

    //------------------------------------------------------------------------------------------------------------------
    // First character of the name was just consumed.
    // Now start parsing the rest of input using a state machine,
    // looking for characters belonging to this name.
    //------------------------------------------------------------------------------------------------------------------
    let mut state = 1;
    loop {
      match state {
        1 => {
          if self.is_next_name_part_char() {
            self.position += 1;
            ch = self.peek_character();
            current_part.push(ch);
          } else {
            parts.push(current_part.clone());
            consumed_positions.push(self.position);
            current_part = String::new();
            state = 2;
          }
        }
        2 => {
          if self.is_next_name_part_char() {
            state = 3;
          } else if self.is_next_additional_name_symbol() {
            state = 4;
          } else if self.is_next_whitespace() {
            state = 5;
          } else {
            self.position += 1;
            break;
          }
        }
        3 => {
          if self.is_next_name_part_char() {
            self.position += 1;
            ch = self.peek_character();
            current_part.push(ch);
          } else {
            parts.push(current_part.clone());
            consumed_positions.push(self.position);
            current_part = String::new();
            state = 2;
          }
        }
        4 => {
          if self.is_next_additional_name_symbol() {
            self.position += 1;
            ch = self.peek_character();
            current_part.push(ch);
            consumed_positions.push(self.position);
            parts.push(current_part.clone());
            current_part = String::new();
          } else {
            state = 2;
          }
        }
        5 => {
          if self.is_next_whitespace() {
            self.position += 1;
            self.peek_character();
          } else {
            state = 2;
          }
        }
        _ => {}
      }
    }

    //==================================================================================================================
    // Now the `parts` vector contains all parts of the longest possible name.
    // Now decide, what kind of name it is.
    //==================================================================================================================

    //------------------------------------------------------------------------------------------------------------------
    // Tweak with the name `item` used in filters.
    // If the first part is `item`, then treat it as a resulting name.
    //------------------------------------------------------------------------------------------------------------------
    const ITEM: &str = "item";
    if let Some(part_name) = parts.first() {
      if part_name == ITEM {
        self.position = consumed_positions[0] + 1;
        return Ok((TokenType::Name, TokenValue::Name(ITEM.into())));
      }
    }

    //------------------------------------------------------------------------------------------------------------------
    // Tweak with the name in `for` and `quantified` expressions.
    // The variable's name is the name before the keyword `in`.
    // Return the name of the local variable before `in` keyword.
    //------------------------------------------------------------------------------------------------------------------
    const KEYWORD_IN: &str = "in";
    if self.till_in {
      if let Some(index) = parts.iter().position(|value| value == KEYWORD_IN) {
        self.till_in = false;
        parts.truncate(index);
        self.position = consumed_positions[index - 1] + 1;
        let name: Name = parts.to_vec().into();
        return Ok((TokenType::Name, TokenValue::Name(name)));
      }
    }

    //------------------------------------------------------------------------------------------------------------------
    // begin with with the longest name containing all parts
    //------------------------------------------------------------------------------------------------------------------
    let mut part_count = parts.len();
    let flattened = self.scope.flattened();
    while part_count > 0 {
      // take a sublist of the original part list until the list is empty
      let part_sublist = &parts[..part_count];
      // flatten the name parts to compare it with built-in names and keys in current scope
      let name = flatten_name_parts(part_sublist);
      // check if the flattened name exists as a key in the current context
      if flattened.contains(&name) {
        // return to the input all characters that do not belong to the name that was found
        self.position = consumed_positions[part_count - 1] + 1;
        let part_vector = part_sublist.to_vec();
        // return the name that exists in the current context
        return Ok((TokenType::Name, TokenValue::Name(part_vector.into())));
      }
      part_count -= 1;
    }

    //------------------------------------------------------------------------------------------------------------------
    // Build the name from name parts.
    //------------------------------------------------------------------------------------------------------------------
    let name: Name = parts.to_vec().into();

    //------------------------------------------------------------------------------------------------------------------
    // Tweak with the names of built-in types.
    // If the name is a built-in type name, then return a type name instead of regular name.
    //------------------------------------------------------------------------------------------------------------------
    if self.type_name
      && matches!(
        name.to_string().as_str(),
        "Any" | "Null" | "boolean" | "number" | "string" | "date" | "date and time" | "time" | "years and months duration" | "days and time duration"
      )
    {
      self.type_name = false;
      return Ok((TokenType::BuiltInTypeName, TokenValue::BuiltInTypeName(name)));
    }

    //------------------------------------------------------------------------------------------------------------------
    // Tweak with `date and time` and `duration` literals.
    // When these names are encountered, then treat them as the names of temporal functions.
    //------------------------------------------------------------------------------------------------------------------
    let name_str = name.to_string();
    if matches!(name_str.as_str(), "date and time" | "duration") {
      return Ok((TokenType::NameDateTime, TokenValue::NameDateTime(name)));
    }

    //------------------------------------------------------------------------------------------------------------------
    // Tweak with `date` and `time` literals.
    // When a colon is encountered after these names, then treat them as a name of the parameter.
    //------------------------------------------------------------------------------------------------------------------
    if matches!(name_str.as_str(), "date" | "time") {
      return if self.is_next_character(&[':'], 0) {
        Ok((TokenType::Name, TokenValue::Name(name)))
      } else {
        Ok((TokenType::NameDateTime, TokenValue::NameDateTime(name)))
      };
    }

    //------------------------------------------------------------------------------------------------------------------
    // By default, return the name as it appears on input.
    //------------------------------------------------------------------------------------------------------------------
    self.scope.add_name(name.clone());
    Ok((TokenType::Name, TokenValue::Name(name)))
  }

  /// Consumes a HEX digit from input or reports an error.
  fn consume_hex_digit(&mut self) -> Result<u64> {
    if let Some(ch) = self.char_at(0) {
      if ch.is_ascii_hexdigit() {
        self.position += 1;
        Ok(hex_to_decimal(ch))
      } else {
        Err(err_expected_hex_digit(ch))
      }
    } else {
      Err(err_unexpected_eof())
    }
  }

  /// Peeks the current character from input.
  fn peek_character(&self) -> char {
    self.char_at(0).unwrap()
  }

  /// Consumes Unicode literal in one of the following forms:
  /// - \u0000 ('\' + 'u' + four hexadecimal characters), or
  /// - \U000000 ('\' + 'U' + six hexadecimal characters).
  fn consume_unicode_literal(&mut self) -> Result<u64> {
    let u = self.char_at(1).unwrap();
    self.position += 2;
    let mut value = 0_u64;
    if u == 'U' {
      value = 1048576 * self.consume_hex_digit()?;
      value += 65536 * self.consume_hex_digit()?;
    }
    value += 4096 * self.consume_hex_digit()?;
    value += 256 * self.consume_hex_digit()?;
    value += 16 * self.consume_hex_digit()?;
    value += self.consume_hex_digit()?;
    Ok(value)
  }

  /// Consumes the UTF-8 encoded character given in one of the following form:
  /// - \u0000 ('\' + 'u' + four hexadecimal characters), or
  /// - \U000000 ('\' + 'U' + six hexadecimal characters).
  fn consume_unicode(&mut self) -> Result<char> {
    let mut value = self.consume_unicode_literal()?;
    match value {
      // one byte UTF-8 value
      0x0000..=0x007F => {
        let b1 = (value & 0x7F) as u8;
        let s = String::from_utf8(vec![b1]).unwrap();
        Ok(s.chars().next().unwrap())
      }
      // two bytes UTF-8 value
      0x0080..=0x07FF => {
        let b2 = ((value & 0x3F) as u8) | 0x80;
        value >>= 6;
        let b1 = ((value & 0x1F) as u8) | 0xC0;
        let s = String::from_utf8(vec![b1, b2]).unwrap();
        Ok(s.chars().next().unwrap())
      }
      // three bytes UTF-8 value
      0x0800..=0xD7FF | 0xE000..=0xFFFF => {
        let b3 = ((value & 0x3F) as u8) | 0x80;
        value >>= 6;
        let b2 = ((value & 0x3F) as u8) | 0x80;
        value >>= 6;
        let b1 = ((value & 0xF) as u8) | 0xE0;
        let s = String::from_utf8(vec![b1, b2, b3]).unwrap();
        Ok(s.chars().next().unwrap())
      }
      // four bytes UTF-8 value
      0x10000..=0x10FFFF => {
        let b4 = ((value & 0x3F) as u8) | 0x80;
        value >>= 6;
        let b3 = ((value & 0x3F) as u8) | 0x80;
        value >>= 6;
        let b2 = ((value & 0x3F) as u8) | 0x80;
        value >>= 6;
        let b1 = ((value & 0x7) as u8) | 0xF0;
        let s = String::from_utf8(vec![b1, b2, b3, b4]).unwrap();
        Ok(s.chars().next().unwrap())
      }
      // value is the high surrogate of UTF-16
      0xD800..=0xDBFF => {
        let low_surrogate = self.consume_unicode_literal()?;
        match low_surrogate {
          0xDC00..=0xDFFF => {
            let mut code_point = 0x10000 + ((value - 0xD800) * 0x400) + (low_surrogate - 0xDC00);
            let b4 = ((code_point & 0xFF) as u8) | 0x80;
            code_point >>= 6;
            let b3 = ((code_point & 0x3F) as u8) | 0x80;
            code_point >>= 6;
            let b2 = ((code_point & 0x3F) as u8) | 0x80;
            code_point >>= 6;
            let b1 = ((code_point & 0x7) as u8) | 0xF0;
            let s = String::from_utf8(vec![b1, b2, b3, b4]).unwrap();
            Ok(s.chars().next().unwrap())
          }
          other => Err(err_unicode_surrogate_out_of_range(other)),
        }
      }
      other => Err(err_unicode_value_out_of_range(other)),
    }
  }

  /// Checks if the next value on input is whitespace character.
  fn is_next_whitespace(&self) -> bool {
    if let Some(ch) = self.char_at(1) {
      is_whitespace(ch)
    } else {
      false
    }
  }

  /// Checks if the next character is the name part character.
  fn is_next_name_part_char(&self) -> bool {
    if let Some(ch) = self.char_at(1) {
      is_name_part_char(ch)
    } else {
      false
    }
  }

  /// Returns **true* when the next character on input is the additional name symbol.
  fn is_next_additional_name_symbol(&self) -> bool {
    if let Some(ch) = self.char_at(1) {
      is_additional_name_symbol(ch)
    } else {
      false
    }
  }

  /// Returns the character at the current position advanced with specified offset.
  fn char_at(&self, offset: usize) -> Option<char> {
    if self.position + offset < self.input.len() {
      Some(self.input[self.position + offset])
    } else {
      None
    }
  }

  /// Returns `true` when the character at the current
  /// position advanced by the offset is a digit.
  fn is_digit_at(&self, offset: usize) -> bool {
    if let Some(ch) = self.char_at(offset) {
      is_digit(ch)
    } else {
      false
    }
  }

  /// Returns `true` when the specified character is a function keyword separator,
  /// that is a character that is allowed after keyword `function`.
  fn is_function_separator(&self, offset: usize) -> bool {
    self.is_next_character(&['(', '<'], offset)
  }

  /// Returns `true` when the specified character is a context keyword separator,
  /// that is a character that is allowed after keyword `context`.
  fn is_context_separator(&self, offset: usize) -> bool {
    self.is_next_character(&['<'], offset)
  }

  /// Returns `true` when the specified character is a range keyword separator,
  /// that is a character that is allowed after keyword `range`.
  fn is_range_separator(&self, offset: usize) -> bool {
    self.is_next_character(&['<'], offset)
  }

  /// Returns `true` when the next character on input is a `list` keyword separator,
  /// that is, a character that is allowed after keyword `list`.
  /// Allowed characters after keyword `list` are whitespace and `<`.
  fn is_list_separator(&self, offset: usize) -> bool {
    self.is_next_character(&['<'], offset)
  }

  /// Returns `true` when the next character on input is a character
  /// from specified `chars` list.
  fn is_next_character(&self, chars: &[char], mut offset: usize) -> bool {
    while let Some(ch) = self.char_at(offset) {
      if chars.contains(&ch) {
        return true;
      } else if !is_whitespace(ch) {
        return false;
      }
      offset += 1;
    }
    false
  }
}

/// Returns `true` when the specified character is a separator (white space equivalent).
fn is_separator(ch: char) -> bool {
  matches!(ch, WS | '=' | '!' | '<' | '>' | '+' | '-' | '*' | '/' | '%' | '.' | ',' | ')' | '[' | ']' | '}')
}

/// Returns **true** when the specified character is an ASCII digit.
fn is_digit(ch: char) -> bool {
  ch.is_ascii_digit()
}

/// Returns `true` when the specified character is a keyword `not` separator,
/// that is a character that is allowed after keyword `not` at the beginning of unary tests.
fn is_keyword_not_separator(ch: char) -> bool {
  matches!(ch, WS | '(')
}

/// Returns `true` when the specified character is an additional name symbol.
/// Specification: 10.3.1.2 Grammar rule, p.120, grammar rule 30.
fn is_additional_name_symbol(ch: char) -> bool {
  matches!(ch, '.' | '/' | '-' | '\'' | '+' | '*')
}

/// Returns `true` when the specified character is name start character.
fn is_name_start_char(ch: char) -> bool {
  matches!(ch, '?' | 'A'..='Z' | '_' | 'a'..='z' |
               '\u{00C0}'..='\u{00D6}' | '\u{00D8}'..='\u{00F6}' | '\u{00F8}'..='\u{02FF}' |
               '\u{0370}'..='\u{037D}' | '\u{037F}'..='\u{1FFF}' | '\u{200C}'..='\u{200D}' |
               '\u{2070}'..='\u{218F}' | '\u{2C00}'..='\u{2FEF}' | '\u{3001}'..='\u{D7FF}' |
               '\u{F900}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}' | '\u{10000}'..='\u{EFFFF}')
}

/// Returns `true` when the specified character is name start character.
fn is_name_part_char(ch: char) -> bool {
  is_name_start_char(ch) || is_digit(ch) || matches!(ch, '\u{00B7}' | '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}')
}

/// Returns `true` when the specified character is a whitespace character.
fn is_whitespace(ch: char) -> bool {
  is_vertical_space(ch)
    || matches!(
      ch,
      '\u{0009}' | '\u{0020}' | '\u{0085}' | '\u{00A0}' | '\u{1680}' | '\u{180E}' | '\u{2000}'
        ..='\u{200B}' | '\u{2028}' | '\u{2029}' | '\u{202F}' | '\u{205F}' | '\u{3000}' | '\u{FEFF}'
    )
}

/// Returns `true` when the specified character is a vertical space.
fn is_vertical_space(ch: char) -> bool {
  matches!(ch, '\u{000A}'..='\u{000D}')
}

/// Return the decimal value that corresponds to hexadecimal digit, case insensitive.
fn hex_to_decimal(ch: char) -> u64 {
  match ch {
    '0' => 0,
    '1' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'A' | 'a' => 10,
    'B' | 'b' => 11,
    'C' | 'c' => 12,
    'D' | 'd' => 13,
    'E' | 'e' => 14,
    'F' | 'f' => 15,
    _ => u64::MAX,
  }
}

/// Joins the parts of the name with single whitespace separator. Every part is trimmed
/// from additional whitespaces at both sides. The final string is also trimmed.
/// After trimming, spaces around additional characters (`.`,`/`,`-`,`'`,`+`,`*`) are removed.
fn flatten_name_parts(parts: &[String]) -> String {
  parts
    .iter()
    .map(|s| s.trim().to_string())
    .collect::<Vec<String>>()
    .join(" ")
    .trim()
    .to_string()
    .replace(" . ", ".")
    .replace(" / ", "/")
    .replace(" - ", "-")
    .replace(" ' ", "'")
    .replace(" + ", "+")
    .replace(" * ", "*")
}

#[cfg(test)]
mod tests {
  use super::{flatten_name_parts, is_additional_name_symbol, is_separator};
  use crate::lexer::hex_to_decimal;

  #[test]
  fn test_is_separator() {
    assert!(is_separator(' '));
    assert!(is_separator('='));
    assert!(is_separator('!'));
    assert!(is_separator('<'));
    assert!(is_separator('>'));
    assert!(is_separator('+'));
    assert!(is_separator('-'));
    assert!(is_separator('*'));
    assert!(is_separator('/'));
    assert!(is_separator('%'));
    assert!(is_separator('.'));
    assert!(is_separator(','));
    assert!(is_separator(')'));
    assert!(is_separator('['));
    assert!(is_separator(']'));
    assert!(is_separator('}'));
    assert!(!is_separator('('));
    assert!(!is_separator('{'));
    assert!(!is_separator('?'));
  }

  #[test]
  fn test_is_additional_name_symbol() {
    assert!(is_additional_name_symbol('/'));
    assert!(is_additional_name_symbol('-'));
    assert!(is_additional_name_symbol('\''));
    assert!(is_additional_name_symbol('+'));
    assert!(is_additional_name_symbol('*'));
    assert!(!is_additional_name_symbol(' '));
    assert!(!is_additional_name_symbol('$'));
    assert!(!is_additional_name_symbol(':'));
    assert!(!is_additional_name_symbol('('));
    assert!(!is_additional_name_symbol(')'));
    assert!(!is_additional_name_symbol('['));
    assert!(!is_additional_name_symbol(']'));
    assert!(!is_additional_name_symbol('{'));
    assert!(!is_additional_name_symbol('}'));
    assert!(!is_additional_name_symbol('?'));
  }

  #[test]
  fn test_flatten_name_parts() {
    assert_eq!("left middle right", flatten_name_parts(&["left".to_string(), "middle".to_string(), "right".to_string()]));
    assert_eq!("a.b", flatten_name_parts(&["   a  ".to_string(), " . ".to_string(), "  b ".to_string()]));
    assert_eq!("a/b", flatten_name_parts(&["   a  ".to_string(), " / ".to_string(), "  b ".to_string()]));
    assert_eq!("a-b", flatten_name_parts(&["   a  ".to_string(), " - ".to_string(), "  b ".to_string()]));
    assert_eq!("a'b", flatten_name_parts(&["   a  ".to_string(), " ' ".to_string(), "  b ".to_string()]));
    assert_eq!("a+b", flatten_name_parts(&["   a  ".to_string(), " + ".to_string(), "  b ".to_string()]));
    assert_eq!("a*b", flatten_name_parts(&["   a  ".to_string(), " * ".to_string(), "  b ".to_string()]));
    assert_eq!("", flatten_name_parts(&["".to_string(), "".to_string(), "".to_string()]));
    assert_eq!("", flatten_name_parts(&[]));
  }

  #[test]
  fn test_hex_to_decimal() {
    assert_eq!(0, hex_to_decimal('0'));
    assert_eq!(1, hex_to_decimal('1'));
    assert_eq!(2, hex_to_decimal('2'));
    assert_eq!(3, hex_to_decimal('3'));
    assert_eq!(4, hex_to_decimal('4'));
    assert_eq!(5, hex_to_decimal('5'));
    assert_eq!(6, hex_to_decimal('6'));
    assert_eq!(7, hex_to_decimal('7'));
    assert_eq!(8, hex_to_decimal('8'));
    assert_eq!(9, hex_to_decimal('9'));
    assert_eq!(10, hex_to_decimal('A'));
    assert_eq!(11, hex_to_decimal('B'));
    assert_eq!(12, hex_to_decimal('C'));
    assert_eq!(13, hex_to_decimal('D'));
    assert_eq!(14, hex_to_decimal('E'));
    assert_eq!(15, hex_to_decimal('F'));
    assert_eq!(10, hex_to_decimal('a'));
    assert_eq!(11, hex_to_decimal('b'));
    assert_eq!(12, hex_to_decimal('c'));
    assert_eq!(13, hex_to_decimal('d'));
    assert_eq!(14, hex_to_decimal('e'));
    assert_eq!(15, hex_to_decimal('f'));
    assert_eq!(u64::MAX, hex_to_decimal('G'));
    assert_eq!(u64::MAX, hex_to_decimal('g'));
  }
}
