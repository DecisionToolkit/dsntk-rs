//! # Terminal color control sequences

/// Control sequence for text color _**black**_.
pub const ASCII_BLACK: &str = "\u{1b}[30m";
/// Control sequence for text color _**red**_.
pub const ASCII_RED: &str = "\u{1b}[31m";
/// Control sequence for text color _**green**_.
pub const ASCII_GREEN: &str = "\u{1b}[32m";
/// Control sequence for text color _**yellow**_.
pub const ASCII_YELLOW: &str = "\u{1b}[33m";
/// Control sequence for text color _**blue**_.
pub const ASCII_BLUE: &str = "\u{1b}[34m";
/// Control sequence for text color _**magenta**_.
pub const ASCII_MAGENTA: &str = "\u{1b}[35m";
/// Control sequence for text color _**cyan**_.
pub const ASCII_CYAN: &str = "\u{1b}[36m";
/// Control sequence for text color _**white**_.
pub const ASCII_WHITE: &str = "\u{1b}[37m";

/// Control sequence for text color _**bright black**_.
pub const ASCII_BRIGHT_BLACK: &str = "\u{1b}[30;1m";
/// Control sequence for text color _**bright red**_.
pub const ASCII_BRIGHT_RED: &str = "\u{1b}[31;1m";
/// Control sequence for text color _**bright green**_.
pub const ASCII_BRIGHT_GREEN: &str = "\u{1b}[32;1m";
/// Control sequence for text color _**bright yellow**_.
pub const ASCII_BRIGHT_YELLOW: &str = "\u{1b}[33;1m";
/// Control sequence for text color _**bright blue**_.
pub const ASCII_BRIGHT_BLUE: &str = "\u{1b}[34;1m";
/// Control sequence for text color _**bright magenta**_.
pub const ASCII_BRIGHT_MAGENTA: &str = "\u{1b}[35;1m";
/// Control sequence for text color _**bright cyan**_.
pub const ASCII_BRIGHT_CYAN: &str = "\u{1b}[36;1m";
/// Control sequence for text color _**bright white**_.
pub const ASCII_BRIGHT_WHITE: &str = "\u{1b}[37;1m";

/// Control sequence for background color _**black**_.
pub const ASCII_BG_BLACK: &str = "\u{1b}[40m";
/// Control sequence for background color _**red**_.
pub const ASCII_BG_RED: &str = "\u{1b}[41m";
/// Control sequence for background color _**green**_.
pub const ASCII_BG_GREEN: &str = "\u{1b}[42m";
/// Control sequence for background color _**yellow**_.
pub const ASCII_BG_YELLOW: &str = "\u{1b}[43m";
/// Control sequence for background color _**blue**_.
pub const ASCII_BG_BLUE: &str = "\u{1b}[44m";
/// Control sequence for background color _**magenta**_.
pub const ASCII_BG_MAGENTA: &str = "\u{1b}[45m";
/// Control sequence for background color _**cyan**_.
pub const ASCII_BG_CYAN: &str = "\u{1b}[46m";
/// Control sequence for background color _**white**_.
pub const ASCII_BG_WHITE: &str = "\u{1b}[47m";

/// Control sequence for background color _**bright black**_.
pub const ASCII_BG_BRIGHT_BLACK: &str = "\u{1b}[40;1m";
/// Control sequence for background color _**bright red**_.
pub const ASCII_BG_BRIGHT_RED: &str = "\u{1b}[41;1m";
/// Control sequence for background color _**bright green**_.
pub const ASCII_BG_BRIGHT_GREEN: &str = "\u{1b}[42;1m";
/// Control sequence for background color _**bright yellow**_.
pub const ASCII_BG_BRIGHT_YELLOW: &str = "\u{1b}[43;1m";
/// Control sequence for background color _**bright blue**_.
pub const ASCII_BG_BRIGHT_BLUE: &str = "\u{1b}[44;1m";
/// Control sequence for background color _**bright magenta**_.
pub const ASCII_BG_BRIGHT_MAGENTA: &str = "\u{1b}[45;1m";
/// Control sequence for background color _**bright cyan**_.
pub const ASCII_BG_BRIGHT_CYAN: &str = "\u{1b}[46;1m";
/// Control sequence for background color _**bright white**_.
pub const ASCII_BG_BRIGHT_WHITE: &str = "\u{1b}[47;1m";

/// Control sequence for cancelling all the previously used settings.
pub const ASCII_CLEAR: &str = "\u{1b}[0m";

