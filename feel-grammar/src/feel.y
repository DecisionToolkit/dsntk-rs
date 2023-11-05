%define api.token.prefix {TOKEN_}
%define api.symbol.prefix {SYMBOL_}

%start feel

%token START_EXPRESSION
%token START_BOXED_EXPRESSION
%token START_CONTEXT
%token START_TEXTUAL_EXPRESSION
%token START_TEXTUAL_EXPRESSIONS
%token START_UNARY_TESTS

%token AT
%token NOT
%token COLON
%token COMMA
%token EVERY
%token FOR
%token LEFT_BRACE
%token NULL
%token RIGHT_ARROW
%token OF
%token LIST
%token RANGE
%token CONTEXT
%token THEN
%token FUNCTION
%token EXTERNAL
%token IF
%token RIGHT_BRACE
%token RIGHT_BRACKET
%token RIGHT_PAREN
%token RETURN
%token ELLIPSIS
%token SOME
%token NUMERIC
%token STRING
%token BOOLEAN

%precedence RETURN EXTERNAL SATISFIES
%precedence ELSE
%left OR
%left AND
%nonassoc EQ NQ LT LE GT GE
%precedence BETWEEN
%precedence BETWEEN_AND
%right IN
%left MINUS PLUS
%left MUL DIV
%left EXP
%precedence PREC_NEG
%precedence INSTANCE
%precedence NAME NAME_DATE_TIME BUILT_IN_TYPE_NAME
%precedence LEFT_PAREN LEFT_BRACKET
%precedence DOT

%%

feel:
    START_EXPRESSION expression
  | START_BOXED_EXPRESSION boxed_expression
  | START_CONTEXT context
  | START_TEXTUAL_EXPRESSION textual_expression
  | START_TEXTUAL_EXPRESSIONS textual_expressions
  | START_UNARY_TESTS {/* unary_tests_begin */} unary_tests
  ;

expression:
    boxed_expression
  | textual_expression
  ;

boxed_expression:
    list
  | function_definition
  | context
  ;

textual_expression:
    FOR {/* for_begin */} iteration_contexts RETURN expression {/* for */}
  | IF expression THEN expression ELSE expression {/* if */}
  | SOME {/* some_begin */} quantified_expressions SATISFIES expression {/* some */}
  | EVERY {/* every_begin */} quantified_expressions SATISFIES expression {/* every */}
  | expression BETWEEN {/* between_begin */} expression BETWEEN_AND expression {/* between */}
  | expression OR expression {/* disjunction */}
  | expression AND expression {/* conjunction */}
  | expression EQ expression {/* comparison_eq */}
  | expression NQ expression {/* comparison_nq */}
  | expression LT expression {/* comparison_lt */}
  | expression LE expression {/* comparison_le */}
  | expression GT expression {/* comparison_gt */}
  | expression GE expression {/* comparison_ge */}
  | expression IN LEFT_PAREN comparison_in {/* comparison_in */}
  | expression IN expression {/* comparison_in */}
  | expression PLUS expression {/* addition */}
  | expression MINUS expression {/* subtraction */}
  | expression MUL expression {/* multiplication */}
  | expression DIV expression {/* division */}
  | expression EXP expression {/* exponentiation */}
  | MINUS expression %prec PREC_NEG {/* negation */}
  | expression INSTANCE OF {/* type_name */} type {/* instance_of */}
  | NAME DOT path_segment {/* path_segment */}
  | expression DOT NAME {/* path */}
  | expression LEFT_BRACKET expression RIGHT_BRACKET {/* filter */}
  | expression LEFT_PAREN parameters
  | literal
  | simple_positive_unary_test
  | NAME {/* name */}
  | LEFT_PAREN expression RIGHT_PAREN
  ;

path_segment:
    NAME DOT path_segment {/* path_segment */}
  | NAME {/* path_segment_tail */}
  ;

textual_expressions:
    textual_expression COMMA textual_expressions {/* expression_list_tail */}
  | textual_expression {/* expression_list_tail */}
  ;

unary_tests:
    MINUS {/* unary_tests_irrelevant */}
  | NOT LEFT_PAREN positive_unary_tests RIGHT_PAREN {/* unary_tests_negated */}
  | positive_unary_tests
  ;

positive_unary_tests:
    expression COMMA positive_unary_tests {/* expression_list_tail */}
  | expression {/* expression_list_tail */}
  ;

comparison_in:
    expression COMMA positive_unary_tests RIGHT_PAREN {/* expression_list_tail */}
  ;

simple_positive_unary_test:
    LT endpoint {/* comparison_unary_lt */}
  | LE endpoint {/* comparison_unary_le */}
  | GT endpoint {/* comparison_unary_gt */}
  | GE endpoint {/* comparison_unary_ge */}
  | interval
  ;

interval:
    interval_start interval_end {/* interval */}
  ;

interval_start:
    LEFT_PAREN endpoint ELLIPSIS {/* interval_start */}
  | RIGHT_BRACKET endpoint ELLIPSIS {/* interval_start */}
  | LEFT_BRACKET endpoint ELLIPSIS {/* interval_start */}
  ;

