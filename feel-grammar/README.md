**DSNTK** | Decision Toolkit

# dsntk-feel-grammar

[![Crates.io][crates-badge]][crates-url]
![Code coverage][coverage-badge]<br/>
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build MacOs][build-badge-macos]<br/>
[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![Contributor Covenant][cc-badge]][cc-url]

[crates-badge]: https://img.shields.io/crates/v/dsntk-feel-grammar.svg
[crates-url]: https://crates.io/crates/dsntk-feel-grammar
[coverage-badge]: https://img.shields.io/badge/Code%20coverage-100%25-green.svg
[build-badge-linux]: https://github.com/dsntk/dsntk-rs/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/dsntk/dsntk-rs/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/dsntk/dsntk-rs/actions/workflows/build-macos.yml/badge.svg
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://github.com/dsntk/dsntk-rs/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://github.com/dsntk/dsntk-rs/blob/main/LICENSE-APACHE
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[cc-url]: https://github.com/dsntk/dsntk-rs/blob/main/CODE_OF_CONDUCT.md

## Overview

FEEL grammar and parsing tables generator used in [DSNTK](https://github.com/dsntk) project.

This crate implements: 

- The grammar for `FEEL` language in the format acceptable by `Bison` parser generator.
- The generator of LALR parsing tables for `Rust` language.

Excerpt from [Decision Model and Notation™](https://www.omg.org/spec/DMN/1.3/PDF) specification, ver. 1.3, pp. 105-179:

"**DMN 1.3** defines the friendly enough expression language (FEEL) for the purpose
of giving standard executable semantics to many kinds of expressions in decision model.
[...] FEEL syntax is defined as a grammar here [...]". 

**dsntk-feel-grammar** crate defines the `Bison` version of the full `FEEL` grammar 
as defined in clause 10.3 of cited above Decision Model and Notation™ specification.

In addition, **dsntk-feel-grammar** crate provides functionality for generating LALR `parsing tables`
and `reduce actions`, ready to be used by `LALR` parser written in Rust.
Such parser that uses the output from **dsntk-feel-grammar** crate 
is implemented in [dsntk-feel-parser](https://crates.io/crates/dsntk-feel-parser).

## How it works

Inputs and outputs of **dsntk-feel-grammar** crate are depicted in the following diagram:

![feel-grammar](./feel-grammar.svg)

**dsntk-feel-grammar** takes `Yacc/Bison` compatible grammar file as an input.
Grammar file for `FEEL` language is defined as a source file named `feel.y` and compiled
with this crate.

The output from **dsntk-feel-grammar** create is a file named `lalr.rs`.
This file is a Rust source code that contains definitions of parsing tables and reduce actions.

**dsntk-feel-grammar** crate uses [Bison](https://www.gnu.org/software/bison/) parser generator
to process the input grammar file and to generate the parser source for `C` language.
Command that generates the parser for `C` language using `Bison` may look like this:
```shell
$ LANG=C bison -l -r states -L C feel.y
```
`Bison` generates `C` source file that contains the generated parsing tables.
**dsntk-feel-grammar** extracts those tables, adds reduce action definitions and saves
the result code in `lalr.rs` file as Rust source code.

## Requirements

**dsntk-feel-grammar** requires:
- installed the newest Rust nightly version,
- installed the newest `Bison` version.

## Usage

**dsntk-feel-grammar** may be used as a library. 

Add `dsntk-feel-grammar` to your `Cargo.toml`:

```toml
[dependencies]
dsntk-feel-grammar = "0.0.0"
```

To generate LALR parsing tables, call `lalr_rust_tables` function:

```rust
use dsntk_feel_grammar::lalr_rust_tables;

fn main() {
  lalr_rust_tables("./src/lalr.rs");
}
```

## Implemented grammar rules

The following grammar rules are taken from [Decision Model and Notation™](https://www.omg.org/spec/DMN/1.3/PDF) ver. 1.3

1. expression  =
    - a. boxed_expression |
    - b. textual_expression ;

2. textual_expression  =
    - a. for_expression | if_expression | quantified_expression |
    - b. disjunction |
    - c. conjunction |
    - d. comparison |
    - e. arithmetic_expression |
    - f. instance_of |
    - g. path_expression | filter_expression | function_invocation |
    - h. literal | simple_positive_unary_test | name | `(` , expression  , `)` ;

3. textual_expressions  = textual expression  , { `,` , textual_expression  } ;

4. arithmetic_expression =
    - a. addition | subtraction |
    - b. multiplication | division |
    - c. exponentiation |
    - d. arithmetic_negation ;

5. simple_expression  = arithmetic_expression | simple_value  ;

6. simple_expressions  = simple_expression  , { `,` , simple_expression  } ;

7. simple_positive_unary_test  =
    - a. [ `<` | `<=` | `>` | `>=` ] , endpoint |
    - b. interval  ;

8. interval = ( open_interval_start | closed_interval_start ) , 
              endpoint , `..` , endpoint , 
              ( open_interval_end | closed_interval_end ) ;

9. open_interval_start = `(` | `]` ;

10. closed_interval_start = `[` ;

11. open_interval_end = `)` | `[` ;

12. closed_interval_end = `]` ;

13. positive_unary_test = expression ;

14. positive_unary_tests = positive_unary_test , { `,` , positive_unary_test } ;

15. unary_tests =
    - a. positive_unary_tests |
    - b. `not`, `(`, positive_unary_tests, `)` |
    - c. `-` ;

16. endpoint = simple_value ;

17. simple_value = qualified_name | simple_literal ;

18. qualified_name = name , { `.` , name } ;

19. addition = expression , `+` , expression ;

20. subtraction = expression , `-` , expression ;

21. multiplication = expression , `*` , expression ;

22. division = expression , `/` , expression ;

23. exponentiation = expression, `**`, expression ;

24. arithmetic_negation = `-` , expression ;

25. name = name_start , { name_part | additional_name_symbols } ;

26. name_start = name_start_char, { name_part_char } ;

27. name_part = name_part_char , { name_part_char } ;

28. name_start_char = `?`  | `[A-Z]`  | `_` | `[a-z]` 
   | `[\uC0-\uD6]`     | `[\uD8-\uF6]`     | `[\uF8-\u2FF]`    | `[\u370-\u37D]` 
   | `[\u37F-\u1FFF]`  | `[\u200C-\u200D]` | `[\u2070-\u218F]` | `[\u2C00-\u2FEF]` 
   | `[\u3001-\uD7FF]` | `[\uF900-\uFDCF]` | `[\uFDF0-\uFFFD]` | `[\u10000-\uEFFFF]` ;

29. name_part_char = name_start_char | digit | `\uB7` | `[\u0300-\u036F]` | `[\u203F-\u2040]` ;

30. additional_name_symbols = `.` | `/` | `-` | `’` | `+` | `*` ;

31. literal = simple_literal | `null` ;

32. simple_literal = numeric_literal | string_literal | boolean_literal | date_time_literal ;

33. string_literal = `"`, { character – (`"` | vertical_space) | string_escape_sequence}, `"` ;

34. boolean_literal = `true` | `false` ;

35. numeric_literal = [ `-` ] , ( digits , [ `.`, digits ] | `.` , digits ) ;

36. digit = [0-9] ;

37. digits = digit , { digit } ;

38. function_invocation = expression , parameters ;

39. parameters = `(` , ( named_parameters | positional_parameters ) , `)` ;

40. named_parameters = parameter_name , `:` , expression , { `,` , parameter name , `:` , expression } ;

41. parameter_name = name ;

42. positional_parameters = [ expression , { `,` , expression } ] ;

43. path_expression = expression , `.` , name ;

44. for_expression = `for` , name , `in` , iteration context { `,` , name , `in` , iteration context } , `return` , expression ;

45. if_expression = `if` , expression , `then` , expression , `else` expression ;

46. quantified_expression = (`some` | `every`) , name , `in` , expression , { `,` , name , `in` , expression } , `satisfies` , expression ;

47. disjunction = expression , `or` , expression ;

48. conjunction = expression , `and` , expression ;

49. comparison =
    - a. expression , ( `=` | `!=` | `<` | `<=` | `>` | `>=` ) , expression |
    - b. expression , `between` , expression , `and` , expression |
    - c. expression , `in` , positive_unary_test |
    - d. expression , `in` , `(`, positive unary tests, `)` ;

50. filter_expression = expression , `[` , expression , `]` ;

51. instance_of = expression , `instance` , `of` , type ;

52. type =
    - a. qualified_name |
    - b. `list` `<` type `>` |
    - c. `context` `<` name `:` type { `,` name `:` type } `>` |
    - d. `function` `<` [ type { `,` type } ] `>` `->` type ;

53. boxed_expression = list | function_definition | context ;

54. list = `[` , [ expression , { `,` , expression } ] , `]` ;

55. function_definition = `function` , `(` , [ formal_parameter { `,` , formal parameter } ] , `)` , [ `external` ] , expression ;

56. formal_parameter = parameter_name [`:` type ] ;

57. context = `{` , [context_entry , { `,` , context_entry } ] , `}` ;

58. context_entry = key , `:` , expression ;

59. key = name | string_literal ;

60. date_time_literal = at_literal | function_invocation ;

61. white_space = vertical_space | `\u0009` | `\u0020` | `\u0085` | `\u00A0` | `\u1680` | `\u180E` | `[\u2000-\u200B]`
                                 | `\u2028` | `\u2029` | `\u202F` | `\u205F` | `\u3000` | `\uFEFF` ;

62. vertical_space = `[\u000A-\u000D]` ;

63. iteration_context = expression, [ `..`, expression ] ;

64. string_escape_sequence = `\'` | `\"` | `\\` | `\n` | `\r` | `\t` | code_point;

65. at_literal = `@`, string_literal

## License

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT][mit-url]) or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE][apache-url])

at your option.

## Contribution

Any contributions are greatly appreciated.
If you would like to get involved, please don't hesitate to reach out to us.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
