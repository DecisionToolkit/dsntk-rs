//! # Parsing tables
//!
//! Parsing tables extracted from files generated by `Bison` for `C` language.
//! This file was generated by dedicated tool, do not modify it manually.

use dsntk_common::Result;

/// Types of tokens returned by lexer.
#[derive(Clone)]
pub enum TokenType {
  YyEmpty = -2,
  YyEof = 0,
  YyError = 256,
  YyUndef = 257,
  StartExpression = 258,
  StartBoxedExpression = 259,
  StartContext = 260,
  StartTextualExpression = 261,
  StartTextualExpressions = 262,
  StartUnaryTests = 263,
  At = 264,
  Not = 265,
  Colon = 266,
  Comma = 267,
  Every = 268,
  For = 269,
  LeftBrace = 270,
  Null = 271,
  RightArrow = 272,
  Of = 273,
  List = 274,
  Range = 275,
  Context = 276,
  Then = 277,
  Function = 278,
  External = 279,
  If = 280,
  RightBrace = 281,
  RightBracket = 282,
  RightParen = 283,
  Return = 284,
  Ellipsis = 285,
  Some = 286,
  Numeric = 287,
  String = 288,
  Boolean = 289,
  Satisfies = 290,
  Else = 291,
  Or = 292,
  And = 293,
  Eq = 294,
  Nq = 295,
  Lt = 296,
  Le = 297,
  Gt = 298,
  Ge = 299,
  Between = 300,
  BetweenAnd = 301,
  In = 302,
  Minus = 303,
  Plus = 304,
  Mul = 305,
  Div = 306,
  Exp = 307,
  Instance = 309,
  Name = 310,
  NameDateTime = 311,
  BuiltInTypeName = 312,
  LeftParen = 313,
  LeftBracket = 314,
  Dot = 315,
}

/// Kinds of symbols recognized by parser.
#[allow(clippy::enum_variant_names)]
pub enum SymbolKind {
  YyEmpty = -2,
  YyEof = 0,
  YyError = 1,
  YyUndef = 2,
  YyAccept = 61,
}

///
pub const YY_PACT_N_INF: i16 = -222;

///
pub const YY_TABLE_N_INF: i16 = -108;

///
pub const YY_FINAL: usize = 48;

///
pub const YY_LAST: i16 = 867;

///
pub const YY_N_TOKENS: usize = 61;

/// `YY_TRANSLATE[TOKEN-NUM]` - symbol number corresponding to TOKEN-NUM as returned by lexer.
pub const YY_TRANSLATE: [i8; 316] = [
  0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
];

/// `YY_PACT[STATE-NUM]` - index in YY_TABLE of the portion describing STATE-NUM.
pub const YY_PACT: [i16; 286] = [
  193, 304, -3, 31, 304, 304, -222, 48, -8, -222, -222, -222, -222, -4, 304, 10, -222, -222, -222, -222, 10, 10, 10, 10, 304, 14, 41, 341, 378, 740, -222, -222, -222, -222, 10,
  -222, -222, -222, -222, -222, 415, -222, -222, 740, 102, 45, -222, 179, -222, -222, -222, -222, 68, -222, 591, 46, 78, -222, -222, -222, -222, -222, -222, -222, -222, 109, 55,
  -222, -2, 642, 81, 84, 10, 475, 89, -222, 304, 304, 304, 304, 304, 304, 304, 304, -222, 452, 304, 304, 304, 304, 304, 94, 230, 304, 63, -222, 24, 304, 64, 304, 567, -222, -222,
  92, 112, 73, 106, 131, 93, -222, -222, -222, -222, 91, 133, -6, 304, 96, -222, 115, 97, -222, 230, 101, -222, -222, 304, -222, -222, -222, 763, 785, 807, 807, 807, 807, 807,
  807, 304, 341, 82, 95, 95, 120, 120, 109, -222, -222, 7, 509, -222, -222, 85, -222, 617, -222, -222, -222, -222, -222, 304, 304, 304, -222, -222, 304, -222, -222, 17, -222,
  -222, 304, -222, 147, 267, -222, 692, -222, 304, 55, -222, 30, 475, 716, 543, -222, 50, 304, 304, -222, -222, 107, -222, -222, -222, 137, -222, 740, -222, 124, 740, -222, 128,
  91, 740, -222, 304, 740, -222, 88, 304, 740, -222, 101, -222, 304, 304, 135, 141, 143, 144, -222, -222, -222, 740, 509, 180, 85, -222, 304, 304, -222, 50, 740, 148, -222, -222,
  740, 82, 187, -222, -222, 152, 165, -222, -222, 740, 667, -222, -222, -222, -222, 50, 50, 205, -222, 25, -222, 200, 50, 304, 88, 175, 176, -222, 152, -222, -222, -222, 52, 740,
  -222, -222, -222, 50, 25, 50, -222, -222, -222, -222, -222, -222, 50, 52, -222,
];