/// Color mode to switch terminal colouring `ON` and `OFF`.
#[derive(Copy, Clone, PartialEq)]
pub enum ColorMode {
  /// Switch colouring **on**.
  On,
  /// Switch colouring **off**.
  Off,
}

impl From<String> for ColorMode {
  /// Converts a string into [ColorMode].
  fn from(value: String) -> Self {
    Self::from(&value)
  }
}

impl From<&String> for ColorMode {
  /// Converts a string into [ColorMode].
  fn from(value: &String) -> Self {
    match value.to_lowercase().trim() {
      "never" => Self::Off,
      "always" => Self::On,
      _ => {
        if atty::is(atty::Stream::Stdout) {
          Self::On
        } else {
          Self::Off
        }
      }
    }
  }
}

/// Returns a color control string based on [ColorMode] for specified color number.
///
/// Color number must be in range 0..255, otherwise an empty string is returned.
///
/// # Examples
///
/// ```
/// use dsntk_common::{ColorMode, color_256};
///
/// assert_eq!("\u{1b}[38;5;0m", color_256!(ColorMode::On, 0));
/// assert_eq!("\u{1b}[38;5;28m", color_256!(ColorMode::On, 28));
/// assert_eq!("\u{1b}[38;5;255m", color_256!(ColorMode::On, 255));
/// assert_eq!("", color_256!(ColorMode::On, -1));
/// assert_eq!("", color_256!(ColorMode::On, 256));
/// assert_eq!("", color_256!(ColorMode::Off, 28));
/// ```
#[macro_export]
macro_rules! color_256 {
  ($color_mode:expr,$color_number:expr) => {{
    match $color_mode {
      ColorMode::On if (0..256).contains(&$color_number) => format!("\u{1b}[38;5;{}m", $color_number),
      _ => "".to_string(),
    }
  }};
}

/// Internal utility macro for building exported color macros.
macro_rules! make_color_macro {
  ($color:tt,$name:tt) => {
    #[doc = concat!("Returns a color control string as defined in [", stringify!($color), "] based on [ColorMode].")]
    #[doc = "# Examples"]
    #[doc = "```"]
    #[doc = concat!("use dsntk_common::{ColorMode, ", stringify!($name), ", ", stringify!($color), "};")]
    #[doc = ""]
    #[doc = concat!("assert_eq!(", stringify!($color), ", ", stringify!($name), "!(ColorMode::On));")]
    #[doc = concat!("assert_eq!(\"\"", ", ", stringify!($name), "!(ColorMode::Off));")]
    #[doc = "```"]
    #[macro_export]
    macro_rules! $name {
      ($color_mode:expr) => {{
        use dsntk_common::$color;
        match $color_mode {
          ColorMode::On => $color,
          _ => "",
        }
        .to_string()
      }};
    }
  };
}

make_color_macro!(ASCII_BLACK, color_black);
make_color_macro!(ASCII_RED, color_red);
make_color_macro!(ASCII_GREEN, color_green);
make_color_macro!(ASCII_YELLOW, color_yellow);
make_color_macro!(ASCII_BLUE, color_blue);
make_color_macro!(ASCII_MAGENTA, color_magenta);
make_color_macro!(ASCII_CYAN, color_cyan);
make_color_macro!(ASCII_WHITE, color_white);

make_color_macro!(ASCII_BRIGHT_BLACK, color_bright_black);
make_color_macro!(ASCII_BRIGHT_RED, color_bright_red);
make_color_macro!(ASCII_BRIGHT_GREEN, color_bright_green);
make_color_macro!(ASCII_BRIGHT_YELLOW, color_bright_yellow);
make_color_macro!(ASCII_BRIGHT_BLUE, color_bright_blue);
make_color_macro!(ASCII_BRIGHT_MAGENTA, color_bright_magenta);
make_color_macro!(ASCII_BRIGHT_CYAN, color_bright_cyan);
make_color_macro!(ASCII_BRIGHT_WHITE, color_bright_white);

make_color_macro!(ASCII_BG_BLACK, color_bg_black);
make_color_macro!(ASCII_BG_RED, color_bg_red);
make_color_macro!(ASCII_BG_GREEN, color_bg_green);
make_color_macro!(ASCII_BG_YELLOW, color_bg_yellow);
make_color_macro!(ASCII_BG_BLUE, color_bg_blue);
make_color_macro!(ASCII_BG_MAGENTA, color_bg_magenta);
make_color_macro!(ASCII_BG_CYAN, color_bg_cyan);
make_color_macro!(ASCII_BG_WHITE, color_bg_white);

