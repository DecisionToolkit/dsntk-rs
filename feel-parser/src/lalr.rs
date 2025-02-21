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
  StartTextualExpression = 260,
  StartTextualExpressions = 261,
  StartUnaryTests = 262,
  StartSimpleExpression = 263,
  StartSimpleExpressions = 264,
  StartSimpleValue = 265,
  StartRangeLiteral = 266,
  At = 267,
  Not = 268,
  Colon = 269,
  Comma = 270,
  Every = 271,
  For = 272,
  LeftBrace = 273,
  Null = 274,
  RightArrow = 275,
  Of = 276,
  List = 277,
  Range = 278,
  Context = 279,
  Then = 280,
  Function = 281,
  External = 282,
  If = 283,
  RightBrace = 284,
  Return = 285,
  Some = 286,
  Numeric = 287,
  String = 288,
  Boolean = 289,
  Satisfies = 290,
  Else = 291,
  Or = 292,
  And = 293,
  Eq = 294,
  Ne = 295,
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
  RightParen = 314,
  LeftBracket = 315,
  RightBracket = 316,
  Ellipsis = 317,
  Dot = 318,
}

/// Kinds of symbols recognized by parser.
#[allow(clippy::enum_variant_names)]
pub enum SymbolKind {
  YyEmpty = -2,
  YyEof = 0,
  YyError = 1,
  YyUndef = 2,
  YyAccept = 64,
}

/// (tbd)
pub const YY_PACT_N_INF: i16 = -186;

/// (tbd)
pub const YY_TABLE_N_INF: i16 = -119;

/// (tbd)
pub const YY_FINAL: usize = 69;

/// (tbd)
pub const YY_LAST: i16 = 1050;

/// (tbd)
pub const YY_N_TOKENS: usize = 64;

/// `YY_TRANSLATE[TOKEN-NUM]` - symbol number corresponding to TOKEN-NUM as returned by lexer.
pub const YY_TRANSLATE: [i8; 319] = [
  0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
];

/// `YY_PACT[STATE-NUM]` - index in YY_TABLE of the portion describing STATE-NUM.
pub const YY_PACT: [i16; 341] = [
  1039, 400, 8, 400, 400, -186, 447, 447, 203, 118, 22, -10, -186, -186, -186, -186, -18, 400, -186, -186, -186, -186, 400, 400, 400, 400, 400, 400, 400, -186, -17, 400, 494, 400,
  875, -186, -186, -186, -186, 400, -186, -186, -186, -186, -186, 494, -186, 875, 46, 150, -186, 253, 400, 24, 902, -186, -186, 151, -186, 34, -186, -5, -186, -186, 5, 33, 129,
  -186, 171, -186, -186, -186, -186, 153, -186, 712, -186, 44, -186, -186, -186, -186, -186, -186, 95, -186, 767, 48, 713, 571, 58, -186, 59, 400, 400, 400, 400, 400, 400, 400,
  400, -186, 541, 400, 400, 400, 400, 400, 110, 303, 400, 78, -186, 187, 571, 400, 94, 400, 664, -186, -186, 28, 105, 400, 400, 400, 400, 400, 447, 134, -186, -186, 116, -186,
  106, 119, -186, 122, -186, -186, -186, 214, 155, 173, 137, 163, 192, 154, -186, -186, -186, -186, 61, 199, -26, 400, 182, 303, -186, -186, 400, -186, -186, -186, -186, 928, 953,
  978, 978, 978, 978, 978, 978, 400, 400, 191, 540, 540, 158, 158, 95, -186, 205, -186, 602, -186, -186, -11, -186, 794, -186, -186, -186, -186, -186, 400, 400, -186, 20, 85, 21,
  117, 142, -186, -186, -186, -186, -186, -186, -186, -186, -186, 400, -186, -186, 400, -186, -186, -8, -186, -186, 400, 208, -186, 353, -186, 740, 400, -186, 571, 821, 633, -186,
  89, 400, 400, -186, -186, 169, -186, -186, -186, 147, -186, 9, 875, -186, 178, 875, -186, 181, 61, 875, -186, 400, 875, -186, 0, 400, 875, -186, 400, 400, 188, 209, 211, 215,
  -186, -186, -186, 875, 602, 205, -11, -186, -186, 218, 185, 400, 400, -186, 89, 875, 200, -186, -186, 875, 191, 204, -186, -186, 207, 221, -186, -186, -186, 234, -186, 875, 848,
  -186, -186, -186, -186, 89, 89, 254, -186, 17, -186, 256, 89, -186, 400, 0, 237, 239, -186, 207, -186, -186, -186, 104, -186, 875, -186, -186, -186, 89, 17, 89, -186, -186,
  -186, -186, -186, -186, -186, 89, 104, -186,
];