/// `YY_DEF_ACT[STATE-NUM]` - default reduction number in state STATE-NUM.
/// Performed when YY_TABLE does not specify something else to do.
/// Zero means the default is an error.
pub const YY_DEF_ACT: [u8; 286] = [
  0, 0, 0, 0, 0, 0, 7, 0, 0, 19, 14, 82, 75, 0, 0, 0, 17, 76, 77, 78, 0, 0, 0, 0, 0, 47, 0, 0, 0, 2, 9, 10, 46, 63, 0, 45, 74, 13, 11, 12, 0, 3, 4, 0, 10, 10, 6, 0, 1, 79, 137,
  130, 0, 140, 0, 107, 0, 71, 73, 72, 137, 59, 60, 61, 62, 38, 0, 80, 47, 0, 0, 74, 92, 0, 0, 91, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 53, 57, 8,
  55, 0, 136, 0, 0, 129, 0, 84, 90, 89, 83, 0, 0, 0, 0, 0, 66, 0, 50, 41, 0, 0, 48, 65, 0, 94, 93, 67, 23, 24, 25, 26, 27, 28, 29, 30, 0, 0, 32, 34, 33, 35, 36, 37, 39, 96, 47, 0,
  44, 97, 0, 98, 0, 42, 70, 68, 69, 51, 0, 0, 0, 137, 138, 0, 130, 131, 0, 87, 85, 0, 142, 150, 0, 143, 0, 106, 0, 0, 81, 50, 0, 0, 0, 31, 0, 0, 0, 104, 103, 0, 101, 99, 43, 0,
  56, 20, 135, 0, 15, 128, 0, 0, 86, 148, 0, 152, 141, 0, 0, 18, 49, 0, 95, 0, 0, 0, 0, 0, 0, 108, 109, 40, 100, 0, 0, 0, 54, 0, 0, 88, 0, 151, 0, 145, 144, 16, 22, 0, 110, 112,
  0, 123, 105, 102, 139, 133, 132, 149, 146, 58, 0, 0, 0, 114, 0, 122, 0, 0, 0, 0, 0, 0, 118, 0, 120, 117, 115, 0, 134, 147, 111, 113, 0, 0, 0, 126, 125, 124, 119, 121, 116, 0, 0,
  127,
];

/// `YY_P_GOTO[NTERM-NUM]`
pub const YY_P_GOTO: [i16; 71] = [
  -222, -222, -222, -1, 222, 12, -222, -222, -222, -222, -222, -150, 129, -222, -155, -222, -222, -222, -222, -222, 13, -222, -222, -13, -222, 227, -222, -222, 57, 28, -222, -222,
  -222, 51, 110, -222, 49, 9, -222, 16, -93, -221, -222, -222, -222, -222, -23, -222, -28, -222, -222, -36, -222, 83, -222, -222, -222, -222, -59, -222, -222, -222, -222, -222,
  -222, -222, -11, -222, 18, -222, -222,
];

/// `YY_DEF_GOTO[NTERM-NUM]`
pub const YY_DEF_GOTO: [i16; 71] = [
  0, 7, 47, 100, 30, 31, 51, 60, 50, 138, 186, 121, 46, 101, 102, 185, 32, 33, 34, 95, 56, 57, 35, 36, 122, 37, 52, 112, 113, 170, 114, 38, 75, 128, 150, 151, 152, 193, 153, 190,
  59, 223, 252, 253, 276, 255, 256, 274, 267, 258, 259, 279, 283, 106, 107, 108, 202, 248, 103, 104, 105, 199, 39, 115, 174, 209, 236, 261, 175, 232, 208,
];