make_color_macro!(ASCII_BG_BRIGHT_BLACK, color_bg_bright_black);
make_color_macro!(ASCII_BG_BRIGHT_RED, color_bg_bright_red);
make_color_macro!(ASCII_BG_BRIGHT_GREEN, color_bg_bright_green);
make_color_macro!(ASCII_BG_BRIGHT_YELLOW, color_bg_bright_yellow);
make_color_macro!(ASCII_BG_BRIGHT_BLUE, color_bg_bright_blue);
make_color_macro!(ASCII_BG_BRIGHT_MAGENTA, color_bg_bright_magenta);
make_color_macro!(ASCII_BG_BRIGHT_CYAN, color_bg_bright_cyan);
make_color_macro!(ASCII_BG_BRIGHT_WHITE, color_bg_bright_white);

make_color_macro!(ASCII_CLEAR, color_clear);

/// Color palette.
#[derive(Clone)]
pub struct ColorPalette {
  black: String,
  red: String,
  green: String,
  yellow: String,
  blue: String,
  magenta: String,
  cyan: String,
  white: String,
  reset: String,
}

impl From<ColorMode> for ColorPalette {
  /// Converts a [ColorPalette] from [ColorMode].
  fn from(value: ColorMode) -> Self {
    Self {
      black: match value {
        ColorMode::On => ASCII_BLACK.to_string(),
        _ => "".to_string(),
      },
      red: match value {
        ColorMode::On => ASCII_RED.to_string(),
        _ => "".to_string(),
      },
      green: match value {
        ColorMode::On => ASCII_GREEN.to_string(),
        _ => "".to_string(),
      },
      yellow: match value {
        ColorMode::On => ASCII_YELLOW.to_string(),
        _ => "".to_string(),
      },
      blue: match value {
        ColorMode::On => ASCII_BLUE.to_string(),
        _ => "".to_string(),
      },
      magenta: match value {
        ColorMode::On => ASCII_MAGENTA.to_string(),
        _ => "".to_string(),
      },
      cyan: match value {
        ColorMode::On => ASCII_CYAN.to_string(),
        _ => "".to_string(),
      },
      white: match value {
        ColorMode::On => ASCII_WHITE.to_string(),
        _ => "".to_string(),
      },
      reset: match value {
        ColorMode::On => ASCII_CLEAR.to_string(),
        _ => "".to_string(),
      },
    }
  }
}

impl ColorPalette {
  pub fn black(&self) -> &str {
    &self.black
  }

  pub fn red(&self) -> &str {
    &self.red
  }

  pub fn green(&self) -> &str {
    &self.green
  }

  pub fn yellow(&self) -> &str {
    &self.yellow
  }

  pub fn blue(&self) -> &str {
    &self.blue
  }

  pub fn magenta(&self) -> &str {
    &self.magenta
  }

  pub fn cyan(&self) -> &str {
    &self.cyan
  }

  pub fn white(&self) -> &str {
    &self.white
  }