/// `YY_DEF_ACT[STATE-NUM]` - default reduction number in state STATE-NUM.
/// Performed when YY_TABLE does not specify something else to do.
/// Zero means the default is an error.
pub const YY_DEF_ACT: [u8; 341] = [
  0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 22, 17, 93, 86, 0, 0, 20, 87, 88, 89, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 2, 12, 13, 48, 74, 0, 47, 85, 16, 14, 15, 0, 3, 0, 13, 13, 5, 0,
  0, 49, 0, 8, 65, 85, 83, 67, 9, 118, 10, 84, 0, 0, 0, 11, 0, 1, 90, 148, 141, 0, 151, 0, 148, 82, 72, 73, 68, 69, 70, 71, 41, 91, 82, 0, 103, 82, 0, 102, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 53, 57, 7, 55, 41, 0, 0, 0, 0, 0, 0, 0, 0, 175, 176, 0, 165, 0, 0, 166, 0, 170, 171, 164, 0, 0, 147, 0, 0, 140, 0, 95, 101,
  100, 94, 0, 0, 0, 0, 0, 0, 50, 76, 0, 105, 104, 78, 77, 26, 27, 28, 29, 30, 31, 32, 33, 0, 0, 35, 37, 36, 38, 39, 40, 42, 49, 107, 0, 46, 108, 0, 109, 0, 44, 79, 80, 81, 51, 0,
  0, 117, 37, 36, 38, 39, 40, 66, 177, 178, 167, 169, 168, 172, 173, 174, 0, 148, 149, 0, 141, 142, 0, 98, 96, 0, 161, 153, 0, 154, 0, 0, 92, 0, 0, 82, 34, 0, 0, 0, 115, 114, 0,
  112, 110, 45, 0, 56, 0, 23, 146, 0, 18, 139, 0, 0, 97, 159, 0, 163, 152, 0, 0, 21, 106, 0, 0, 0, 0, 0, 0, 119, 120, 43, 111, 0, 0, 0, 54, 183, 0, 0, 0, 0, 99, 0, 162, 0, 156,
  155, 19, 25, 0, 121, 123, 0, 134, 116, 113, 184, 0, 179, 150, 144, 143, 160, 157, 58, 0, 0, 0, 125, 0, 133, 0, 0, 180, 0, 0, 0, 0, 129, 0, 131, 128, 126, 0, 181, 145, 158, 122,
  124, 0, 0, 0, 137, 136, 135, 182, 130, 132, 127, 0, 0, 138,
];

/// `YY_P_GOTO[NTERM-NUM]`
pub const YY_P_GOTO: [i16; 81] = [
  -186, -186, -186, -1, 281, 15, -186, -186, -186, -186, -186, 174, -186, -185, -186, 282, 162, -186, -186, -186, -186, 30, 283, -186, 1, -186, -186, -186, -186, 80, 49, -186,
  -186, -186, 70, 145, -186, 65, 31, -186, 35, 6, -141, -186, -186, -186, -186, -13, -186, -22, -186, -186, -29, -186, 100, -186, -186, -186, -186, -75, -186, -186, -186, -186,
  -186, -186, -186, 3, -186, 40, -186, -186, -186, -186, -186, 133, -186, -186, -186, -186, -186,
];

/// `YY_DEF_GOTO[NTERM-NUM]`
pub const YY_DEF_GOTO: [i16; 81] = [
  0, 10, 51, 77, 35, 36, 72, 76, 71, 173, 233, 50, 119, 120, 232, 59, 60, 37, 38, 39, 112, 87, 56, 40, 41, 157, 42, 73, 151, 152, 220, 153, 43, 91, 162, 185, 186, 187, 240, 188,
  237, 268, 269, 304, 305, 330, 307, 308, 328, 320, 310, 311, 333, 338, 145, 146, 147, 250, 300, 142, 143, 144, 247, 44, 154, 224, 257, 285, 314, 225, 281, 256, 67, 68, 140, 134,
  244, 277, 323, 334, 295,
];