/// `YY_TABLE[YY_PACT[STATE-NUM]]` - what to do in state STATE-NUM.
/// If positive, shift that token.
/// If negative, reduce the rule whose number is the opposite.
/// If `YY_TABLE_N_INF`, syntax error.
pub const YY_TABLE: [i16; 868] = [
  29, 119, 58, 43, 43, 195, 196, 58, 58, 58, 58, 249, 11, 54, 71, 71, 44, 45, 187, 8, 13, 58, 172, 65, 177, 49, 69, 73, -107, 212, 177, 262, 263, 61, 62, 63, 64, 265, 269, 73, 70,
  74, 17, 18, 19, -52, 11, 96, 48, 173, 110, 156, 157, 280, 53, 282, 40, 97, 123, 58, -107, 239, 284, 212, 277, 55, 26, 66, 266, 217, 218, 219, 111, 220, 66, 130, 131, 132, 133,
  134, 135, 136, 137, 158, 140, 141, 142, 143, 144, 145, 213, 149, 154, 222, 109, 278, 43, 191, 65, 67, 234, 110, -5, 168, 198, 55, 117, 221, 118, 45, 120, 125, 146, 192, -73,
  176, 235, 169, 155, 129, 177, 149, 160, 111, 163, 182, 71, 162, 164, 85, 86, 87, 88, 89, 90, 165, 91, 183, 184, 222, 92, 93, 94, 166, 171, 88, 89, 90, 167, 91, 178, 55, 70, 92,
  93, 94, 181, 179, 205, 222, 222, 197, 226, 91, 200, 228, 222, 92, 93, 94, 204, 229, 90, 207, 91, 230, 240, 211, 92, 93, 94, 222, 241, 222, 242, 243, 224, 225, 8, 98, 222, 187,
  9, 10, 11, 12, 1, 2, 3, 4, 5, 6, 13, 173, 14, 233, 15, 254, 257, 237, 16, 17, 18, 19, 238, 251, 264, 268, 272, 273, 20, 21, 22, 23, 41, 203, 159, 99, 246, 247, 42, 231, 180,
  214, 25, 26, 245, 27, 28, 8, 227, 244, 275, 9, 10, 11, 12, 281, 285, 201, 271, 0, 250, 13, 0, 14, 0, 15, 147, 270, 0, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0,
  24, 0, 9, 10, 11, 12, 0, 148, 26, 0, 27, 28, 13, 206, 14, 0, 15, 0, 0, 0, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0, 24, 0, 9, 10, 11, 12, 0, 25, 26, 0, 27, 28,
  13, 0, 14, 0, 15, 0, 0, 0, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0, 24, 0, 9, 10, 11, 12, 0, 25, 26, 0, 27, 28, 13, 0, 14, 0, 15, 0, 0, 0, 16, 17, 18, 19, 0,
  0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0, 24, 0, 9, 10, 11, 12, 0, 68, 26, 0, 27, 28, 13, 0, 14, 0, 72, 0, 0, 0, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0, 24, 0,
  9, 10, 11, 12, 0, 68, 26, 0, 27, 28, 13, 0, 14, 0, 72, 0, 0, 0, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0, 24, 0, 9, 10, 11, 12, 0, 25, 26, 0, 27, 28, 13, 0, 14,
  0, 15, 0, 0, 0, 16, 17, 18, 19, 126, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 0, 0, 24, 0, 127, 0, 0, 0, 0, 25, 26, 0, 139, 28, 76, 77, 78, 79, 80, 81, 82, 83, 84, 188, 85, 86, 87, 88,
  89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 0, 189, 0, 0, 0, 0, 0, 0, 0, 0, 76, 77, 78, 79, 80, 81, 82, 83, 84, 216, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 0, 124, 0, 0, 0,
  0, 0, 0, 0, 161, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 76, 77, 78, 79, 80, 81, 82, 83, 84, 116, 85, 86, 87, 88, 89, 90, 0,
  91, 0, 0, 0, 92, 93, 94, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 194, 91, 0, 0, 0, 92, 93, 94, 0, 0, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87,
  88, 89, 90, 124, 91, 0, 0, 0, 92, 93, 94, 0, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 260, 0, 0, 92, 93, 94, 0, 76, 77, 78, 79, 80, 81, 82, 83, 84,
  0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 210, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 76, 77, 78, 79, 80, 81,
  82, 83, 84, 215, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 77, 78, 79, 80,
  81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, -108, -108, -108, -108,
  -108, -108, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94,
];