  pub fn clear(&self) -> &str {
    &self.reset
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate as dsntk_common;

  #[test]
  fn test_8_colors() {
    let color_mode = ColorMode::On;
    let color_clear = color_clear!(color_mode);
    assert_eq!("\u{1b}[30m0\u{1b}[0m", format!("{1}0{0}", color_clear, color_black!(color_mode)));
    assert_eq!("\u{1b}[31m1\u{1b}[0m", format!("{1}1{0}", color_clear, color_red!(color_mode)));
    assert_eq!("\u{1b}[32m2\u{1b}[0m", format!("{1}2{0}", color_clear, color_green!(color_mode)));
    assert_eq!("\u{1b}[33m3\u{1b}[0m", format!("{1}3{0}", color_clear, color_yellow!(color_mode)));
    assert_eq!("\u{1b}[34m4\u{1b}[0m", format!("{1}4{0}", color_clear, color_blue!(color_mode)));
    assert_eq!("\u{1b}[35m5\u{1b}[0m", format!("{1}5{0}", color_clear, color_magenta!(color_mode)));
    assert_eq!("\u{1b}[36m6\u{1b}[0m", format!("{1}6{0}", color_clear, color_cyan!(color_mode)));
    assert_eq!("\u{1b}[37m7\u{1b}[0m", format!("{1}7{0}", color_clear, color_white!(color_mode)));
  }

  #[test]
  fn test_8_color_palette() {
    let colors: ColorPalette = (ColorMode::On).into();
    let color_reset = &mut colors.clear();
    assert_eq!("\u{1b}[30m0\u{1b}[0m", format!("{1}0{0}", color_reset, colors.black()));
    assert_eq!("\u{1b}[31m1\u{1b}[0m", format!("{1}1{0}", color_reset, colors.red()));
    assert_eq!("\u{1b}[32m2\u{1b}[0m", format!("{1}2{0}", color_reset, colors.green()));
    assert_eq!("\u{1b}[33m3\u{1b}[0m", format!("{1}3{0}", color_reset, colors.yellow()));
    assert_eq!("\u{1b}[34m4\u{1b}[0m", format!("{1}4{0}", color_reset, colors.blue()));
    assert_eq!("\u{1b}[35m5\u{1b}[0m", format!("{1}5{0}", color_reset, colors.magenta()));
    assert_eq!("\u{1b}[36m6\u{1b}[0m", format!("{1}6{0}", color_reset, colors.cyan()));
    assert_eq!("\u{1b}[37m7\u{1b}[0m", format!("{1}7{0}", color_reset, colors.white()));
  }

  #[test]
  fn test_8_bright_colors() {
    let color_mode = ColorMode::On;
    let color_clear = color_clear!(color_mode);
    assert_eq!("\u{1b}[30;1m0\u{1b}[0m", format!("{1}0{0}", color_clear, color_bright_black!(color_mode)));
    assert_eq!("\u{1b}[31;1m1\u{1b}[0m", format!("{1}1{0}", color_clear, color_bright_red!(color_mode)));
    assert_eq!("\u{1b}[32;1m2\u{1b}[0m", format!("{1}2{0}", color_clear, color_bright_green!(color_mode)));
    assert_eq!("\u{1b}[33;1m3\u{1b}[0m", format!("{1}3{0}", color_clear, color_bright_yellow!(color_mode)));
    assert_eq!("\u{1b}[34;1m4\u{1b}[0m", format!("{1}4{0}", color_clear, color_bright_blue!(color_mode)));
    assert_eq!("\u{1b}[35;1m5\u{1b}[0m", format!("{1}5{0}", color_clear, color_bright_magenta!(color_mode)));
    assert_eq!("\u{1b}[36;1m6\u{1b}[0m", format!("{1}6{0}", color_clear, color_bright_cyan!(color_mode)));
    assert_eq!("\u{1b}[37;1m7\u{1b}[0m", format!("{1}7{0}", color_clear, color_bright_white!(color_mode)));
  }

  #[test]
  fn test_8_bg_colors() {
    let color_mode = ColorMode::On;
    let color_clear = color_clear!(color_mode);
    assert_eq!("\u{1b}[40m0\u{1b}[0m", format!("{1}0{0}", color_clear, color_bg_black!(color_mode)));
    assert_eq!("\u{1b}[41m1\u{1b}[0m", format!("{1}1{0}", color_clear, color_bg_red!(color_mode)));
    assert_eq!("\u{1b}[42m2\u{1b}[0m", format!("{1}2{0}", color_clear, color_bg_green!(color_mode)));
    assert_eq!("\u{1b}[43m3\u{1b}[0m", format!("{1}3{0}", color_clear, color_bg_yellow!(color_mode)));
    assert_eq!("\u{1b}[44m4\u{1b}[0m", format!("{1}4{0}", color_clear, color_bg_blue!(color_mode)));
    assert_eq!("\u{1b}[45m5\u{1b}[0m", format!("{1}5{0}", color_clear, color_bg_magenta!(color_mode)));
    assert_eq!("\u{1b}[46m6\u{1b}[0m", format!("{1}6{0}", color_clear, color_bg_cyan!(color_mode)));
    assert_eq!("\u{1b}[47m7\u{1b}[0m", format!("{1}7{0}", color_clear, color_bg_white!(color_mode)));
  }

  #[test]
  fn test_8_bg_bright_colors() {
    let color_mode = ColorMode::On;
    let color_clear = color_clear!(color_mode);
    assert_eq!("\u{1b}[40;1m0\u{1b}[0m", format!("{1}0{0}", color_clear, color_bg_bright_black!(color_mode)));
    assert_eq!("\u{1b}[41;1m1\u{1b}[0m", format!("{1}1{0}", color_clear, color_bg_bright_red!(color_mode)));
    assert_eq!("\u{1b}[42;1m2\u{1b}[0m", format!("{1}2{0}", color_clear, color_bg_bright_green!(color_mode)));
    assert_eq!("\u{1b}[43;1m3\u{1b}[0m", format!("{1}3{0}", color_clear, color_bg_bright_yellow!(color_mode)));
    assert_eq!("\u{1b}[44;1m4\u{1b}[0m", format!("{1}4{0}", color_clear, color_bg_bright_blue!(color_mode)));
    assert_eq!("\u{1b}[45;1m5\u{1b}[0m", format!("{1}5{0}", color_clear, color_bg_bright_magenta!(color_mode)));
    assert_eq!("\u{1b}[46;1m6\u{1b}[0m", format!("{1}6{0}", color_clear, color_bg_bright_cyan!(color_mode)));
    assert_eq!("\u{1b}[47;1m7\u{1b}[0m", format!("{1}7{0}", color_clear, color_bg_bright_white!(color_mode)));
  }

  fn test_display_8_colors() {
    print!("{ASCII_BLACK}0{ASCII_CLEAR} ");
    print!("{ASCII_RED}1{ASCII_CLEAR} ");
    print!("{ASCII_GREEN}2{ASCII_CLEAR} ");
    print!("{ASCII_YELLOW}3{ASCII_CLEAR} ");
    print!("{ASCII_BLUE}4{ASCII_CLEAR} ");
    print!("{ASCII_MAGENTA}5{ASCII_CLEAR} ");
    print!("{ASCII_CYAN}6{ASCII_CLEAR} ");
    print!("{ASCII_WHITE}7{ASCII_CLEAR} ");
    print!("\n\n");
  }

  fn test_display_8_bright_colors() {
    print!("{ASCII_BRIGHT_BLACK}0{ASCII_CLEAR} ");
    print!("{ASCII_BRIGHT_RED}1{ASCII_CLEAR} ");
    print!("{ASCII_BRIGHT_GREEN}2{ASCII_CLEAR} ");
    print!("{ASCII_BRIGHT_YELLOW}3{ASCII_CLEAR} ");
    print!("{ASCII_BRIGHT_BLUE}4{ASCII_CLEAR} ");
    print!("{ASCII_BRIGHT_MAGENTA}5{ASCII_CLEAR} ");
    print!("{ASCII_BRIGHT_CYAN}6{ASCII_CLEAR} ");
    print!("{ASCII_BRIGHT_WHITE}7{ASCII_CLEAR} ");
    print!("\n\n");
  }

  fn test_display_8_bg_colors() {
    print!("{ASCII_BG_BLACK} 0 {ASCII_CLEAR} ");
    print!("{ASCII_BG_RED} 1 {ASCII_CLEAR} ");
    print!("{ASCII_BG_GREEN} 2 {ASCII_CLEAR} ");
    print!("{ASCII_BG_YELLOW} 3 {ASCII_CLEAR} ");
    print!("{ASCII_BG_BLUE} 4 {ASCII_CLEAR} ");
    print!("{ASCII_BG_MAGENTA} 5 {ASCII_CLEAR} ");
    print!("{ASCII_BG_CYAN} 6 {ASCII_CLEAR} ");
    print!("{ASCII_BG_WHITE} 7 {ASCII_CLEAR} ");
    print!("\n\n");
  }

  fn test_display_8_bg_bright_colors() {
    print!("{ASCII_BG_BRIGHT_BLACK} 0 {ASCII_CLEAR} ");
    print!("{ASCII_BG_BRIGHT_RED} 1 {ASCII_CLEAR} ");
    print!("{ASCII_BG_BRIGHT_GREEN} 2 {ASCII_CLEAR} ");
    print!("{ASCII_BG_BRIGHT_YELLOW} 3 {ASCII_CLEAR} ");
    print!("{ASCII_BG_BRIGHT_BLUE} 4 {ASCII_CLEAR} ");
    print!("{ASCII_BG_BRIGHT_MAGENTA} 5 {ASCII_CLEAR} ");
    print!("{ASCII_BG_BRIGHT_CYAN} 6 {ASCII_CLEAR} ");
    print!("{ASCII_BG_BRIGHT_WHITE} 7 {ASCII_CLEAR} ");
    print!("\n\n");
  }

  fn test_display_256_colors() {
    for i in 0..16 {
      for j in 0..16 {
        let code = format!("{}", i * 16 + j);
        print!("\u{1b}[38;5;{code}m{code:>4}{ASCII_CLEAR}")
      }
      println!();
    }
    println!();
  }

  #[test]
  fn test_display_all() {
    test_display_8_colors();
    test_display_8_bright_colors();
    test_display_8_bg_colors();
    test_display_8_bg_bright_colors();
    test_display_256_colors();
  }
}