/// `YY_TABLE[YY_PACT[STATE-NUM]]` - what to do in state STATE-NUM.
/// If positive, shift that token.
/// If negative, reduce the rule whose number is the opposite.
/// If `YY_TABLE_N_INF`, syntax error.
pub const YY_TABLE: [i16; 1051] = [
  34, 156, 47, 47, 238, 54, 54, 57, 57, 63, 242, 243, 58, 58, 58, 283, 75, 129, 48, 49, -60, -61, 69, 70, -118, 149, 14, 84, -64, 222, 86, 89, 318, 223, 16, -60, -61, 130, 131,
  -118, 74, 85, 275, -64, 114, 129, -4, 150, 239, 128, 118, 121, 78, 79, 80, 81, 82, 83, 122, 284, 319, 132, 90, 92, 276, 130, 131, 133, 45, 113, 105, 106, 107, 107, 108, 108,
  218, 288, 109, 109, 110, 110, 108, 111, 111, -59, 109, 122, 110, 132, 219, 111, 165, 166, 167, 168, 169, 170, 171, 172, -59, 175, 176, 177, 178, 179, 180, 111, 184, 189, 159,
  263, 264, 265, 47, 266, 84, -62, 92, 331, 163, 164, 198, 199, 200, 201, 202, 54, 197, 57, 49, 181, -62, 190, 58, 105, 106, 107, 246, 108, 301, 129, -63, 109, 61, 110, 267, 332,
  111, 108, -52, -84, 195, 109, 226, 110, 184, -63, 111, 229, 61, 130, 131, 315, 316, 115, -84, 204, 206, 107, 322, 108, 230, 231, 205, 109, 64, 110, 65, 66, 111, 207, 148, 129,
  208, 132, 149, 335, 213, 337, 212, 136, 214, 215, 118, 118, 108, 339, 135, 137, 109, 141, 110, 130, 131, 111, 274, 216, 150, 217, 107, 245, 108, 221, 248, 11, 109, 227, 110,
  234, 252, 111, 253, 255, 272, 278, 259, 132, 279, 289, 138, 139, 296, 270, 271, 19, 20, 21, 102, 103, 104, 105, 106, 107, 297, 108, 191, 192, 193, 109, 290, 110, 291, 282, 111,
  222, 292, 286, 61, 30, 287, 118, 306, 303, 309, 11, 116, 312, 317, 12, 13, 14, 15, 209, 210, 211, 321, 298, 299, 16, 326, 17, 327, 46, 18, 19, 20, 21, 55, 194, 203, 62, 22, 23,
  24, 25, 26, 27, 251, 260, 280, 117, 228, 273, 294, 329, 293, 336, 29, 30, 340, 31, 324, 32, 33, 11, 249, 325, 0, 12, 13, 14, 15, 302, 0, 0, 0, 0, 0, 16, 0, 17, 0, 0, 18, 19, 20,
  21, 0, 0, 0, 0, 22, 23, 24, 25, 26, 27, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 182, 30, 0, 31, 183, 32, 33, 11, 0, 0, 0, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 16, 254, 17, 0, 0, 18, 19, 20,
  21, 0, 0, 0, 0, 22, 23, 24, 25, 26, 27, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 29, 30, 0, 31, 11, 32, 33, 0, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 16, 0, 17, 0, 0, 18, 19, 20, 21, 0, 0, 0,
  0, 22, 23, 24, 25, 26, 27, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 29, 30, 0, 31, 11, 32, 33, 0, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 16, 0, 17, 0, 0, 18, 19, 20, 21, 0, 0, 0, 0, 22, 23,
  24, 25, 26, 27, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 53, 30, 0, 31, 11, 32, 33, 0, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 16, 0, 17, 0, 0, 18, 19, 20, 21, 0, 0, 0, 0, 22, 23, 24, 25, 26,
  27, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 29, 30, 0, 31, 11, 32, 88, 0, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 16, 0, 17, 0, 0, 18, 19, 20, 21, 0, 0, 0, 0, 22, 23, 24, 25, 26, 27, 160, 0,
  0, 28, 105, 106, 107, 0, 108, 0, 29, 30, 109, 174, 110, 32, 33, 111, 0, 0, 0, 0, 93, 94, 95, 96, 97, 98, 99, 100, 101, 235, 102, 103, 104, 105, 106, 107, 0, 108, 0, 0, 0, 109,
  0, 110, 161, 0, 111, 0, 0, 0, 0, 93, 94, 95, 96, 97, 98, 99, 100, 101, 262, 102, 103, 104, 105, 106, 107, 0, 108, 0, 0, 0, 109, 236, 110, 0, 0, 111, 0, 0, 0, 0, 93, 94, 95, 96,
  97, 98, 99, 100, 101, 196, 102, 103, 104, 105, 106, 107, 0, 108, 0, 0, 0, 109, 158, 110, 0, 0, 111, 0, 0, 0, 0, 93, 94, 95, 96, 97, 98, 99, 100, 101, 0, 102, 103, 104, 105, 106,
  107, 0, 108, 0, 0, 0, 109, 0, 110, 11, 0, 111, 0, 12, 13, 14, 15, 0, 0, 0, 0, 155, 0, 16, 0, 17, 0, 0, 18, 19, 20, 21, 0, 93, 94, 95, 96, 97, 98, 99, 100, 101, 0, 102, 103, 104,
  105, 106, 107, 0, 108, 0, 29, 30, 109, 0, 110, 0, 0, 111, 258, 93, 94, 95, 96, 97, 98, 99, 100, 101, 0, 102, 103, 104, 105, 106, 107, 0, 108, 0, 0, 0, 109, 0, 110, 0, 0, 111,
  93, 94, 95, 96, 97, 98, 99, 100, 101, 0, 102, 103, 104, 105, 106, 107, 0, 108, 0, 0, 0, 109, 158, 110, 0, 0, 111, 93, 94, 95, 96, 97, 98, 99, 100, 101, 0, 102, 103, 104, 105,
  106, 107, 0, 108, 0, 0, 0, 109, 0, 110, 241, 0, 111, 93, 94, 95, 96, 97, 98, 99, 100, 101, 261, 102, 103, 104, 105, 106, 107, 0, 108, 0, 0, 0, 109, 0, 110, 0, 0, 111, 93, 94,
  95, 96, 97, 98, 99, 100, 101, 0, 102, 103, 104, 105, 106, 107, 0, 108, 0, 0, 0, 109, 0, 110, 0, 313, 111, 93, 94, 95, 96, 97, 98, 99, 100, 101, 0, 102, 103, 104, 105, 106, 107,
  0, 108, 0, 0, 0, 109, 0, 110, 0, 0, 111, 93, 94, 95, 96, 97, 98, 99, 100, 101, 0, 102, 123, 124, 125, 126, 127, 0, 108, 0, 0, 0, 109, 0, 110, 0, 0, 111, 94, 95, 96, 97, 98, 99,
  100, 101, 0, 102, 103, 104, 105, 106, 107, 0, 108, 0, 0, 0, 109, 0, 110, 0, 0, 111, 95, 96, 97, 98, 99, 100, 101, 0, 102, 103, 104, 105, 106, 107, 0, 108, 0, 0, 0, 109, 0, 110,
  0, 0, 111, -119, -119, -119, -119, -119, -119, 101, 0, 102, 103, 104, 105, 106, 107, 0, 108, 0, 0, 0, 109, 0, 110, 0, 0, 111, 1, 2, 3, 4, 5, 6, 7, 8, 9,
];