interval_end:
    endpoint RIGHT_PAREN {/* interval_end */}
  | endpoint LEFT_BRACKET {/* interval_end */}
  | endpoint RIGHT_BRACKET {/* interval_end */}
  ;

endpoint:
    simple_value
  ;

simple_value:
    qualified_name
  | simple_literal
  ;

literal:
    simple_literal
  | NULL {/* literal_null */}
  ;

simple_literal:
    NUMERIC {/* literal_numeric */}
  | STRING {/* literal_string */}
  | BOOLEAN {/* literal_boolean */}
  | AT STRING {/* literal_at */}
  | NAME_DATE_TIME LEFT_PAREN {/* literal_date_time */} parameters
  ;

context:
    LEFT_BRACE {/* context_begin */} context_entries {/* context_end */}
  ;

context_entries:
    RIGHT_BRACE {/* empty_context */}
  | context_entry context_entry_tail {/* context_entry_tail */}
  ;

context_entry:
    key COLON expression {/* context_entry */}
  ;

context_entry_tail:
    RIGHT_BRACE
  | COMMA context_entry context_entry_tail {/* context_entry_tail */}
  ;

key:
    NAME {/* key_name */}
  | STRING {/* key_string */}
  ;

list:
    LEFT_BRACKET list_items {/* list */}
  ;

list_items:
    RIGHT_BRACKET {/* list_empty */}
  | expression list_tail {/* list_tail */}
  ;

list_tail:
    RIGHT_BRACKET
  | COMMA expression list_tail {/* list_tail */}
  ;

parameters:
    RIGHT_PAREN {/* function_invocation_no_parameters */}
  | named_parameters {/* function_invocation */}
  | positional_parameters {/* function_invocation */}
  ;

named_parameters:
    named_parameter named_parameters_tail {/* named_parameters_tail */}
  ;

named_parameter:
    NAME COLON expression {/* named_parameter */}
  ;

named_parameters_tail:
    RIGHT_PAREN
  | COMMA named_parameter named_parameters_tail {/* named_parameters_tail */}
  ;

positional_parameters:
    expression positional_parameters_tail {/* positional_parameters_tail */}
  ;

positional_parameters_tail:
    RIGHT_PAREN
  | COMMA expression positional_parameters_tail {/* positional_parameters_tail */}
  ;

qualified_name:
    NAME DOT qualified_name {/* qualified_name_tail */}
  | NAME {/* qualified_name */}
  ;

type:
    BUILT_IN_TYPE_NAME {/* built_in_type_name */}
  | qualified_name
  | LIST LT {/* type_name */} type GT {/* list_type */}
  | RANGE LT {/* type_name */} type GT {/* range_type */}
  | CONTEXT LT context_type_entries
  | FUNCTION LT function_type_parameters RIGHT_ARROW {/* type_name */} type {/* function_type */}
  ;

context_type_entries:
    context_type_entry context_type_entry_tail {/* context_type_entry_tail */}
  ;

context_type_entry:
    NAME COLON {/* type_name */} type {/* context_type_entry */}
  ;

context_type_entry_tail:
    GT
  | COMMA context_type_entry context_type_entry_tail {/* context_type_entry_tail */}
  ;

function_type_parameters:
    GT {/* function_type_parameters_empty */}
  | {/* type_name */} type function_type_parameters_tail {/* function_type_parameters_tail */}
  ;

function_type_parameters_tail:
    GT
  | COMMA {/* type_name */} type function_type_parameters_tail {/* function_type_parameters_tail */}
  ;

iteration_contexts:
    iteration_context COMMA iteration_contexts {/* iteration_contexts_tail */}
  | iteration_context {/* iteration_contexts_tail */}
  ;

iteration_context:
    {/* iteration_context_variable_name_begin */} NAME {/* iteration_context_variable_name */} IN iteration_context_value
  ;

iteration_context_value:
    expression {/* iteration_context_value_single */}
  | expression ELLIPSIS expression {/* iteration_context_value_range */}
  ;

quantified_expressions:
    quantified_expression COMMA quantified_expressions {/* quantified_expressions_tail */}
  | quantified_expression {/* quantified_expressions_tail */}
  ;

quantified_expression:
    {/* quantified_expression_variable_name_begin */} NAME {/* quantified_expression_variable_name */} IN expression {/* quantified_expression */}
  ;

function_definition:
    FUNCTION LEFT_PAREN {/* formal_parameters_begin */} formal_parameters external {/* function_definition */}
  ;

formal_parameters:
    RIGHT_PAREN {/* formal_parameters_empty */}
  | formal_parameter {/* formal_parameters_first */} formal_parameters_tail
  ;

formal_parameters_tail:
    RIGHT_PAREN
  | COMMA formal_parameter {/* formal_parameters_tail */} formal_parameters_tail
  ;

formal_parameter:
    NAME COLON {/* type_name */} type {/* formal_parameter_with_type */}
  | NAME {/* formal_parameter_without_type */}
  ;

external:
    EXTERNAL expression {/* function_body_external */}
  | expression %prec EXTERNAL {/* function_body */}
  ;

%%