/// ???
pub const YY_CHECK: [i16; 868] = [
  1, 60, 15, 4, 5, 160, 161, 20, 21, 22, 23, 232, 15, 14, 27, 28, 4, 5, 11, 9, 23, 34, 28, 24, 117, 33, 27, 28, 30, 179, 123, 252, 253, 20, 21, 22, 23, 12, 259, 40, 27, 28, 32,
  33, 34, 0, 15, 34, 0, 55, 33, 27, 28, 274, 58, 276, 59, 12, 60, 72, 30, 216, 283, 213, 12, 55, 56, 60, 43, 19, 20, 21, 55, 23, 60, 76, 77, 78, 79, 80, 81, 82, 83, 59, 85, 86,
  87, 88, 89, 90, 60, 92, 93, 186, 26, 43, 97, 12, 99, 58, 12, 33, 0, 12, 163, 55, 60, 57, 30, 97, 55, 30, 18, 28, 30, 116, 28, 26, 55, 30, 213, 122, 58, 55, 12, 126, 139, 35, 55,
  47, 48, 49, 50, 51, 52, 29, 54, 138, 139, 232, 58, 59, 60, 12, 11, 50, 51, 52, 55, 54, 35, 55, 139, 58, 59, 60, 55, 60, 11, 252, 253, 162, 55, 54, 165, 28, 259, 58, 59, 60, 171,
  47, 52, 174, 54, 47, 41, 178, 58, 59, 60, 274, 41, 276, 41, 41, 187, 188, 9, 10, 283, 11, 13, 14, 15, 16, 3, 4, 5, 6, 7, 8, 23, 55, 25, 206, 27, 55, 43, 210, 31, 32, 33, 34,
  215, 28, 11, 17, 43, 43, 41, 42, 43, 44, 2, 168, 97, 48, 229, 230, 3, 203, 122, 182, 55, 56, 227, 58, 59, 9, 191, 225, 265, 13, 14, 15, 16, 275, 284, 166, 261, -1, 234, 23, -1,
  25, -1, 27, 28, 260, -1, 31, 32, 33, 34, -1, -1, -1, -1, -1, -1, 41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23, 24, 25, -1, 27, -1, -1, -1, 31,
  32, 33, 34, -1, -1, -1, -1, -1, -1, 41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23, -1, 25, -1, 27, -1, -1, -1, 31, 32, 33, 34, -1, -1, -1, -1,
  -1, -1, 41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23, -1, 25, -1, 27, -1, -1, -1, 31, 32, 33, 34, -1, -1, -1, -1, -1, -1, 41, 42, 43, 44, -1, 9,
  -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23, -1, 25, -1, 27, -1, -1, -1, 31, 32, 33, 34, -1, -1, -1, -1, -1, -1, 41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16,
  -1, 55, 56, -1, 58, 59, 23, -1, 25, -1, 27, -1, -1, -1, 31, 32, 33, 34, -1, -1, -1, -1, -1, -1, 41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23,
  -1, 25, -1, 27, -1, -1, -1, 31, 32, 33, 34, 12, -1, -1, -1, -1, -1, 41, 42, 43, 44, -1, -1, -1, 48, -1, 27, -1, -1, -1, -1, 55, 56, -1, 58, 59, 37, 38, 39, 40, 41, 42, 43, 44,
  45, 12, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, -1, 28, -1, -1, -1, -1, -1, -1, -1, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, 12, 47, 48, 49, 50, 51, 52, -1, 54,
  -1, -1, -1, 58, 59, 60, -1, 28, -1, -1, -1, -1, -1, -1, -1, 12, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 37, 38, 39, 40,
  41, 42, 43, 44, 45, 22, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, 27, 54, -1, -1, -1, 58, 59, 60,
  -1, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, 28, 54, -1, -1, -1, 58, 59, 60, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1,
  54, 30, -1, -1, 58, 59, 60, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47,
  48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 37, 38, 39, 40, 41, 42, 43,
  44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 39, 40, 41, 42,
  43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60,
];

/// `YY_R1[YYN]` - symbol number of symbol that rule YYN derives.
pub const YY_R1: [u8; 153] = [
  0, 61, 62, 62, 62, 62, 62, 63, 62, 64, 64, 65, 65, 65, 67, 66, 66, 68, 66, 69, 66, 70, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 71, 66, 66, 66, 66,
  66, 66, 66, 66, 66, 72, 72, 73, 73, 74, 74, 74, 75, 75, 76, 77, 77, 77, 77, 77, 78, 79, 79, 79, 80, 80, 80, 81, 82, 82, 83, 83, 84, 84, 84, 84, 85, 84, 87, 86, 88, 88, 89, 90,
  90, 91, 91, 92, 93, 93, 94, 94, 95, 95, 95, 96, 97, 98, 98, 99, 100, 100, 101, 101, 102, 102, 103, 102, 104, 102, 102, 105, 102, 106, 108, 107, 109, 109, 110, 111, 110, 112,
  113, 112, 114, 114, 116, 117, 115, 118, 118, 119, 119, 121, 122, 120, 124, 123, 125, 126, 125, 127, 128, 127, 130, 129, 129, 131, 131,
];