/// ???
pub const YY_CHECK: [i16; 1051] = [
  1, 76, 3, 4, 15, 6, 7, 6, 7, 8, 195, 196, 6, 7, 8, 15, 17, 12, 3, 4, 0, 0, 0, 33, 0, 33, 18, 28, 0, 55, 31, 32, 15, 59, 26, 15, 15, 32, 33, 15, 58, 58, 33, 15, 45, 12, 0, 55,
  59, 15, 51, 52, 22, 23, 24, 25, 26, 27, 63, 59, 43, 56, 32, 33, 55, 32, 33, 62, 60, 39, 50, 51, 52, 52, 54, 54, 15, 262, 58, 58, 60, 60, 54, 63, 63, 0, 58, 63, 60, 56, 29, 63,
  93, 94, 95, 96, 97, 98, 99, 100, 15, 102, 103, 104, 105, 106, 107, 63, 109, 110, 62, 22, 23, 24, 115, 26, 117, 0, 88, 15, 62, 62, 123, 124, 125, 126, 127, 128, 122, 128, 115,
  21, 15, 55, 128, 50, 51, 52, 213, 54, 281, 12, 0, 58, 55, 60, 57, 43, 63, 54, 0, 0, 58, 58, 155, 60, 157, 15, 63, 160, 55, 32, 33, 304, 305, 15, 15, 33, 62, 52, 311, 54, 173,
  174, 58, 58, 58, 60, 60, 61, 63, 62, 29, 12, 62, 56, 33, 328, 15, 330, 35, 62, 55, 30, 195, 196, 54, 338, 65, 66, 58, 68, 60, 32, 33, 63, 59, 15, 55, 55, 52, 212, 54, 14, 215,
  12, 58, 35, 60, 14, 221, 63, 14, 224, 55, 47, 227, 56, 47, 41, 59, 60, 14, 234, 235, 32, 33, 34, 47, 48, 49, 50, 51, 52, 59, 54, 59, 60, 61, 58, 41, 60, 41, 254, 63, 55, 41,
  258, 55, 56, 261, 262, 55, 59, 43, 12, 13, 33, 14, 16, 17, 18, 19, 59, 60, 61, 20, 278, 279, 26, 43, 28, 43, 2, 31, 32, 33, 34, 6, 115, 128, 8, 39, 40, 41, 42, 43, 44, 218, 229,
  251, 48, 157, 238, 273, 318, 271, 329, 55, 56, 339, 58, 313, 60, 61, 12, 216, 314, -1, 16, 17, 18, 19, 283, -1, -1, -1, -1, -1, 26, -1, 28, -1, -1, 31, 32, 33, 34, -1, -1, -1,
  -1, 39, 40, 41, 42, 43, 44, -1, -1, -1, 48, -1, -1, -1, -1, -1, -1, 55, 56, -1, 58, 59, 60, 61, 12, -1, -1, -1, 16, 17, 18, 19, -1, -1, -1, -1, -1, -1, 26, 27, 28, -1, -1, 31,
  32, 33, 34, -1, -1, -1, -1, 39, 40, 41, 42, 43, 44, -1, -1, -1, 48, -1, -1, -1, -1, -1, -1, 55, 56, -1, 58, 12, 60, 61, -1, 16, 17, 18, 19, -1, -1, -1, -1, -1, -1, 26, -1, 28,
  -1, -1, 31, 32, 33, 34, -1, -1, -1, -1, 39, 40, 41, 42, 43, 44, -1, -1, -1, 48, -1, -1, -1, -1, -1, -1, 55, 56, -1, 58, 12, 60, 61, -1, 16, 17, 18, 19, -1, -1, -1, -1, -1, -1,
  26, -1, 28, -1, -1, 31, 32, 33, 34, -1, -1, -1, -1, 39, 40, 41, 42, 43, 44, -1, -1, -1, 48, -1, -1, -1, -1, -1, -1, 55, 56, -1, 58, 12, 60, 61, -1, 16, 17, 18, 19, -1, -1, -1,
  -1, -1, -1, 26, -1, 28, -1, -1, 31, 32, 33, 34, -1, -1, -1, -1, 39, 40, 41, 42, 43, 44, -1, -1, -1, 48, -1, -1, -1, -1, -1, -1, 55, 56, -1, 58, 12, 60, 61, -1, 16, 17, 18, 19,
  -1, -1, -1, -1, -1, -1, 26, -1, 28, -1, -1, 31, 32, 33, 34, -1, -1, -1, -1, 39, 40, 41, 42, 43, 44, 15, -1, -1, 48, 50, 51, 52, -1, 54, -1, 55, 56, 58, 58, 60, 60, 61, 63, -1,
  -1, -1, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, 15, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, -1, 60, 61, -1, 63, -1, -1, -1, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, 15,
  47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, -1, -1, 63, -1, -1, -1, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, 15, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59,
  60, -1, -1, 63, -1, -1, -1, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, -1, 60, 12, -1, 63, -1, 16, 17, 18, 19, -1, -1, -1, -1,
  25, -1, 26, -1, 28, -1, -1, 31, 32, 33, 34, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, 55, 56, 58, -1, 60, -1, -1, 63, 36, 37, 38, 39, 40,
  41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, -1, 60, -1, -1, 63, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1,
  58, 59, 60, -1, -1, 63, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, -1, 60, 61, -1, 63, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, -1, 60, -1, -1, 63, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, -1, 60, -1, 62, 63, 37,
  38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, -1, 60, -1, -1, 63, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54,
  -1, -1, -1, 58, -1, 60, -1, -1, 63, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, -1, 60, -1, -1, 63, 39, 40, 41, 42, 43, 44, 45, -1, 47,
  48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, -1, 60, -1, -1, 63, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, -1, 60, -1, -1, 63, 3, 4, 5, 6,
  7, 8, 9, 10, 11,
];

/// `YY_R1[YYN]` - symbol number of symbol that rule YYN derives.
pub const YY_R1: [u8; 185] = [
  0, 64, 65, 65, 65, 65, 66, 65, 65, 65, 65, 65, 67, 67, 68, 68, 68, 70, 69, 69, 71, 69, 72, 69, 73, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 74, 69,
  69, 69, 69, 69, 69, 69, 69, 75, 75, 76, 76, 76, 77, 77, 78, 79, 79, 79, 79, 79, 79, 79, 80, 80, 81, 81, 81, 81, 81, 81, 81, 82, 83, 83, 83, 84, 84, 84, 85, 86, 86, 87, 87, 88,
  88, 88, 88, 89, 88, 91, 90, 92, 92, 93, 94, 94, 95, 95, 96, 97, 97, 98, 98, 99, 99, 99, 100, 101, 102, 102, 103, 104, 104, 105, 105, 106, 106, 107, 106, 108, 106, 106, 109, 106,
  110, 112, 111, 113, 113, 114, 115, 114, 116, 117, 116, 118, 118, 120, 121, 119, 122, 122, 123, 123, 125, 126, 124, 128, 127, 129, 130, 129, 131, 132, 131, 134, 133, 133, 135,
  135, 136, 137, 137, 137, 137, 137, 138, 138, 138, 138, 138, 139, 139, 139, 140, 139, 142, 143, 141, 144, 141,
];