/// `YY_R2[YYN]` - number of symbols on the right hand side of rule YYN.
pub const YY_R2: [i8; 153] = [
  0, 2, 2, 2, 2, 2, 2, 0, 3, 1, 1, 1, 1, 1, 0, 5, 6, 0, 5, 0, 5, 0, 6, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 2, 0, 5, 3, 3, 4, 3, 1, 1, 1, 3, 3, 1, 3, 1, 1, 4, 1, 3, 1, 4,
  2, 2, 2, 2, 1, 2, 3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 0, 4, 0, 3, 1, 2, 3, 1, 3, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 3, 1, 3, 2, 1, 3, 3, 1, 1, 1, 0, 5, 0, 5, 3, 0, 6, 2,
  0, 4, 1, 3, 1, 0, 3, 1, 0, 4, 3, 1, 0, 0, 5, 1, 3, 3, 1, 0, 0, 5, 0, 5, 1, 0, 3, 1, 0, 4, 0, 4, 1, 2, 1,
];

///Trait for reduce action definitions.
pub trait ReduceActions {
  fn action_addition(&mut self) -> Result<()>;
  fn action_between(&mut self) -> Result<()>;
  fn action_between_begin(&mut self) -> Result<()>;
  fn action_built_in_type_name(&mut self) -> Result<()>;
  fn action_comparison_eq(&mut self) -> Result<()>;
  fn action_comparison_ge(&mut self) -> Result<()>;
  fn action_comparison_gt(&mut self) -> Result<()>;
  fn action_comparison_in(&mut self) -> Result<()>;
  fn action_comparison_le(&mut self) -> Result<()>;
  fn action_comparison_lt(&mut self) -> Result<()>;
  fn action_comparison_nq(&mut self) -> Result<()>;
  fn action_comparison_unary_ge(&mut self) -> Result<()>;
  fn action_comparison_unary_gt(&mut self) -> Result<()>;
  fn action_comparison_unary_le(&mut self) -> Result<()>;
  fn action_comparison_unary_lt(&mut self) -> Result<()>;
  fn action_conjunction(&mut self) -> Result<()>;
  fn action_context_begin(&mut self) -> Result<()>;
  fn action_context_end(&mut self) -> Result<()>;
  fn action_context_entry(&mut self) -> Result<()>;
  fn action_context_entry_tail(&mut self) -> Result<()>;
  fn action_context_type_entry(&mut self) -> Result<()>;
  fn action_context_type_entry_tail(&mut self) -> Result<()>;
  fn action_disjunction(&mut self) -> Result<()>;
  fn action_division(&mut self) -> Result<()>;
  fn action_empty_context(&mut self) -> Result<()>;
  fn action_every(&mut self) -> Result<()>;
  fn action_every_begin(&mut self) -> Result<()>;
  fn action_exponentiation(&mut self) -> Result<()>;
  fn action_expression_list_tail(&mut self) -> Result<()>;
  fn action_filter(&mut self) -> Result<()>;
  fn action_for(&mut self) -> Result<()>;
  fn action_for_begin(&mut self) -> Result<()>;
  fn action_formal_parameter_with_type(&mut self) -> Result<()>;
  fn action_formal_parameter_without_type(&mut self) -> Result<()>;
  fn action_formal_parameters_begin(&mut self) -> Result<()>;
  fn action_formal_parameters_empty(&mut self) -> Result<()>;
  fn action_formal_parameters_first(&mut self) -> Result<()>;
  fn action_formal_parameters_tail(&mut self) -> Result<()>;
  fn action_function_body(&mut self) -> Result<()>;
  fn action_function_body_external(&mut self) -> Result<()>;
  fn action_function_definition(&mut self) -> Result<()>;
  fn action_function_invocation(&mut self) -> Result<()>;
  fn action_function_invocation_no_parameters(&mut self) -> Result<()>;
  fn action_function_type(&mut self) -> Result<()>;
  fn action_function_type_parameters_empty(&mut self) -> Result<()>;
  fn action_function_type_parameters_tail(&mut self) -> Result<()>;
  fn action_if(&mut self) -> Result<()>;
  fn action_instance_of(&mut self) -> Result<()>;
  fn action_interval(&mut self) -> Result<()>;
  fn action_interval_end(&mut self) -> Result<()>;
  fn action_interval_start(&mut self) -> Result<()>;
  fn action_iteration_context_value_range(&mut self) -> Result<()>;
  fn action_iteration_context_value_single(&mut self) -> Result<()>;
  fn action_iteration_context_variable_name(&mut self) -> Result<()>;
  fn action_iteration_context_variable_name_begin(&mut self) -> Result<()>;
  fn action_iteration_contexts_tail(&mut self) -> Result<()>;
  fn action_key_name(&mut self) -> Result<()>;
  fn action_key_string(&mut self) -> Result<()>;
  fn action_list(&mut self) -> Result<()>;
  fn action_list_empty(&mut self) -> Result<()>;
  fn action_list_tail(&mut self) -> Result<()>;
  fn action_list_type(&mut self) -> Result<()>;
  fn action_literal_at(&mut self) -> Result<()>;
  fn action_literal_boolean(&mut self) -> Result<()>;
  fn action_literal_date_time(&mut self) -> Result<()>;
  fn action_literal_null(&mut self) -> Result<()>;
  fn action_literal_numeric(&mut self) -> Result<()>;
  fn action_literal_string(&mut self) -> Result<()>;
  fn action_multiplication(&mut self) -> Result<()>;
  fn action_name(&mut self) -> Result<()>;
  fn action_named_parameter(&mut self) -> Result<()>;
  fn action_named_parameters_tail(&mut self) -> Result<()>;
  fn action_negation(&mut self) -> Result<()>;
  fn action_path(&mut self) -> Result<()>;
  fn action_path_segment(&mut self) -> Result<()>;
  fn action_path_segment_tail(&mut self) -> Result<()>;
  fn action_positional_parameters_tail(&mut self) -> Result<()>;
  fn action_qualified_name(&mut self) -> Result<()>;
  fn action_qualified_name_tail(&mut self) -> Result<()>;
  fn action_quantified_expression(&mut self) -> Result<()>;
  fn action_quantified_expression_variable_name(&mut self) -> Result<()>;
  fn action_quantified_expression_variable_name_begin(&mut self) -> Result<()>;
  fn action_quantified_expressions_tail(&mut self) -> Result<()>;
  fn action_range_type(&mut self) -> Result<()>;
  fn action_some(&mut self) -> Result<()>;
  fn action_some_begin(&mut self) -> Result<()>;
  fn action_subtraction(&mut self) -> Result<()>;
  fn action_type_name(&mut self) -> Result<()>;
  fn action_unary_tests_begin(&mut self) -> Result<()>;
  fn action_unary_tests_irrelevant(&mut self) -> Result<()>;
  fn action_unary_tests_negated(&mut self) -> Result<()>;
}