/// `YY_R2[YYN]` - number of symbols on the right hand side of rule YYN.
pub const YY_R2: [i8; 185] = [
  0, 2, 2, 2, 2, 2, 0, 3, 2, 2, 2, 2, 1, 1, 1, 1, 1, 0, 5, 6, 0, 5, 0, 5, 0, 6, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 2, 0, 5, 3, 4, 3, 1, 1, 1, 3, 3, 1, 1, 4, 1, 3, 1, 4,
  3, 3, 3, 3, 3, 2, 1, 3, 1, 2, 2, 2, 2, 2, 2, 1, 2, 3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 0, 4, 0, 3, 1, 2, 3, 1, 3, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 3, 1, 3, 2, 1, 3, 3,
  1, 1, 1, 0, 5, 0, 5, 3, 0, 6, 2, 0, 4, 1, 3, 1, 0, 3, 1, 0, 4, 3, 1, 0, 0, 5, 1, 3, 3, 1, 0, 0, 5, 0, 5, 1, 0, 3, 1, 0, 4, 0, 4, 1, 2, 1, 2, 2, 2, 3, 3, 3, 1, 1, 2, 2, 2, 1, 1,
  2, 0, 5, 0, 0, 5, 0, 2,
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
  fn action_comparison_ne(&mut self) -> Result<()>;
  fn action_comparison_unary_eq(&mut self) -> Result<()>;
  fn action_comparison_unary_ge(&mut self) -> Result<()>;
  fn action_comparison_unary_gt(&mut self) -> Result<()>;
  fn action_comparison_unary_le(&mut self) -> Result<()>;
  fn action_comparison_unary_lt(&mut self) -> Result<()>;
  fn action_comparison_unary_ne(&mut self) -> Result<()>;
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
  fn action_positional_parameters_tail(&mut self) -> Result<()>;
  fn action_qualified_name(&mut self) -> Result<()>;
  fn action_qualified_name_tail(&mut self) -> Result<()>;
  fn action_quantified_expression(&mut self) -> Result<()>;
  fn action_quantified_expression_variable_name(&mut self) -> Result<()>;
  fn action_quantified_expression_variable_name_begin(&mut self) -> Result<()>;
  fn action_quantified_expressions_tail(&mut self) -> Result<()>;
  fn action_range_literal(&mut self) -> Result<()>;
  fn action_range_literal_empty_end(&mut self) -> Result<()>;
  fn action_range_literal_empty_start(&mut self) -> Result<()>;
  fn action_range_literal_end(&mut self) -> Result<()>;
  fn action_range_literal_start(&mut self) -> Result<()>;
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
    6 => reduce_actions.action_unary_tests_begin(),
    17 => reduce_actions.action_for_begin(),
    18 => reduce_actions.action_for(),
    19 => reduce_actions.action_if(),
    20 => reduce_actions.action_some_begin(),
    21 => reduce_actions.action_some(),
    22 => reduce_actions.action_every_begin(),
    23 => reduce_actions.action_every(),
    24 => reduce_actions.action_between_begin(),
    25 => reduce_actions.action_between(),
    26 => reduce_actions.action_disjunction(),
    27 => reduce_actions.action_conjunction(),
    28 => reduce_actions.action_comparison_eq(),
    29 => reduce_actions.action_comparison_ne(),
    30 => reduce_actions.action_comparison_lt(),
    31 => reduce_actions.action_comparison_le(),
    32 => reduce_actions.action_comparison_gt(),
    33 => reduce_actions.action_comparison_ge(),
    34 => reduce_actions.action_comparison_in(),
    35 => reduce_actions.action_comparison_in(),
    36 => reduce_actions.action_addition(),
    37 => reduce_actions.action_subtraction(),
    38 => reduce_actions.action_multiplication(),
    39 => reduce_actions.action_division(),
    40 => reduce_actions.action_exponentiation(),
    41 => reduce_actions.action_negation(),
    42 => reduce_actions.action_type_name(),
    43 => reduce_actions.action_instance_of(),
    44 => reduce_actions.action_path(),
    45 => reduce_actions.action_filter(),
    49 => reduce_actions.action_name(),
    51 => reduce_actions.action_expression_list_tail(),
    52 => reduce_actions.action_expression_list_tail(),
    53 => reduce_actions.action_unary_tests_irrelevant(),
    54 => reduce_actions.action_unary_tests_negated(),
    56 => reduce_actions.action_expression_list_tail(),
    57 => reduce_actions.action_expression_list_tail(),
    58 => reduce_actions.action_expression_list_tail(),
    59 => reduce_actions.action_addition(),
    60 => reduce_actions.action_subtraction(),
    61 => reduce_actions.action_multiplication(),
    62 => reduce_actions.action_division(),
    63 => reduce_actions.action_exponentiation(),
    64 => reduce_actions.action_negation(),
    66 => reduce_actions.action_expression_list_tail(),
    67 => reduce_actions.action_expression_list_tail(),
    68 => reduce_actions.action_comparison_unary_lt(),
    69 => reduce_actions.action_comparison_unary_le(),
    70 => reduce_actions.action_comparison_unary_gt(),
    71 => reduce_actions.action_comparison_unary_ge(),
    72 => reduce_actions.action_comparison_unary_eq(),
    73 => reduce_actions.action_comparison_unary_ne(),
    75 => reduce_actions.action_interval(),
    76 => reduce_actions.action_interval_start(),
    77 => reduce_actions.action_interval_start(),
    78 => reduce_actions.action_interval_start(),
    79 => reduce_actions.action_interval_end(),
    80 => reduce_actions.action_interval_end(),
    81 => reduce_actions.action_interval_end(),
    86 => reduce_actions.action_literal_null(),
    87 => reduce_actions.action_literal_numeric(),
    88 => reduce_actions.action_literal_string(),
    89 => reduce_actions.action_literal_boolean(),
    90 => reduce_actions.action_literal_at(),
    91 => reduce_actions.action_literal_date_time(),
    93 => reduce_actions.action_context_begin(),
    94 => reduce_actions.action_context_end(),
    95 => reduce_actions.action_empty_context(),
    96 => reduce_actions.action_context_entry_tail(),
    97 => reduce_actions.action_context_entry(),
    99 => reduce_actions.action_context_entry_tail(),
    100 => reduce_actions.action_key_name(),
    101 => reduce_actions.action_key_string(),
    102 => reduce_actions.action_list(),
    103 => reduce_actions.action_list_empty(),
    104 => reduce_actions.action_list_tail(),
    106 => reduce_actions.action_list_tail(),
    107 => reduce_actions.action_function_invocation_no_parameters(),
    108 => reduce_actions.action_function_invocation(),
    109 => reduce_actions.action_function_invocation(),
    110 => reduce_actions.action_named_parameters_tail(),
    111 => reduce_actions.action_named_parameter(),
    113 => reduce_actions.action_named_parameters_tail(),
    114 => reduce_actions.action_positional_parameters_tail(),
    116 => reduce_actions.action_positional_parameters_tail(),
    117 => reduce_actions.action_qualified_name_tail(),
    118 => reduce_actions.action_qualified_name(),
    119 => reduce_actions.action_built_in_type_name(),
    121 => reduce_actions.action_type_name(),
    122 => reduce_actions.action_list_type(),
    123 => reduce_actions.action_type_name(),
    124 => reduce_actions.action_range_type(),
    126 => reduce_actions.action_type_name(),
    127 => reduce_actions.action_function_type(),
    128 => reduce_actions.action_context_type_entry_tail(),
    129 => reduce_actions.action_type_name(),
    130 => reduce_actions.action_context_type_entry(),
    132 => reduce_actions.action_context_type_entry_tail(),
    133 => reduce_actions.action_function_type_parameters_empty(),
    134 => reduce_actions.action_type_name(),
    135 => reduce_actions.action_function_type_parameters_tail(),
    137 => reduce_actions.action_type_name(),
    138 => reduce_actions.action_function_type_parameters_tail(),
    139 => reduce_actions.action_iteration_contexts_tail(),
    140 => reduce_actions.action_iteration_contexts_tail(),
    141 => reduce_actions.action_iteration_context_variable_name_begin(),
    142 => reduce_actions.action_iteration_context_variable_name(),
    144 => reduce_actions.action_iteration_context_value_single(),
    145 => reduce_actions.action_iteration_context_value_range(),
    146 => reduce_actions.action_quantified_expressions_tail(),
    147 => reduce_actions.action_quantified_expressions_tail(),
    148 => reduce_actions.action_quantified_expression_variable_name_begin(),
    149 => reduce_actions.action_quantified_expression_variable_name(),
    150 => reduce_actions.action_quantified_expression(),
    151 => reduce_actions.action_formal_parameters_begin(),
    152 => reduce_actions.action_function_definition(),
    153 => reduce_actions.action_formal_parameters_empty(),
    154 => reduce_actions.action_formal_parameters_first(),
    157 => reduce_actions.action_formal_parameters_tail(),
    159 => reduce_actions.action_type_name(),
    160 => reduce_actions.action_formal_parameter_with_type(),
    161 => reduce_actions.action_formal_parameter_without_type(),
    162 => reduce_actions.action_function_body_external(),
    163 => reduce_actions.action_function_body(),
    164 => reduce_actions.action_range_literal(),
    165 => reduce_actions.action_range_literal_empty_start(),
    166 => reduce_actions.action_range_literal_empty_start(),
    167 => reduce_actions.action_range_literal_start(),
    168 => reduce_actions.action_range_literal_start(),
    169 => reduce_actions.action_range_literal_start(),
    170 => reduce_actions.action_range_literal_empty_end(),
    171 => reduce_actions.action_range_literal_empty_end(),
    172 => reduce_actions.action_range_literal_end(),
    173 => reduce_actions.action_range_literal_end(),
    174 => reduce_actions.action_range_literal_end(),
    175 => reduce_actions.action_literal_numeric(),
    176 => reduce_actions.action_literal_string(),
    177 => reduce_actions.action_literal_at(),
    178 => reduce_actions.action_literal_date_time(),
    179 => reduce_actions.action_function_invocation(),
    180 => reduce_actions.action_literal_string(),
    181 => reduce_actions.action_named_parameter(),
    182 => reduce_actions.action_named_parameters_tail(),
    183 => reduce_actions.action_literal_string(),
    184 => reduce_actions.action_positional_parameters_tail(),
    _ => Ok(()),
  }
}