/// Calls requested reduce action.
pub fn reduce(reduce_actions: &mut impl ReduceActions, rule_number: i16) -> Result<()> {
  match rule_number {
    7 => reduce_actions.action_unary_tests_begin(),                           // $@1: %empty
    14 => reduce_actions.action_for_begin(),                                  // $@2: %empty
    15 => reduce_actions.action_for(),                                        // textual_expression: FOR $@2 iteration_contexts RETURN expression
    16 => reduce_actions.action_if(),                                         // textual_expression: IF expression THEN expression ELSE expression
    17 => reduce_actions.action_some_begin(),                                 // $@3: %empty
    18 => reduce_actions.action_some(),                                       // textual_expression: SOME $@3 quantified_expressions SATISFIES expression
    19 => reduce_actions.action_every_begin(),                                // $@4: %empty
    20 => reduce_actions.action_every(),                                      // textual_expression: EVERY $@4 quantified_expressions SATISFIES expression
    21 => reduce_actions.action_between_begin(),                              // $@5: %empty
    22 => reduce_actions.action_between(),                                    // textual_expression: expression BETWEEN $@5 expression BETWEEN_AND expression
    23 => reduce_actions.action_disjunction(),                                // textual_expression: expression OR expression
    24 => reduce_actions.action_conjunction(),                                // textual_expression: expression AND expression
    25 => reduce_actions.action_comparison_eq(),                              // textual_expression: expression EQ expression
    26 => reduce_actions.action_comparison_nq(),                              // textual_expression: expression NQ expression
    27 => reduce_actions.action_comparison_lt(),                              // textual_expression: expression LT expression
    28 => reduce_actions.action_comparison_le(),                              // textual_expression: expression LE expression
    29 => reduce_actions.action_comparison_gt(),                              // textual_expression: expression GT expression
    30 => reduce_actions.action_comparison_ge(),                              // textual_expression: expression GE expression
    31 => reduce_actions.action_comparison_in(),                              // textual_expression: expression IN LEFT_PAREN comparison_in
    32 => reduce_actions.action_comparison_in(),                              // textual_expression: expression IN expression
    33 => reduce_actions.action_addition(),                                   // textual_expression: expression PLUS expression
    34 => reduce_actions.action_subtraction(),                                // textual_expression: expression MINUS expression
    35 => reduce_actions.action_multiplication(),                             // textual_expression: expression MUL expression
    36 => reduce_actions.action_division(),                                   // textual_expression: expression DIV expression
    37 => reduce_actions.action_exponentiation(),                             // textual_expression: expression EXP expression
    38 => reduce_actions.action_negation(),                                   // textual_expression: MINUS expression
    39 => reduce_actions.action_type_name(),                                  // $@6: %empty
    40 => reduce_actions.action_instance_of(),                                // textual_expression: expression INSTANCE OF $@6 type
    41 => reduce_actions.action_path_segment(),                               // textual_expression: NAME DOT path_segment
    42 => reduce_actions.action_path(),                                       // textual_expression: expression DOT NAME
    43 => reduce_actions.action_filter(),                                     // textual_expression: expression LEFT_BRACKET expression RIGHT_BRACKET
    47 => reduce_actions.action_name(),                                       // textual_expression: NAME
    49 => reduce_actions.action_path_segment(),                               // path_segment: NAME DOT path_segment
    50 => reduce_actions.action_path_segment_tail(),                          // path_segment: NAME
    51 => reduce_actions.action_expression_list_tail(),                       // textual_expressions: textual_expression COMMA textual_expressions
    52 => reduce_actions.action_expression_list_tail(),                       // textual_expressions: textual_expression
    53 => reduce_actions.action_unary_tests_irrelevant(),                     // unary_tests: MINUS
    54 => reduce_actions.action_unary_tests_negated(),                        // unary_tests: NOT LEFT_PAREN positive_unary_tests RIGHT_PAREN
    56 => reduce_actions.action_expression_list_tail(),                       // positive_unary_tests: expression COMMA positive_unary_tests
    57 => reduce_actions.action_expression_list_tail(),                       // positive_unary_tests: expression
    58 => reduce_actions.action_expression_list_tail(),                       // comparison_in: expression COMMA positive_unary_tests RIGHT_PAREN
    59 => reduce_actions.action_comparison_unary_lt(),                        // simple_positive_unary_test: LT endpoint
    60 => reduce_actions.action_comparison_unary_le(),                        // simple_positive_unary_test: LE endpoint
    61 => reduce_actions.action_comparison_unary_gt(),                        // simple_positive_unary_test: GT endpoint
    62 => reduce_actions.action_comparison_unary_ge(),                        // simple_positive_unary_test: GE endpoint
    64 => reduce_actions.action_interval(),                                   // interval: interval_start interval_end
    65 => reduce_actions.action_interval_start(),                             // interval_start: LEFT_PAREN endpoint ELLIPSIS
    66 => reduce_actions.action_interval_start(),                             // interval_start: RIGHT_BRACKET endpoint ELLIPSIS
    67 => reduce_actions.action_interval_start(),                             // interval_start: LEFT_BRACKET endpoint ELLIPSIS
    68 => reduce_actions.action_interval_end(),                               // interval_end: endpoint RIGHT_PAREN
    69 => reduce_actions.action_interval_end(),                               // interval_end: endpoint LEFT_BRACKET
    70 => reduce_actions.action_interval_end(),                               // interval_end: endpoint RIGHT_BRACKET
    75 => reduce_actions.action_literal_null(),                               // literal: NULL
    76 => reduce_actions.action_literal_numeric(),                            // simple_literal: NUMERIC
    77 => reduce_actions.action_literal_string(),                             // simple_literal: STRING
    78 => reduce_actions.action_literal_boolean(),                            // simple_literal: BOOLEAN
    79 => reduce_actions.action_literal_at(),                                 // simple_literal: AT STRING
    80 => reduce_actions.action_literal_date_time(),                          // $@7: %empty
    82 => reduce_actions.action_context_begin(),                              // $@8: %empty
    83 => reduce_actions.action_context_end(),                                // context: LEFT_BRACE $@8 context_entries
    84 => reduce_actions.action_empty_context(),                              // context_entries: RIGHT_BRACE
    85 => reduce_actions.action_context_entry_tail(),                         // context_entries: context_entry context_entry_tail
    86 => reduce_actions.action_context_entry(),                              // context_entry: key COLON expression
    88 => reduce_actions.action_context_entry_tail(),                         // context_entry_tail: COMMA context_entry context_entry_tail
    89 => reduce_actions.action_key_name(),                                   // key: NAME
    90 => reduce_actions.action_key_string(),                                 // key: STRING
    91 => reduce_actions.action_list(),                                       // list: LEFT_BRACKET list_items
    92 => reduce_actions.action_list_empty(),                                 // list_items: RIGHT_BRACKET
    93 => reduce_actions.action_list_tail(),                                  // list_items: expression list_tail
    95 => reduce_actions.action_list_tail(),                                  // list_tail: COMMA expression list_tail
    96 => reduce_actions.action_function_invocation_no_parameters(),          // parameters: RIGHT_PAREN
    97 => reduce_actions.action_function_invocation(),                        // parameters: named_parameters
    98 => reduce_actions.action_function_invocation(),                        // parameters: positional_parameters
    99 => reduce_actions.action_named_parameters_tail(),                      // named_parameters: named_parameter named_parameters_tail
    100 => reduce_actions.action_named_parameter(),                           // named_parameter: NAME COLON expression
    102 => reduce_actions.action_named_parameters_tail(),                     // named_parameters_tail: COMMA named_parameter named_parameters_tail
    103 => reduce_actions.action_positional_parameters_tail(),                // positional_parameters: expression positional_parameters_tail
    105 => reduce_actions.action_positional_parameters_tail(),                // positional_parameters_tail: COMMA expression positional_parameters_tail
    106 => reduce_actions.action_qualified_name_tail(),                       // qualified_name: NAME DOT qualified_name
    107 => reduce_actions.action_qualified_name(),                            // qualified_name: NAME
    108 => reduce_actions.action_built_in_type_name(),                        // type: BUILT_IN_TYPE_NAME
    110 => reduce_actions.action_type_name(),                                 // $@9: %empty
    111 => reduce_actions.action_list_type(),                                 // type: LIST LT $@9 type GT
    112 => reduce_actions.action_type_name(),                                 // $@10: %empty
    113 => reduce_actions.action_range_type(),                                // type: RANGE LT $@10 type GT
    115 => reduce_actions.action_type_name(),                                 // $@11: %empty
    116 => reduce_actions.action_function_type(),                             // type: FUNCTION LT function_type_parameters RIGHT_ARROW $@11 type
    117 => reduce_actions.action_context_type_entry_tail(),                   // context_type_entries: context_type_entry context_type_entry_tail
    118 => reduce_actions.action_type_name(),                                 // $@12: %empty
    119 => reduce_actions.action_context_type_entry(),                        // context_type_entry: NAME COLON $@12 type
    121 => reduce_actions.action_context_type_entry_tail(),                   // context_type_entry_tail: COMMA context_type_entry context_type_entry_tail
    122 => reduce_actions.action_function_type_parameters_empty(),            // function_type_parameters: GT
    123 => reduce_actions.action_type_name(),                                 // $@13: %empty
    124 => reduce_actions.action_function_type_parameters_tail(),             // function_type_parameters: $@13 type function_type_parameters_tail
    126 => reduce_actions.action_type_name(),                                 // $@14: %empty
    127 => reduce_actions.action_function_type_parameters_tail(),             // function_type_parameters_tail: COMMA $@14 type function_type_parameters_tail
    128 => reduce_actions.action_iteration_contexts_tail(),                   // iteration_contexts: iteration_context COMMA iteration_contexts
    129 => reduce_actions.action_iteration_contexts_tail(),                   // iteration_contexts: iteration_context
    130 => reduce_actions.action_iteration_context_variable_name_begin(),     // $@15: %empty
    131 => reduce_actions.action_iteration_context_variable_name(),           // $@16: %empty
    133 => reduce_actions.action_iteration_context_value_single(),            // iteration_context_value: expression
    134 => reduce_actions.action_iteration_context_value_range(),             // iteration_context_value: expression ELLIPSIS expression
    135 => reduce_actions.action_quantified_expressions_tail(),               // quantified_expressions: quantified_expression COMMA quantified_expressions
    136 => reduce_actions.action_quantified_expressions_tail(),               // quantified_expressions: quantified_expression
    137 => reduce_actions.action_quantified_expression_variable_name_begin(), // $@17: %empty
    138 => reduce_actions.action_quantified_expression_variable_name(),       // $@18: %empty
    139 => reduce_actions.action_quantified_expression(),                     // quantified_expression: $@17 NAME $@18 IN expression
    140 => reduce_actions.action_formal_parameters_begin(),                   // $@19: %empty
    141 => reduce_actions.action_function_definition(),                       // function_definition: FUNCTION LEFT_PAREN $@19 formal_parameters external
    142 => reduce_actions.action_formal_parameters_empty(),                   // formal_parameters: RIGHT_PAREN
    143 => reduce_actions.action_formal_parameters_first(),                   // $@20: %empty
    146 => reduce_actions.action_formal_parameters_tail(),                    // $@21: %empty
    148 => reduce_actions.action_type_name(),                                 // $@22: %empty
    149 => reduce_actions.action_formal_parameter_with_type(),                // formal_parameter: NAME COLON $@22 type
    150 => reduce_actions.action_formal_parameter_without_type(),             // formal_parameter: NAME
    151 => reduce_actions.action_function_body_external(),                    // external: EXTERNAL expression
    152 => reduce_actions.action_function_body(),                             // external: expression
    _ => Ok(()),
  }
}
