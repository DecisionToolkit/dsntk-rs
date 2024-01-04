/* A Bison parser, made by GNU Bison 3.7.6.  */

/* Bison implementation for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2021 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */

/* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */

/* Identify Bison output, and Bison version.  */
#define YYBISON 30706

/* Bison version string.  */
#define YYBISON_VERSION "3.7.6"

/* Skeleton name.  */
#define YYSKELETON_NAME "yacc.c"

/* Pure parsers.  */
#define YYPURE 0

/* Push parsers.  */
#define YYPUSH 0

/* Pull parsers.  */
#define YYPULL 1





# ifndef YY_CAST
#  ifdef __cplusplus
#   define YY_CAST(Type, Val) static_cast<Type> (Val)
#   define YY_REINTERPRET_CAST(Type, Val) reinterpret_cast<Type> (Val)
#  else
#   define YY_CAST(Type, Val) ((Type) (Val))
#   define YY_REINTERPRET_CAST(Type, Val) ((Type) (Val))
#  endif
# endif
# ifndef YY_NULLPTR
#  if defined __cplusplus
#   if 201103L <= __cplusplus
#    define YY_NULLPTR nullptr
#   else
#    define YY_NULLPTR 0
#   endif
#  else
#   define YY_NULLPTR ((void*)0)
#  endif
# endif


/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int yydebug;
#endif

/* Token kinds.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    TOKEN_YYEMPTY = -2,
    TOKEN_YYEOF = 0,               /* "end of file"  */
    TOKEN_YYerror = 256,           /* error  */
    TOKEN_YYUNDEF = 257,           /* "invalid token"  */
    TOKEN_START_EXPRESSION = 258,  /* START_EXPRESSION  */
    TOKEN_START_BOXED_EXPRESSION = 259, /* START_BOXED_EXPRESSION  */
    TOKEN_START_CONTEXT = 260,     /* START_CONTEXT  */
    TOKEN_START_TEXTUAL_EXPRESSION = 261, /* START_TEXTUAL_EXPRESSION  */
    TOKEN_START_TEXTUAL_EXPRESSIONS = 262, /* START_TEXTUAL_EXPRESSIONS  */
    TOKEN_START_UNARY_TESTS = 263, /* START_UNARY_TESTS  */
    TOKEN_START_SIMPLE_EXPRESSION = 264, /* START_SIMPLE_EXPRESSION  */
    TOKEN_START_SIMPLE_EXPRESSIONS = 265, /* START_SIMPLE_EXPRESSIONS  */
    TOKEN_START_SIMPLE_VALUE = 266, /* START_SIMPLE_VALUE  */
    TOKEN_START_RANGE_LITERAL = 267, /* START_RANGE_LITERAL  */
    TOKEN_AT = 268,                /* AT  */
    TOKEN_NOT = 269,               /* NOT  */
    TOKEN_COLON = 270,             /* COLON  */
    TOKEN_COMMA = 271,             /* COMMA  */
    TOKEN_EVERY = 272,             /* EVERY  */
    TOKEN_FOR = 273,               /* FOR  */
    TOKEN_LEFT_BRACE = 274,        /* LEFT_BRACE  */
    TOKEN_NULL = 275,              /* NULL  */
    TOKEN_RIGHT_ARROW = 276,       /* RIGHT_ARROW  */
    TOKEN_OF = 277,                /* OF  */
    TOKEN_LIST = 278,              /* LIST  */
    TOKEN_RANGE = 279,             /* RANGE  */
    TOKEN_CONTEXT = 280,           /* CONTEXT  */
    TOKEN_THEN = 281,              /* THEN  */
    TOKEN_FUNCTION = 282,          /* FUNCTION  */
    TOKEN_EXTERNAL = 283,          /* EXTERNAL  */
    TOKEN_IF = 284,                /* IF  */
    TOKEN_RIGHT_BRACE = 285,       /* RIGHT_BRACE  */
    TOKEN_RETURN = 286,            /* RETURN  */
    TOKEN_SOME = 287,              /* SOME  */
    TOKEN_NUMERIC = 288,           /* NUMERIC  */
    TOKEN_STRING = 289,            /* STRING  */
    TOKEN_BOOLEAN = 290,           /* BOOLEAN  */
    TOKEN_SATISFIES = 291,         /* SATISFIES  */
    TOKEN_ELSE = 292,              /* ELSE  */
    TOKEN_OR = 293,                /* OR  */
    TOKEN_AND = 294,               /* AND  */
    TOKEN_EQ = 295,                /* EQ  */
    TOKEN_NE = 296,                /* NE  */
    TOKEN_LT = 297,                /* LT  */
    TOKEN_LE = 298,                /* LE  */
    TOKEN_GT = 299,                /* GT  */
    TOKEN_GE = 300,                /* GE  */
    TOKEN_BETWEEN = 301,           /* BETWEEN  */
    TOKEN_BETWEEN_AND = 302,       /* BETWEEN_AND  */
    TOKEN_IN = 303,                /* IN  */
    TOKEN_MINUS = 304,             /* MINUS  */
    TOKEN_PLUS = 305,              /* PLUS  */
    TOKEN_MUL = 306,               /* MUL  */
    TOKEN_DIV = 307,               /* DIV  */
    TOKEN_EXP = 308,               /* EXP  */
    TOKEN_PREC_NEG = 309,          /* PREC_NEG  */
    TOKEN_INSTANCE = 310,          /* INSTANCE  */
    TOKEN_NAME = 311,              /* NAME  */
    TOKEN_NAME_DATE_TIME = 312,    /* NAME_DATE_TIME  */
    TOKEN_BUILT_IN_TYPE_NAME = 313, /* BUILT_IN_TYPE_NAME  */
    TOKEN_LEFT_PAREN = 314,        /* LEFT_PAREN  */
    TOKEN_RIGHT_PAREN = 315,       /* RIGHT_PAREN  */
    TOKEN_LEFT_BRACKET = 316,      /* LEFT_BRACKET  */
    TOKEN_RIGHT_BRACKET = 317,     /* RIGHT_BRACKET  */
    TOKEN_ELLIPSIS = 318,          /* ELLIPSIS  */
    TOKEN_DOT = 319                /* DOT  */
  };
  typedef enum yytokentype yytoken_kind_t;
#endif

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
typedef int YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE yylval;

int yyparse (void);


/* Symbol kind.  */
enum yysymbol_kind_t
{
  SYMBOL_YYEMPTY = -2,
  SYMBOL_YYEOF = 0,                        /* "end of file"  */
  SYMBOL_YYerror = 1,                      /* error  */
  SYMBOL_YYUNDEF = 2,                      /* "invalid token"  */
  SYMBOL_START_EXPRESSION = 3,             /* START_EXPRESSION  */
  SYMBOL_START_BOXED_EXPRESSION = 4,       /* START_BOXED_EXPRESSION  */
  SYMBOL_START_CONTEXT = 5,                /* START_CONTEXT  */
  SYMBOL_START_TEXTUAL_EXPRESSION = 6,     /* START_TEXTUAL_EXPRESSION  */
  SYMBOL_START_TEXTUAL_EXPRESSIONS = 7,    /* START_TEXTUAL_EXPRESSIONS  */
  SYMBOL_START_UNARY_TESTS = 8,            /* START_UNARY_TESTS  */
  SYMBOL_START_SIMPLE_EXPRESSION = 9,      /* START_SIMPLE_EXPRESSION  */
  SYMBOL_START_SIMPLE_EXPRESSIONS = 10,    /* START_SIMPLE_EXPRESSIONS  */
  SYMBOL_START_SIMPLE_VALUE = 11,          /* START_SIMPLE_VALUE  */
  SYMBOL_START_RANGE_LITERAL = 12,         /* START_RANGE_LITERAL  */
  SYMBOL_AT = 13,                          /* AT  */
  SYMBOL_NOT = 14,                         /* NOT  */
  SYMBOL_COLON = 15,                       /* COLON  */
  SYMBOL_COMMA = 16,                       /* COMMA  */
  SYMBOL_EVERY = 17,                       /* EVERY  */
  SYMBOL_FOR = 18,                         /* FOR  */
  SYMBOL_LEFT_BRACE = 19,                  /* LEFT_BRACE  */
  SYMBOL_NULL = 20,                        /* NULL  */
  SYMBOL_RIGHT_ARROW = 21,                 /* RIGHT_ARROW  */
  SYMBOL_OF = 22,                          /* OF  */
  SYMBOL_LIST = 23,                        /* LIST  */
  SYMBOL_RANGE = 24,                       /* RANGE  */
  SYMBOL_CONTEXT = 25,                     /* CONTEXT  */
  SYMBOL_THEN = 26,                        /* THEN  */
  SYMBOL_FUNCTION = 27,                    /* FUNCTION  */
  SYMBOL_EXTERNAL = 28,                    /* EXTERNAL  */
  SYMBOL_IF = 29,                          /* IF  */
  SYMBOL_RIGHT_BRACE = 30,                 /* RIGHT_BRACE  */
  SYMBOL_RETURN = 31,                      /* RETURN  */
  SYMBOL_SOME = 32,                        /* SOME  */
  SYMBOL_NUMERIC = 33,                     /* NUMERIC  */
  SYMBOL_STRING = 34,                      /* STRING  */
  SYMBOL_BOOLEAN = 35,                     /* BOOLEAN  */
  SYMBOL_SATISFIES = 36,                   /* SATISFIES  */
  SYMBOL_ELSE = 37,                        /* ELSE  */
  SYMBOL_OR = 38,                          /* OR  */
  SYMBOL_AND = 39,                         /* AND  */
  SYMBOL_EQ = 40,                          /* EQ  */
  SYMBOL_NE = 41,                          /* NE  */
  SYMBOL_LT = 42,                          /* LT  */
  SYMBOL_LE = 43,                          /* LE  */
  SYMBOL_GT = 44,                          /* GT  */
  SYMBOL_GE = 45,                          /* GE  */
  SYMBOL_BETWEEN = 46,                     /* BETWEEN  */
  SYMBOL_BETWEEN_AND = 47,                 /* BETWEEN_AND  */
  SYMBOL_IN = 48,                          /* IN  */
  SYMBOL_MINUS = 49,                       /* MINUS  */
  SYMBOL_PLUS = 50,                        /* PLUS  */
  SYMBOL_MUL = 51,                         /* MUL  */
  SYMBOL_DIV = 52,                         /* DIV  */
  SYMBOL_EXP = 53,                         /* EXP  */
  SYMBOL_PREC_NEG = 54,                    /* PREC_NEG  */
  SYMBOL_INSTANCE = 55,                    /* INSTANCE  */
  SYMBOL_NAME = 56,                        /* NAME  */
  SYMBOL_NAME_DATE_TIME = 57,              /* NAME_DATE_TIME  */
  SYMBOL_BUILT_IN_TYPE_NAME = 58,          /* BUILT_IN_TYPE_NAME  */
  SYMBOL_LEFT_PAREN = 59,                  /* LEFT_PAREN  */
  SYMBOL_RIGHT_PAREN = 60,                 /* RIGHT_PAREN  */
  SYMBOL_LEFT_BRACKET = 61,                /* LEFT_BRACKET  */
  SYMBOL_RIGHT_BRACKET = 62,               /* RIGHT_BRACKET  */
  SYMBOL_ELLIPSIS = 63,                    /* ELLIPSIS  */
  SYMBOL_DOT = 64,                         /* DOT  */
  SYMBOL_YYACCEPT = 65,                    /* $accept  */
  SYMBOL_feel = 66,                        /* feel  */
  SYMBOL_67_1 = 67,                        /* $@1  */
  SYMBOL_expression = 68,                  /* expression  */
  SYMBOL_boxed_expression = 69,            /* boxed_expression  */
  SYMBOL_textual_expression = 70,          /* textual_expression  */
  SYMBOL_71_2 = 71,                        /* $@2  */
  SYMBOL_72_3 = 72,                        /* $@3  */
  SYMBOL_73_4 = 73,                        /* $@4  */
  SYMBOL_74_5 = 74,                        /* $@5  */
  SYMBOL_75_6 = 75,                        /* $@6  */
  SYMBOL_textual_expressions = 76,         /* textual_expressions  */
  SYMBOL_unary_tests = 77,                 /* unary_tests  */
  SYMBOL_positive_unary_tests = 78,        /* positive_unary_tests  */
  SYMBOL_comparison_in = 79,               /* comparison_in  */
  SYMBOL_simple_expression = 80,           /* simple_expression  */
  SYMBOL_simple_expressions = 81,          /* simple_expressions  */
  SYMBOL_simple_positive_unary_test = 82,  /* simple_positive_unary_test  */
  SYMBOL_interval = 83,                    /* interval  */
  SYMBOL_interval_start = 84,              /* interval_start  */
  SYMBOL_interval_end = 85,                /* interval_end  */
  SYMBOL_endpoint = 86,                    /* endpoint  */
  SYMBOL_simple_value = 87,                /* simple_value  */
  SYMBOL_literal = 88,                     /* literal  */
  SYMBOL_simple_literal = 89,              /* simple_literal  */
  SYMBOL_90_7 = 90,                        /* $@7  */
  SYMBOL_context = 91,                     /* context  */
  SYMBOL_92_8 = 92,                        /* $@8  */
  SYMBOL_context_entries = 93,             /* context_entries  */
  SYMBOL_context_entry = 94,               /* context_entry  */
  SYMBOL_context_entry_tail = 95,          /* context_entry_tail  */
  SYMBOL_key = 96,                         /* key  */
  SYMBOL_list = 97,                        /* list  */
  SYMBOL_list_items = 98,                  /* list_items  */
  SYMBOL_list_tail = 99,                   /* list_tail  */
  SYMBOL_parameters = 100,                 /* parameters  */
  SYMBOL_named_parameters = 101,           /* named_parameters  */
  SYMBOL_named_parameter = 102,            /* named_parameter  */
  SYMBOL_named_parameters_tail = 103,      /* named_parameters_tail  */
  SYMBOL_positional_parameters = 104,      /* positional_parameters  */
  SYMBOL_positional_parameters_tail = 105, /* positional_parameters_tail  */
  SYMBOL_qualified_name = 106,             /* qualified_name  */
  SYMBOL_type = 107,                       /* type  */
  SYMBOL_108_9 = 108,                      /* $@9  */
  SYMBOL_109_10 = 109,                     /* $@10  */
  SYMBOL_110_11 = 110,                     /* $@11  */
  SYMBOL_context_type_entries = 111,       /* context_type_entries  */
  SYMBOL_context_type_entry = 112,         /* context_type_entry  */
  SYMBOL_113_12 = 113,                     /* $@12  */
  SYMBOL_context_type_entry_tail = 114,    /* context_type_entry_tail  */
  SYMBOL_function_type_parameters = 115,   /* function_type_parameters  */
  SYMBOL_116_13 = 116,                     /* $@13  */
  SYMBOL_function_type_parameters_tail = 117, /* function_type_parameters_tail  */
  SYMBOL_118_14 = 118,                     /* $@14  */
  SYMBOL_iteration_contexts = 119,         /* iteration_contexts  */
  SYMBOL_iteration_context = 120,          /* iteration_context  */
  SYMBOL_121_15 = 121,                     /* $@15  */
  SYMBOL_122_16 = 122,                     /* $@16  */
  SYMBOL_iteration_context_value = 123,    /* iteration_context_value  */
  SYMBOL_quantified_expressions = 124,     /* quantified_expressions  */
  SYMBOL_quantified_expression = 125,      /* quantified_expression  */
  SYMBOL_126_17 = 126,                     /* $@17  */
  SYMBOL_127_18 = 127,                     /* $@18  */
  SYMBOL_function_definition = 128,        /* function_definition  */
  SYMBOL_129_19 = 129,                     /* $@19  */
  SYMBOL_formal_parameters = 130,          /* formal_parameters  */
  SYMBOL_131_20 = 131,                     /* $@20  */
  SYMBOL_formal_parameters_tail = 132,     /* formal_parameters_tail  */
  SYMBOL_133_21 = 133,                     /* $@21  */
  SYMBOL_formal_parameter = 134,           /* formal_parameter  */
  SYMBOL_135_22 = 135,                     /* $@22  */
  SYMBOL_external = 136,                   /* external  */
  SYMBOL_range_literal = 137,              /* range_literal  */
  SYMBOL_range_literal_start = 138,        /* range_literal_start  */
  SYMBOL_range_literal_end = 139,          /* range_literal_end  */
  SYMBOL_range_endpoint = 140,             /* range_endpoint  */
  SYMBOL_141_23 = 141                      /* $@23  */
};
typedef enum yysymbol_kind_t yysymbol_kind_t;




#ifdef short
# undef short
#endif

/* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
   <limits.h> and (if available) <stdint.h> are included
   so that the code can choose integer types of a good width.  */

#ifndef __PTRDIFF_MAX__
# include <limits.h> /* INFRINGES ON USER NAME SPACE */
# if defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stdint.h> /* INFRINGES ON USER NAME SPACE */
#  define YY_STDINT_H
# endif
#endif

/* Narrow types that promote to a signed type and that can represent a
   signed or unsigned integer of at least N bits.  In tables they can
   save space and decrease cache pressure.  Promoting to a signed type
   helps avoid bugs in integer arithmetic.  */

#ifdef __INT_LEAST8_MAX__
typedef __INT_LEAST8_TYPE__ yytype_int8;
#elif defined YY_STDINT_H
typedef int_least8_t yytype_int8;
#else
typedef signed char yytype_int8;
#endif

#ifdef __INT_LEAST16_MAX__
typedef __INT_LEAST16_TYPE__ yytype_int16;
#elif defined YY_STDINT_H
typedef int_least16_t yytype_int16;
#else
typedef short yytype_int16;
#endif

/* Work around bug in HP-UX 11.23, which defines these macros
   incorrectly for preprocessor constants.  This workaround can likely
   be removed in 2023, as HPE has promised support for HP-UX 11.23
   (aka HP-UX 11i v2) only through the end of 2022; see Table 2 of
   <https://h20195.www2.hpe.com/V2/getpdf.aspx/4AA4-7673ENW.pdf>.  */
#ifdef __hpux
# undef UINT_LEAST8_MAX
# undef UINT_LEAST16_MAX
# define UINT_LEAST8_MAX 255
# define UINT_LEAST16_MAX 65535
#endif

#if defined __UINT_LEAST8_MAX__ && __UINT_LEAST8_MAX__ <= __INT_MAX__
typedef __UINT_LEAST8_TYPE__ yytype_uint8;
#elif (!defined __UINT_LEAST8_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST8_MAX <= INT_MAX)
typedef uint_least8_t yytype_uint8;
#elif !defined __UINT_LEAST8_MAX__ && UCHAR_MAX <= INT_MAX
typedef unsigned char yytype_uint8;
#else
typedef short yytype_uint8;
#endif

#if defined __UINT_LEAST16_MAX__ && __UINT_LEAST16_MAX__ <= __INT_MAX__
typedef __UINT_LEAST16_TYPE__ yytype_uint16;
#elif (!defined __UINT_LEAST16_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST16_MAX <= INT_MAX)
typedef uint_least16_t yytype_uint16;
#elif !defined __UINT_LEAST16_MAX__ && USHRT_MAX <= INT_MAX
typedef unsigned short yytype_uint16;
#else
typedef int yytype_uint16;
#endif

#ifndef YYPTRDIFF_T
# if defined __PTRDIFF_TYPE__ && defined __PTRDIFF_MAX__
#  define YYPTRDIFF_T __PTRDIFF_TYPE__
#  define YYPTRDIFF_MAXIMUM __PTRDIFF_MAX__
# elif defined PTRDIFF_MAX
#  ifndef ptrdiff_t
#   include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  endif
#  define YYPTRDIFF_T ptrdiff_t
#  define YYPTRDIFF_MAXIMUM PTRDIFF_MAX
# else
#  define YYPTRDIFF_T long
#  define YYPTRDIFF_MAXIMUM LONG_MAX
# endif
#endif

#ifndef YYSIZE_T
# ifdef __SIZE_TYPE__
#  define YYSIZE_T __SIZE_TYPE__
# elif defined size_t
#  define YYSIZE_T size_t
# elif defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  define YYSIZE_T size_t
# else
#  define YYSIZE_T unsigned
# endif
#endif

#define YYSIZE_MAXIMUM                                  \
  YY_CAST (YYPTRDIFF_T,                                 \
           (YYPTRDIFF_MAXIMUM < YY_CAST (YYSIZE_T, -1)  \
            ? YYPTRDIFF_MAXIMUM                         \
            : YY_CAST (YYSIZE_T, -1)))

#define YYSIZEOF(X) YY_CAST (YYPTRDIFF_T, sizeof (X))


/* Stored state numbers (used for stacks). */
typedef yytype_int16 yy_state_t;

/* State numbers in computations.  */
typedef int yy_state_fast_t;

#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> /* INFRINGES ON USER NAME SPACE */
#   define YY_(Msgid) dgettext ("bison-runtime", Msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(Msgid) Msgid
# endif
#endif


#ifndef YY_ATTRIBUTE_PURE
# if defined __GNUC__ && 2 < __GNUC__ + (96 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_PURE __attribute__ ((__pure__))
# else
#  define YY_ATTRIBUTE_PURE
# endif
#endif

#ifndef YY_ATTRIBUTE_UNUSED
# if defined __GNUC__ && 2 < __GNUC__ + (7 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_UNUSED __attribute__ ((__unused__))
# else
#  define YY_ATTRIBUTE_UNUSED
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YY_USE(E) ((void) (E))
#else
# define YY_USE(E) /* empty */
#endif

#if defined __GNUC__ && ! defined __ICC && 407 <= __GNUC__ * 100 + __GNUC_MINOR__
/* Suppress an incorrect diagnostic about yylval being uninitialized.  */
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                            \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")              \
    _Pragma ("GCC diagnostic ignored \"-Wmaybe-uninitialized\"")
# define YY_IGNORE_MAYBE_UNINITIALIZED_END      \
    _Pragma ("GCC diagnostic pop")
#else
# define YY_INITIAL_VALUE(Value) Value
#endif
#ifndef YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_END
#endif
#ifndef YY_INITIAL_VALUE
# define YY_INITIAL_VALUE(Value) /* Nothing. */
#endif

#if defined __cplusplus && defined __GNUC__ && ! defined __ICC && 6 <= __GNUC__
# define YY_IGNORE_USELESS_CAST_BEGIN                          \
    _Pragma ("GCC diagnostic push")                            \
    _Pragma ("GCC diagnostic ignored \"-Wuseless-cast\"")
# define YY_IGNORE_USELESS_CAST_END            \
    _Pragma ("GCC diagnostic pop")
#endif
#ifndef YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_END
#endif


#define YY_ASSERT(E) ((void) (0 && (E)))

#if !defined yyoverflow

/* The parser invokes alloca or malloc; define the necessary symbols.  */

# ifdef YYSTACK_USE_ALLOCA
#  if YYSTACK_USE_ALLOCA
#   ifdef __GNUC__
#    define YYSTACK_ALLOC __builtin_alloca
#   elif defined __BUILTIN_VA_ARG_INCR
#    include <alloca.h> /* INFRINGES ON USER NAME SPACE */
#   elif defined _AIX
#    define YYSTACK_ALLOC __alloca
#   elif defined _MSC_VER
#    include <malloc.h> /* INFRINGES ON USER NAME SPACE */
#    define alloca _alloca
#   else
#    define YYSTACK_ALLOC alloca
#    if ! defined _ALLOCA_H && ! defined EXIT_SUCCESS
#     include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
      /* Use EXIT_SUCCESS as a witness for stdlib.h.  */
#     ifndef EXIT_SUCCESS
#      define EXIT_SUCCESS 0
#     endif
#    endif
#   endif
#  endif
# endif

# ifdef YYSTACK_ALLOC
   /* Pacify GCC's 'empty if-body' warning.  */
#  define YYSTACK_FREE(Ptr) do { /* empty */; } while (0)
#  ifndef YYSTACK_ALLOC_MAXIMUM
    /* The OS might guarantee only one guard page at the bottom of the stack,
       and a page size can be as small as 4096 bytes.  So we cannot safely
       invoke alloca (N) if N exceeds 4096.  Use a slightly smaller number
       to allow for a few compiler-allocated temporary stack slots.  */
#   define YYSTACK_ALLOC_MAXIMUM 4032 /* reasonable circa 2006 */
#  endif
# else
#  define YYSTACK_ALLOC YYMALLOC
#  define YYSTACK_FREE YYFREE
#  ifndef YYSTACK_ALLOC_MAXIMUM
#   define YYSTACK_ALLOC_MAXIMUM YYSIZE_MAXIMUM
#  endif
#  if (defined __cplusplus && ! defined EXIT_SUCCESS \
       && ! ((defined YYMALLOC || defined malloc) \
             && (defined YYFREE || defined free)))
#   include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
#   ifndef EXIT_SUCCESS
#    define EXIT_SUCCESS 0
#   endif
#  endif
#  ifndef YYMALLOC
#   define YYMALLOC malloc
#   if ! defined malloc && ! defined EXIT_SUCCESS
void *malloc (YYSIZE_T); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
#  ifndef YYFREE
#   define YYFREE free
#   if ! defined free && ! defined EXIT_SUCCESS
void free (void *); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
# endif
#endif /* !defined yyoverflow */

#if (! defined yyoverflow \
     && (! defined __cplusplus \
         || (defined YYSTYPE_IS_TRIVIAL && YYSTYPE_IS_TRIVIAL)))

/* A type that is properly aligned for any stack member.  */
union yyalloc
{
  yy_state_t yyss_alloc;
  YYSTYPE yyvs_alloc;
};

/* The size of the maximum gap between one aligned stack and the next.  */
# define YYSTACK_GAP_MAXIMUM (YYSIZEOF (union yyalloc) - 1)

/* The size of an array large to enough to hold all stacks, each with
   N elements.  */
# define YYSTACK_BYTES(N) \
     ((N) * (YYSIZEOF (yy_state_t) + YYSIZEOF (YYSTYPE)) \
      + YYSTACK_GAP_MAXIMUM)

# define YYCOPY_NEEDED 1

/* Relocate STACK from its old location to the new one.  The
   local variables YYSIZE and YYSTACKSIZE give the old and new number of
   elements in the stack, and YYPTR gives the new location of the
   stack.  Advance YYPTR to a properly aligned location for the next
   stack.  */
# define YYSTACK_RELOCATE(Stack_alloc, Stack)                           \
    do                                                                  \
      {                                                                 \
        YYPTRDIFF_T yynewbytes;                                         \
        YYCOPY (&yyptr->Stack_alloc, Stack, yysize);                    \
        Stack = &yyptr->Stack_alloc;                                    \
        yynewbytes = yystacksize * YYSIZEOF (*Stack) + YYSTACK_GAP_MAXIMUM; \
        yyptr += yynewbytes / YYSIZEOF (*yyptr);                        \
      }                                                                 \
    while (0)

#endif

#if defined YYCOPY_NEEDED && YYCOPY_NEEDED
/* Copy COUNT objects from SRC to DST.  The source and destination do
   not overlap.  */
# ifndef YYCOPY
#  if defined __GNUC__ && 1 < __GNUC__
#   define YYCOPY(Dst, Src, Count) \
      __builtin_memcpy (Dst, Src, YY_CAST (YYSIZE_T, (Count)) * sizeof (*(Src)))
#  else
#   define YYCOPY(Dst, Src, Count)              \
      do                                        \
        {                                       \
          YYPTRDIFF_T yyi;                      \
          for (yyi = 0; yyi < (Count); yyi++)   \
            (Dst)[yyi] = (Src)[yyi];            \
        }                                       \
      while (0)
#  endif
# endif
#endif /* !YYCOPY_NEEDED */

/* YYFINAL -- State number of the termination state.  */
#define YYFINAL  71
/* YYLAST -- Last index in YYTABLE.  */
#define YYLAST   1060

/* YYNTOKENS -- Number of terminals.  */
#define YYNTOKENS  65
/* YYNNTS -- Number of nonterminals.  */
#define YYNNTS  77
/* YYNRULES -- Number of rules.  */
#define YYNRULES  175
/* YYNSTATES -- Number of states.  */
#define YYNSTATES  329

/* YYMAXUTOK -- Last valid token kind.  */
#define YYMAXUTOK   319


/* YYTRANSLATE(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex, with out-of-bounds checking.  */
#define YYTRANSLATE(YYX)                                \
  (0 <= (YYX) && (YYX) <= YYMAXUTOK                     \
   ? YY_CAST (yysymbol_kind_t, yytranslate[YYX])        \
   : SYMBOL_YYUNDEF)

/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
static const yytype_int8 yytranslate[] =
{
       0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26,    27,    28,    29,    30,    31,    32,    33,    34,
      35,    36,    37,    38,    39,    40,    41,    42,    43,    44,
      45,    46,    47,    48,    49,    50,    51,    52,    53,    54,
      55,    56,    57,    58,    59,    60,    61,    62,    63,    64
};

#if YYDEBUG
  /* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
static const yytype_int16 yyrline[] =
{
       0,    62,    62,    63,    64,    65,    66,    67,    67,    68,
      69,    70,    71,    75,    76,    80,    81,    82,    86,    86,
      87,    88,    88,    89,    89,    90,    90,    91,    92,    93,
      94,    95,    96,    97,    98,    99,   100,   101,   102,   103,
     104,   105,   106,   107,   107,   108,   109,   110,   111,   112,
     113,   114,   118,   119,   123,   124,   125,   129,   130,   134,
     138,   139,   140,   141,   142,   143,   144,   148,   149,   153,
     154,   155,   156,   157,   158,   159,   163,   167,   168,   169,
     173,   174,   175,   179,   183,   184,   188,   189,   193,   194,
     195,   196,   197,   197,   201,   201,   205,   206,   210,   214,
     215,   219,   220,   224,   228,   229,   233,   234,   238,   239,
     240,   244,   248,   252,   253,   257,   261,   262,   266,   267,
     271,   272,   273,   273,   274,   274,   275,   276,   276,   280,
     284,   284,   288,   289,   293,   294,   294,   298,   299,   299,
     303,   304,   308,   308,   308,   312,   313,   317,   318,   322,
     322,   322,   326,   326,   330,   331,   331,   335,   336,   336,
     340,   340,   341,   345,   346,   350,   354,   355,   356,   360,
     361,   362,   366,   367,   368,   368
};
#endif

/** Accessing symbol of state STATE.  */
#define YY_ACCESSING_SYMBOL(State) YY_CAST (yysymbol_kind_t, yystos[State])

#if YYDEBUG || 0
/* The user-facing name of the symbol whose (internal) number is
   YYSYMBOL.  No bounds checking.  */
static const char *yysymbol_name (yysymbol_kind_t yysymbol) YY_ATTRIBUTE_UNUSED;

/* YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
   First, the terminals, then, starting at YYNTOKENS, nonterminals.  */
static const char *const yytname[] =
{
  "\"end of file\"", "error", "\"invalid token\"", "START_EXPRESSION",
  "START_BOXED_EXPRESSION", "START_CONTEXT", "START_TEXTUAL_EXPRESSION",
  "START_TEXTUAL_EXPRESSIONS", "START_UNARY_TESTS",
  "START_SIMPLE_EXPRESSION", "START_SIMPLE_EXPRESSIONS",
  "START_SIMPLE_VALUE", "START_RANGE_LITERAL", "AT", "NOT", "COLON",
  "COMMA", "EVERY", "FOR", "LEFT_BRACE", "NULL", "RIGHT_ARROW", "OF",
  "LIST", "RANGE", "CONTEXT", "THEN", "FUNCTION", "EXTERNAL", "IF",
  "RIGHT_BRACE", "RETURN", "SOME", "NUMERIC", "STRING", "BOOLEAN",
  "SATISFIES", "ELSE", "OR", "AND", "EQ", "NE", "LT", "LE", "GT", "GE",
  "BETWEEN", "BETWEEN_AND", "IN", "MINUS", "PLUS", "MUL", "DIV", "EXP",
  "PREC_NEG", "INSTANCE", "NAME", "NAME_DATE_TIME", "BUILT_IN_TYPE_NAME",
  "LEFT_PAREN", "RIGHT_PAREN", "LEFT_BRACKET", "RIGHT_BRACKET", "ELLIPSIS",
  "DOT", "$accept", "feel", "$@1", "expression", "boxed_expression",
  "textual_expression", "$@2", "$@3", "$@4", "$@5", "$@6",
  "textual_expressions", "unary_tests", "positive_unary_tests",
  "comparison_in", "simple_expression", "simple_expressions",
  "simple_positive_unary_test", "interval", "interval_start",
  "interval_end", "endpoint", "simple_value", "literal", "simple_literal",
  "$@7", "context", "$@8", "context_entries", "context_entry",
  "context_entry_tail", "key", "list", "list_items", "list_tail",
  "parameters", "named_parameters", "named_parameter",
  "named_parameters_tail", "positional_parameters",
  "positional_parameters_tail", "qualified_name", "type", "$@9", "$@10",
  "$@11", "context_type_entries", "context_type_entry", "$@12",
  "context_type_entry_tail", "function_type_parameters", "$@13",
  "function_type_parameters_tail", "$@14", "iteration_contexts",
  "iteration_context", "$@15", "$@16", "iteration_context_value",
  "quantified_expressions", "quantified_expression", "$@17", "$@18",
  "function_definition", "$@19", "formal_parameters", "$@20",
  "formal_parameters_tail", "$@21", "formal_parameter", "$@22", "external",
  "range_literal", "range_literal_start", "range_literal_end",
  "range_endpoint", "$@23", YY_NULLPTR
};

static const char *
yysymbol_name (yysymbol_kind_t yysymbol)
{
  return yytname[yysymbol];
}
#endif

#ifdef YYPRINT
/* YYTOKNUM[NUM] -- (External) token number corresponding to the
   (internal) symbol number NUM (which must be that of a token).  */
static const yytype_int16 yytoknum[] =
{
       0,   256,   257,   258,   259,   260,   261,   262,   263,   264,
     265,   266,   267,   268,   269,   270,   271,   272,   273,   274,
     275,   276,   277,   278,   279,   280,   281,   282,   283,   284,
     285,   286,   287,   288,   289,   290,   291,   292,   293,   294,
     295,   296,   297,   298,   299,   300,   301,   302,   303,   304,
     305,   306,   307,   308,   309,   310,   311,   312,   313,   314,
     315,   316,   317,   318,   319
};
#endif

#define YYPACT_NINF (-260)

#define yypact_value_is_default(Yyn) \
  ((Yyn) == YYPACT_NINF)

#define YYTABLE_NINF (-120)

#define yytable_value_is_error(Yyn) \
  ((Yyn) == YYTABLE_NINF)

  /* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
static const yytype_int16 yypact[] =
{
    1048,   391,     3,     4,   391,   391,  -260,   438,   438,   130,
      53,    29,     7,  -260,  -260,  -260,  -260,     2,   391,  -260,
    -260,  -260,  -260,   391,   391,   391,   391,   391,   391,   391,
    -260,    25,   391,   485,   391,   866,  -260,  -260,  -260,  -260,
     391,  -260,  -260,  -260,  -260,  -260,   485,  -260,  -260,   866,
      65,    59,  -260,   247,   391,    18,   893,  -260,  -260,    73,
    -260,    75,  -260,    28,  -260,  -260,     5,     5,     5,  -260,
       5,  -260,  -260,  -260,  -260,    10,  -260,   703,  -260,    58,
    -260,  -260,  -260,  -260,  -260,  -260,   173,  -260,   758,    60,
     704,   562,    77,  -260,    81,   391,   391,   391,   391,   391,
     391,   391,   391,  -260,   532,   391,   391,   391,   391,   391,
     127,   294,   391,    99,  -260,   106,   562,   391,   102,   391,
     655,  -260,  -260,   119,   121,   391,   391,   391,   391,   391,
     438,  -260,  -260,   117,   122,   131,   132,  -260,   128,   146,
     177,   140,   174,   188,   152,  -260,  -260,  -260,  -260,   104,
     197,    61,   391,   178,   294,  -260,  -260,   391,  -260,  -260,
    -260,  -260,   919,   944,   969,   969,   969,   969,   969,   969,
     391,   391,   986,   120,   120,   156,   156,   173,  -260,   198,
    -260,   593,  -260,  -260,     8,  -260,   785,  -260,  -260,  -260,
    -260,  -260,   391,   391,  -260,    19,    86,    26,    93,   142,
    -260,  -260,  -260,  -260,  -260,  -260,  -260,  -260,   391,  -260,
    -260,   391,  -260,  -260,    20,  -260,  -260,   391,   203,  -260,
     344,  -260,   731,   391,  -260,   562,   812,   624,  -260,   175,
     391,   391,  -260,  -260,   165,  -260,  -260,  -260,   163,  -260,
     294,   866,  -260,   176,   866,  -260,   179,   104,   866,  -260,
     391,   866,  -260,     9,   391,   866,  -260,   391,   391,   183,
     184,   193,   194,  -260,  -260,  -260,   866,   593,   198,     8,
    -260,  -260,   391,   391,  -260,   175,   866,   182,  -260,  -260,
     866,   986,   180,  -260,  -260,   185,   199,  -260,  -260,   866,
     839,  -260,  -260,  -260,  -260,   175,   175,   227,  -260,   -11,
    -260,   223,   175,   391,     9,   201,   202,  -260,   185,  -260,
    -260,  -260,    11,   866,  -260,  -260,  -260,   175,   -11,   175,
    -260,  -260,  -260,  -260,  -260,  -260,   175,    11,  -260
};

  /* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
static const yytype_uint8 yydefact[] =
{
       0,     0,     0,     0,     0,     0,     7,     0,     0,     0,
       0,     0,     0,    23,    18,    94,    87,     0,     0,    21,
      88,    89,    90,     0,     0,     0,     0,     0,     0,     0,
      50,     0,     0,     0,     0,     2,    13,    14,    49,    75,
       0,    48,    86,    17,    15,    16,     0,     3,     4,     0,
      14,    14,     6,     0,     0,    50,     0,     9,    66,    86,
      84,    68,    10,   119,    11,    85,     0,     0,     0,    12,
       0,     1,    91,   149,   142,     0,   152,     0,   149,    83,
      73,    74,    69,    70,    71,    72,    42,    92,    83,     0,
     104,    83,     0,   103,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    25,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    76,     0,     0,     0,     0,    54,
      58,     8,    56,    42,     0,     0,     0,     0,     0,     0,
       0,   172,   173,     0,     0,     0,     0,   165,     0,     0,
     148,     0,     0,   141,     0,    96,   102,   101,    95,     0,
       0,     0,     0,     0,     0,    51,    77,     0,   106,   105,
      79,    78,    27,    28,    29,    30,    31,    32,    33,    34,
       0,     0,    36,    38,    37,    39,    40,    41,    43,    50,
     108,     0,    47,   109,     0,   110,     0,    45,    80,    81,
      82,    52,     0,     0,   118,    38,    37,    39,    40,    41,
      67,   174,   166,   168,   167,   169,   170,   171,     0,   149,
     150,     0,   142,   143,     0,    99,    97,     0,   162,   154,
       0,   155,     0,     0,    93,     0,     0,    83,    35,     0,
       0,     0,   116,   115,     0,   113,   111,    46,     0,    57,
       0,    24,   147,     0,    19,   140,     0,     0,    98,   160,
       0,   164,   153,     0,     0,    22,   107,     0,     0,     0,
       0,     0,     0,   120,   121,    44,   112,     0,     0,     0,
      55,   175,     0,     0,   100,     0,   163,     0,   157,   156,
      20,    26,     0,   122,   124,     0,   135,   117,   114,   151,
     145,   144,   161,   158,    59,     0,     0,     0,   126,     0,
     134,     0,     0,     0,     0,     0,     0,   130,     0,   132,
     129,   127,     0,   146,   159,   123,   125,     0,     0,     0,
     138,   137,   136,   131,   133,   128,     0,     0,   139
};

  /* YYPGOTO[NTERM-NUM].  */
static const yytype_int16 yypgoto[] =
{
    -260,  -260,  -260,    -1,   245,    16,  -260,  -260,  -260,  -260,
    -260,   133,  -260,  -181,  -260,   241,   124,  -260,  -260,  -260,
    -260,    23,   242,  -260,     1,  -260,   249,  -260,  -260,    41,
      12,  -260,  -260,  -260,    33,  -152,  -260,    34,    -7,  -260,
      -4,     6,  -259,  -260,  -260,  -260,  -260,   -39,  -260,   -48,
    -260,  -260,   -54,  -260,    63,  -260,  -260,  -260,  -260,   -77,
    -260,  -260,  -260,  -260,  -260,  -260,  -260,   -27,  -260,    17,
    -260,  -260,  -260,  -260,  -260,    92,  -260
};

  /* YYDEFGOTO[NTERM-NUM].  */
static const yytype_int16 yydefgoto[] =
{
       0,    11,    53,    79,    36,    37,    74,    78,    73,   170,
     229,    52,   121,   122,   228,    61,    62,    38,    39,    40,
     114,    89,    58,    41,    42,   154,    43,    75,   148,   149,
     216,   150,    44,    93,   159,   182,   183,   184,   236,   185,
     233,   264,   265,   295,   296,   319,   298,   299,   317,   310,
     301,   302,   322,   326,   142,   143,   144,   246,   291,   139,
     140,   141,   243,    45,   151,   220,   253,   279,   304,   221,
     275,   252,    69,    70,   137,   134,   240
};

  /* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
static const yytype_int16 yytable[] =
{
      35,   153,   224,    49,    49,   308,    56,    56,    59,    59,
      65,   238,   239,    60,    60,    60,   292,    77,  -119,   -61,
      50,    51,    15,    15,   234,   277,   -62,   320,    86,    71,
      17,    88,    91,   309,  -119,   -61,   305,   306,   131,   132,
     145,    72,   -62,   312,   146,   116,    80,    81,    82,    83,
      84,    85,   120,   123,   146,   321,    92,    94,   323,   -53,
     325,    76,   133,   115,    46,    -5,   147,   327,   235,   278,
     107,   108,   109,   -85,   110,   117,   147,   282,   111,   109,
     112,   110,   124,   113,    87,   111,   -60,   112,   271,   -85,
     113,   130,   124,   -63,   162,   163,   164,   165,   166,   167,
     168,   169,   -60,   172,   173,   174,   175,   176,   177,   -63,
     181,   186,    66,    94,    67,    68,    49,   218,    86,   -65,
     214,   219,   113,   156,   195,   196,   197,   198,   199,    56,
     194,    59,   242,    51,   215,   -65,    60,   107,   108,   109,
     160,   110,   -64,    12,   161,   111,   109,   112,   110,   178,
     113,   222,   111,   181,   112,   187,   225,   113,   -64,   135,
     136,   192,   138,    20,    21,    22,   188,   189,   190,   226,
     227,   107,   108,   109,   110,   110,   201,    63,   111,   111,
     112,   112,   208,   113,   113,   202,    63,    31,   205,   206,
     207,   120,   120,   209,   203,   204,   210,   110,   259,   260,
     261,   111,   262,   112,   212,   211,   113,   241,   213,   109,
     244,   110,   217,   230,   223,   111,   248,   112,   249,   251,
     113,   268,   255,   270,   272,   283,   284,   273,   110,   266,
     267,    63,   111,   263,   112,   285,   286,   113,   218,   181,
     294,   297,   307,   300,   311,   315,   316,    47,    57,   276,
     191,    64,    48,   280,   200,   247,   281,   120,   256,   274,
      12,   118,   288,   287,    13,    14,    15,    16,   269,   318,
     324,   289,   290,   328,    17,   245,    18,   314,     0,    19,
      20,    21,    22,     0,     0,     0,     0,    23,    24,    25,
      26,    27,    28,     0,   293,     0,   119,     0,     0,     0,
       0,     0,   313,    30,    31,     0,    32,    12,    33,    34,
       0,    13,    14,    15,    16,     0,     0,     0,     0,     0,
       0,    17,     0,    18,     0,     0,    19,    20,    21,    22,
       0,     0,     0,     0,    23,    24,    25,    26,    27,    28,
       0,     0,     0,    29,     0,     0,     0,     0,     0,     0,
     179,    31,     0,    32,   180,    33,    34,    12,     0,     0,
       0,    13,    14,    15,    16,     0,     0,     0,     0,     0,
       0,    17,   250,    18,     0,     0,    19,    20,    21,    22,
       0,     0,     0,     0,    23,    24,    25,    26,    27,    28,
       0,     0,     0,    29,     0,     0,     0,     0,     0,     0,
      30,    31,     0,    32,    12,    33,    34,     0,    13,    14,
      15,    16,     0,     0,     0,     0,     0,     0,    17,     0,
      18,     0,     0,    19,    20,    21,    22,     0,     0,     0,
       0,    23,    24,    25,    26,    27,    28,     0,     0,     0,
      29,     0,     0,     0,     0,     0,     0,    30,    31,     0,
      32,    12,    33,    34,     0,    13,    14,    15,    16,     0,
       0,     0,     0,     0,     0,    17,     0,    18,     0,     0,
      19,    20,    21,    22,     0,     0,     0,     0,    23,    24,
      25,    26,    27,    28,     0,     0,     0,    54,     0,     0,
       0,     0,     0,     0,    55,    31,     0,    32,    12,    33,
      34,     0,    13,    14,    15,    16,     0,     0,     0,     0,
       0,     0,    17,     0,    18,     0,     0,    19,    20,    21,
      22,     0,     0,     0,     0,    23,    24,    25,    26,    27,
      28,     0,     0,     0,    29,     0,     0,     0,     0,     0,
       0,    30,    31,     0,    32,    12,    33,    90,     0,    13,
      14,    15,    16,     0,     0,     0,     0,     0,     0,    17,
       0,    18,     0,     0,    19,    20,    21,    22,     0,     0,
       0,     0,    23,    24,    25,    26,    27,    28,   157,     0,
       0,    29,     0,     0,     0,     0,     0,     0,    30,    31,
       0,   171,     0,    33,    34,     0,     0,     0,     0,     0,
      95,    96,    97,    98,    99,   100,   101,   102,   103,   231,
     104,   105,   106,   107,   108,   109,     0,   110,     0,     0,
       0,   111,     0,   112,   158,     0,   113,     0,     0,     0,
       0,    95,    96,    97,    98,    99,   100,   101,   102,   103,
     258,   104,   105,   106,   107,   108,   109,     0,   110,     0,
       0,     0,   111,   232,   112,     0,     0,   113,     0,     0,
       0,     0,    95,    96,    97,    98,    99,   100,   101,   102,
     103,   193,   104,   105,   106,   107,   108,   109,     0,   110,
       0,     0,     0,   111,   155,   112,     0,     0,   113,     0,
       0,     0,     0,    95,    96,    97,    98,    99,   100,   101,
     102,   103,     0,   104,   105,   106,   107,   108,   109,     0,
     110,     0,     0,     0,   111,     0,   112,    12,     0,   113,
       0,    13,    14,    15,    16,     0,     0,     0,     0,   152,
       0,    17,     0,    18,     0,     0,    19,    20,    21,    22,
       0,    95,    96,    97,    98,    99,   100,   101,   102,   103,
       0,   104,   105,   106,   107,   108,   109,     0,   110,     0,
      30,    31,   111,     0,   112,     0,     0,   113,   254,    95,
      96,    97,    98,    99,   100,   101,   102,   103,     0,   104,
     105,   106,   107,   108,   109,     0,   110,     0,     0,     0,
     111,     0,   112,     0,     0,   113,    95,    96,    97,    98,
      99,   100,   101,   102,   103,     0,   104,   105,   106,   107,
     108,   109,     0,   110,     0,     0,     0,   111,   155,   112,
       0,     0,   113,    95,    96,    97,    98,    99,   100,   101,
     102,   103,     0,   104,   105,   106,   107,   108,   109,     0,
     110,     0,     0,     0,   111,     0,   112,   237,     0,   113,
      95,    96,    97,    98,    99,   100,   101,   102,   103,   257,
     104,   105,   106,   107,   108,   109,     0,   110,     0,     0,
       0,   111,     0,   112,     0,     0,   113,    95,    96,    97,
      98,    99,   100,   101,   102,   103,     0,   104,   105,   106,
     107,   108,   109,     0,   110,     0,     0,     0,   111,     0,
     112,     0,   303,   113,    95,    96,    97,    98,    99,   100,
     101,   102,   103,     0,   104,   105,   106,   107,   108,   109,
       0,   110,     0,     0,     0,   111,     0,   112,     0,     0,
     113,    95,    96,    97,    98,    99,   100,   101,   102,   103,
       0,   104,   125,   126,   127,   128,   129,     0,   110,     0,
       0,     0,   111,     0,   112,     0,     0,   113,    96,    97,
      98,    99,   100,   101,   102,   103,     0,   104,   105,   106,
     107,   108,   109,     0,   110,     0,     0,     0,   111,     0,
     112,     0,     0,   113,    97,    98,    99,   100,   101,   102,
     103,     0,   104,   105,   106,   107,   108,   109,     0,   110,
       0,     0,     0,   111,     0,   112,     0,     0,   113,  -120,
    -120,  -120,  -120,  -120,  -120,   103,     0,   104,   105,   106,
     107,   108,   109,     0,   110,     0,     0,     0,   111,     0,
     112,     0,     0,   113,   104,   105,   106,   107,   108,   109,
       0,   110,     0,     0,     0,   111,     0,   112,     0,     0,
     113,     1,     2,     3,     4,     5,     6,     7,     8,     9,
      10
};

static const yytype_int16 yycheck[] =
{
       1,    78,   154,     4,     5,    16,     7,     8,     7,     8,
       9,   192,   193,     7,     8,     9,   275,    18,     0,     0,
       4,     5,    19,    19,    16,    16,     0,    16,    29,     0,
      27,    32,    33,    44,    16,    16,   295,   296,    33,    34,
      30,    34,    16,   302,    34,    46,    23,    24,    25,    26,
      27,    28,    53,    54,    34,    44,    33,    34,   317,     0,
     319,    59,    57,    40,    61,     0,    56,   326,    60,    60,
      51,    52,    53,     0,    55,    16,    56,   258,    59,    53,
      61,    55,    64,    64,    59,    59,     0,    61,   240,    16,
      64,    16,    64,     0,    95,    96,    97,    98,    99,   100,
     101,   102,    16,   104,   105,   106,   107,   108,   109,    16,
     111,   112,    59,    90,    61,    62,   117,    56,   119,     0,
      16,    60,    64,    63,   125,   126,   127,   128,   129,   130,
     124,   130,   209,   117,    30,    16,   130,    51,    52,    53,
      63,    55,     0,    13,    63,    59,    53,    61,    55,    22,
      64,   152,    59,   154,    61,    56,   157,    64,    16,    67,
      68,    59,    70,    33,    34,    35,    60,    61,    62,   170,
     171,    51,    52,    53,    55,    55,    59,    56,    59,    59,
      61,    61,    36,    64,    64,    63,    56,    57,    60,    61,
      62,   192,   193,    16,    63,    63,    56,    55,    23,    24,
      25,    59,    27,    61,    16,    31,    64,   208,    56,    53,
     211,    55,    15,    15,    36,    59,   217,    61,    15,   220,
      64,    56,   223,    60,    48,    42,    42,    48,    55,   230,
     231,    56,    59,    58,    61,    42,    42,    64,    56,   240,
      60,    56,    15,    44,    21,    44,    44,     2,     7,   250,
     117,     9,     3,   254,   130,   214,   257,   258,   225,   247,
      13,    14,   269,   267,    17,    18,    19,    20,   234,   308,
     318,   272,   273,   327,    27,   212,    29,   304,    -1,    32,
      33,    34,    35,    -1,    -1,    -1,    -1,    40,    41,    42,
      43,    44,    45,    -1,   277,    -1,    49,    -1,    -1,    -1,
      -1,    -1,   303,    56,    57,    -1,    59,    13,    61,    62,
      -1,    17,    18,    19,    20,    -1,    -1,    -1,    -1,    -1,
      -1,    27,    -1,    29,    -1,    -1,    32,    33,    34,    35,
      -1,    -1,    -1,    -1,    40,    41,    42,    43,    44,    45,
      -1,    -1,    -1,    49,    -1,    -1,    -1,    -1,    -1,    -1,
      56,    57,    -1,    59,    60,    61,    62,    13,    -1,    -1,
      -1,    17,    18,    19,    20,    -1,    -1,    -1,    -1,    -1,
      -1,    27,    28,    29,    -1,    -1,    32,    33,    34,    35,
      -1,    -1,    -1,    -1,    40,    41,    42,    43,    44,    45,
      -1,    -1,    -1,    49,    -1,    -1,    -1,    -1,    -1,    -1,
      56,    57,    -1,    59,    13,    61,    62,    -1,    17,    18,
      19,    20,    -1,    -1,    -1,    -1,    -1,    -1,    27,    -1,
      29,    -1,    -1,    32,    33,    34,    35,    -1,    -1,    -1,
      -1,    40,    41,    42,    43,    44,    45,    -1,    -1,    -1,
      49,    -1,    -1,    -1,    -1,    -1,    -1,    56,    57,    -1,
      59,    13,    61,    62,    -1,    17,    18,    19,    20,    -1,
      -1,    -1,    -1,    -1,    -1,    27,    -1,    29,    -1,    -1,
      32,    33,    34,    35,    -1,    -1,    -1,    -1,    40,    41,
      42,    43,    44,    45,    -1,    -1,    -1,    49,    -1,    -1,
      -1,    -1,    -1,    -1,    56,    57,    -1,    59,    13,    61,
      62,    -1,    17,    18,    19,    20,    -1,    -1,    -1,    -1,
      -1,    -1,    27,    -1,    29,    -1,    -1,    32,    33,    34,
      35,    -1,    -1,    -1,    -1,    40,    41,    42,    43,    44,
      45,    -1,    -1,    -1,    49,    -1,    -1,    -1,    -1,    -1,
      -1,    56,    57,    -1,    59,    13,    61,    62,    -1,    17,
      18,    19,    20,    -1,    -1,    -1,    -1,    -1,    -1,    27,
      -1,    29,    -1,    -1,    32,    33,    34,    35,    -1,    -1,
      -1,    -1,    40,    41,    42,    43,    44,    45,    16,    -1,
      -1,    49,    -1,    -1,    -1,    -1,    -1,    -1,    56,    57,
      -1,    59,    -1,    61,    62,    -1,    -1,    -1,    -1,    -1,
      38,    39,    40,    41,    42,    43,    44,    45,    46,    16,
      48,    49,    50,    51,    52,    53,    -1,    55,    -1,    -1,
      -1,    59,    -1,    61,    62,    -1,    64,    -1,    -1,    -1,
      -1,    38,    39,    40,    41,    42,    43,    44,    45,    46,
      16,    48,    49,    50,    51,    52,    53,    -1,    55,    -1,
      -1,    -1,    59,    60,    61,    -1,    -1,    64,    -1,    -1,
      -1,    -1,    38,    39,    40,    41,    42,    43,    44,    45,
      46,    16,    48,    49,    50,    51,    52,    53,    -1,    55,
      -1,    -1,    -1,    59,    60,    61,    -1,    -1,    64,    -1,
      -1,    -1,    -1,    38,    39,    40,    41,    42,    43,    44,
      45,    46,    -1,    48,    49,    50,    51,    52,    53,    -1,
      55,    -1,    -1,    -1,    59,    -1,    61,    13,    -1,    64,
      -1,    17,    18,    19,    20,    -1,    -1,    -1,    -1,    26,
      -1,    27,    -1,    29,    -1,    -1,    32,    33,    34,    35,
      -1,    38,    39,    40,    41,    42,    43,    44,    45,    46,
      -1,    48,    49,    50,    51,    52,    53,    -1,    55,    -1,
      56,    57,    59,    -1,    61,    -1,    -1,    64,    37,    38,
      39,    40,    41,    42,    43,    44,    45,    46,    -1,    48,
      49,    50,    51,    52,    53,    -1,    55,    -1,    -1,    -1,
      59,    -1,    61,    -1,    -1,    64,    38,    39,    40,    41,
      42,    43,    44,    45,    46,    -1,    48,    49,    50,    51,
      52,    53,    -1,    55,    -1,    -1,    -1,    59,    60,    61,
      -1,    -1,    64,    38,    39,    40,    41,    42,    43,    44,
      45,    46,    -1,    48,    49,    50,    51,    52,    53,    -1,
      55,    -1,    -1,    -1,    59,    -1,    61,    62,    -1,    64,
      38,    39,    40,    41,    42,    43,    44,    45,    46,    47,
      48,    49,    50,    51,    52,    53,    -1,    55,    -1,    -1,
      -1,    59,    -1,    61,    -1,    -1,    64,    38,    39,    40,
      41,    42,    43,    44,    45,    46,    -1,    48,    49,    50,
      51,    52,    53,    -1,    55,    -1,    -1,    -1,    59,    -1,
      61,    -1,    63,    64,    38,    39,    40,    41,    42,    43,
      44,    45,    46,    -1,    48,    49,    50,    51,    52,    53,
      -1,    55,    -1,    -1,    -1,    59,    -1,    61,    -1,    -1,
      64,    38,    39,    40,    41,    42,    43,    44,    45,    46,
      -1,    48,    49,    50,    51,    52,    53,    -1,    55,    -1,
      -1,    -1,    59,    -1,    61,    -1,    -1,    64,    39,    40,
      41,    42,    43,    44,    45,    46,    -1,    48,    49,    50,
      51,    52,    53,    -1,    55,    -1,    -1,    -1,    59,    -1,
      61,    -1,    -1,    64,    40,    41,    42,    43,    44,    45,
      46,    -1,    48,    49,    50,    51,    52,    53,    -1,    55,
      -1,    -1,    -1,    59,    -1,    61,    -1,    -1,    64,    40,
      41,    42,    43,    44,    45,    46,    -1,    48,    49,    50,
      51,    52,    53,    -1,    55,    -1,    -1,    -1,    59,    -1,
      61,    -1,    -1,    64,    48,    49,    50,    51,    52,    53,
      -1,    55,    -1,    -1,    -1,    59,    -1,    61,    -1,    -1,
      64,     3,     4,     5,     6,     7,     8,     9,    10,    11,
      12
};

  /* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static const yytype_uint8 yystos[] =
{
       0,     3,     4,     5,     6,     7,     8,     9,    10,    11,
      12,    66,    13,    17,    18,    19,    20,    27,    29,    32,
      33,    34,    35,    40,    41,    42,    43,    44,    45,    49,
      56,    57,    59,    61,    62,    68,    69,    70,    82,    83,
      84,    88,    89,    91,    97,   128,    61,    69,    91,    68,
      70,    70,    76,    67,    49,    56,    68,    80,    87,    89,
     106,    80,    81,    56,    87,    89,    59,    61,    62,   137,
     138,     0,    34,    73,    71,    92,    59,    68,    72,    68,
      86,    86,    86,    86,    86,    86,    68,    59,    68,    86,
      62,    68,    86,    98,    86,    38,    39,    40,    41,    42,
      43,    44,    45,    46,    48,    49,    50,    51,    52,    53,
      55,    59,    61,    64,    85,    86,    68,    16,    14,    49,
      68,    77,    78,    68,    64,    49,    50,    51,    52,    53,
      16,    33,    34,    57,   140,   140,   140,   139,   140,   124,
     125,   126,   119,   120,   121,    30,    34,    56,    93,    94,
      96,   129,    26,   124,    90,    60,    63,    16,    62,    99,
      63,    63,    68,    68,    68,    68,    68,    68,    68,    68,
      74,    59,    68,    68,    68,    68,    68,    68,    22,    56,
      60,    68,   100,   101,   102,   104,    68,    56,    60,    61,
      62,    76,    59,    16,   106,    68,    68,    68,    68,    68,
      81,    59,    63,    63,    63,    60,    61,    62,    36,    16,
      56,    31,    16,    56,    16,    30,    95,    15,    56,    60,
     130,   134,    68,    36,   100,    68,    68,    68,    79,    75,
      15,    16,    60,   105,    16,    60,   103,    62,    78,    78,
     141,    68,   124,   127,    68,   119,   122,    94,    68,    15,
      28,    68,   136,   131,    37,    68,    99,    47,    16,    23,
      24,    25,    27,    58,   106,   107,    68,    68,    56,   102,
      60,   100,    48,    48,    95,   135,    68,    16,    60,   132,
      68,    68,    78,    42,    42,    42,    42,   105,   103,    68,
      68,   123,   107,   134,    60,   108,   109,    56,   111,   112,
      44,   115,   116,    63,   133,   107,   107,    15,    16,    44,
     114,    21,   107,    68,   132,    44,    44,   113,   112,   110,
      16,    44,   117,   107,   114,   107,   118,   107,   117
};

  /* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static const yytype_uint8 yyr1[] =
{
       0,    65,    66,    66,    66,    66,    66,    67,    66,    66,
      66,    66,    66,    68,    68,    69,    69,    69,    71,    70,
      70,    72,    70,    73,    70,    74,    70,    70,    70,    70,
      70,    70,    70,    70,    70,    70,    70,    70,    70,    70,
      70,    70,    70,    75,    70,    70,    70,    70,    70,    70,
      70,    70,    76,    76,    77,    77,    77,    78,    78,    79,
      80,    80,    80,    80,    80,    80,    80,    81,    81,    82,
      82,    82,    82,    82,    82,    82,    83,    84,    84,    84,
      85,    85,    85,    86,    87,    87,    88,    88,    89,    89,
      89,    89,    90,    89,    92,    91,    93,    93,    94,    95,
      95,    96,    96,    97,    98,    98,    99,    99,   100,   100,
     100,   101,   102,   103,   103,   104,   105,   105,   106,   106,
     107,   107,   108,   107,   109,   107,   107,   110,   107,   111,
     113,   112,   114,   114,   115,   116,   115,   117,   118,   117,
     119,   119,   121,   122,   120,   123,   123,   124,   124,   126,
     127,   125,   129,   128,   130,   131,   130,   132,   133,   132,
     135,   134,   134,   136,   136,   137,   138,   138,   138,   139,
     139,   139,   140,   140,   141,   140
};

  /* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static const yytype_int8 yyr2[] =
{
       0,     2,     2,     2,     2,     2,     2,     0,     3,     2,
       2,     2,     2,     1,     1,     1,     1,     1,     0,     5,
       6,     0,     5,     0,     5,     0,     6,     3,     3,     3,
       3,     3,     3,     3,     3,     4,     3,     3,     3,     3,
       3,     3,     2,     0,     5,     3,     4,     3,     1,     1,
       1,     3,     3,     1,     1,     4,     1,     3,     1,     4,
       3,     3,     3,     3,     3,     2,     1,     3,     1,     2,
       2,     2,     2,     2,     2,     1,     2,     3,     3,     3,
       2,     2,     2,     1,     1,     1,     1,     1,     1,     1,
       1,     2,     0,     4,     0,     3,     1,     2,     3,     1,
       3,     1,     1,     2,     1,     2,     1,     3,     1,     1,
       1,     2,     3,     1,     3,     2,     1,     3,     3,     1,
       1,     1,     0,     5,     0,     5,     3,     0,     6,     2,
       0,     4,     1,     3,     1,     0,     3,     1,     0,     4,
       3,     1,     0,     0,     5,     1,     3,     3,     1,     0,
       0,     5,     0,     5,     1,     0,     3,     1,     0,     4,
       0,     4,     1,     2,     1,     2,     3,     3,     3,     2,
       2,     2,     1,     1,     0,     4
};


enum { YYENOMEM = -2 };

#define yyerrok         (yyerrstatus = 0)
#define yyclearin       (yychar = TOKEN_YYEMPTY)

#define YYACCEPT        goto yyacceptlab
#define YYABORT         goto yyabortlab
#define YYERROR         goto yyerrorlab


#define YYRECOVERING()  (!!yyerrstatus)

#define YYBACKUP(Token, Value)                                    \
  do                                                              \
    if (yychar == TOKEN_YYEMPTY)                                        \
      {                                                           \
        yychar = (Token);                                         \
        yylval = (Value);                                         \
        YYPOPSTACK (yylen);                                       \
        yystate = *yyssp;                                         \
        goto yybackup;                                            \
      }                                                           \
    else                                                          \
      {                                                           \
        yyerror (YY_("syntax error: cannot back up")); \
        YYERROR;                                                  \
      }                                                           \
  while (0)

/* Backward compatibility with an undocumented macro.
   Use TOKEN_YYerror or TOKEN_YYUNDEF. */
#define YYERRCODE TOKEN_YYUNDEF


/* Enable debugging if requested.  */
#if YYDEBUG

# ifndef YYFPRINTF
#  include <stdio.h> /* INFRINGES ON USER NAME SPACE */
#  define YYFPRINTF fprintf
# endif

# define YYDPRINTF(Args)                        \
do {                                            \
  if (yydebug)                                  \
    YYFPRINTF Args;                             \
} while (0)

/* This macro is provided for backward compatibility. */
# ifndef YY_LOCATION_PRINT
#  define YY_LOCATION_PRINT(File, Loc) ((void) 0)
# endif


# define YY_SYMBOL_PRINT(Title, Kind, Value, Location)                    \
do {                                                                      \
  if (yydebug)                                                            \
    {                                                                     \
      YYFPRINTF (stderr, "%s ", Title);                                   \
      yy_symbol_print (stderr,                                            \
                  Kind, Value); \
      YYFPRINTF (stderr, "\n");                                           \
    }                                                                     \
} while (0)


/*-----------------------------------.
| Print this symbol's value on YYO.  |
`-----------------------------------*/

static void
yy_symbol_value_print (FILE *yyo,
                       yysymbol_kind_t yykind, YYSTYPE const * const yyvaluep)
{
  FILE *yyoutput = yyo;
  YY_USE (yyoutput);
  if (!yyvaluep)
    return;
# ifdef YYPRINT
  if (yykind < YYNTOKENS)
    YYPRINT (yyo, yytoknum[yykind], *yyvaluep);
# endif
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YY_USE (yykind);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}


/*---------------------------.
| Print this symbol on YYO.  |
`---------------------------*/

static void
yy_symbol_print (FILE *yyo,
                 yysymbol_kind_t yykind, YYSTYPE const * const yyvaluep)
{
  YYFPRINTF (yyo, "%s %s (",
             yykind < YYNTOKENS ? "token" : "nterm", yysymbol_name (yykind));

  yy_symbol_value_print (yyo, yykind, yyvaluep);
  YYFPRINTF (yyo, ")");
}

/*------------------------------------------------------------------.
| yy_stack_print -- Print the state stack from its BOTTOM up to its |
| TOP (included).                                                   |
`------------------------------------------------------------------*/

static void
yy_stack_print (yy_state_t *yybottom, yy_state_t *yytop)
{
  YYFPRINTF (stderr, "Stack now");
  for (; yybottom <= yytop; yybottom++)
    {
      int yybot = *yybottom;
      YYFPRINTF (stderr, " %d", yybot);
    }
  YYFPRINTF (stderr, "\n");
}

# define YY_STACK_PRINT(Bottom, Top)                            \
do {                                                            \
  if (yydebug)                                                  \
    yy_stack_print ((Bottom), (Top));                           \
} while (0)


/*------------------------------------------------.
| Report that the YYRULE is going to be reduced.  |
`------------------------------------------------*/

static void
yy_reduce_print (yy_state_t *yyssp, YYSTYPE *yyvsp,
                 int yyrule)
{
  int yylno = yyrline[yyrule];
  int yynrhs = yyr2[yyrule];
  int yyi;
  YYFPRINTF (stderr, "Reducing stack by rule %d (line %d):\n",
             yyrule - 1, yylno);
  /* The symbols being reduced.  */
  for (yyi = 0; yyi < yynrhs; yyi++)
    {
      YYFPRINTF (stderr, "   $%d = ", yyi + 1);
      yy_symbol_print (stderr,
                       YY_ACCESSING_SYMBOL (+yyssp[yyi + 1 - yynrhs]),
                       &yyvsp[(yyi + 1) - (yynrhs)]);
      YYFPRINTF (stderr, "\n");
    }
}

# define YY_REDUCE_PRINT(Rule)          \
do {                                    \
  if (yydebug)                          \
    yy_reduce_print (yyssp, yyvsp, Rule); \
} while (0)

/* Nonzero means print parse trace.  It is left uninitialized so that
   multiple parsers can coexist.  */
int yydebug;
#else /* !YYDEBUG */
# define YYDPRINTF(Args) ((void) 0)
# define YY_SYMBOL_PRINT(Title, Kind, Value, Location)
# define YY_STACK_PRINT(Bottom, Top)
# define YY_REDUCE_PRINT(Rule)
#endif /* !YYDEBUG */


/* YYINITDEPTH -- initial size of the parser's stacks.  */
#ifndef YYINITDEPTH
# define YYINITDEPTH 200
#endif

/* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
   if the built-in stack extension method is used).

   Do not make this value too large; the results are undefined if
   YYSTACK_ALLOC_MAXIMUM < YYSTACK_BYTES (YYMAXDEPTH)
   evaluated with infinite-precision integer arithmetic.  */

#ifndef YYMAXDEPTH
# define YYMAXDEPTH 10000
#endif






/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/

static void
yydestruct (const char *yymsg,
            yysymbol_kind_t yykind, YYSTYPE *yyvaluep)
{
  YY_USE (yyvaluep);
  if (!yymsg)
    yymsg = "Deleting";
  YY_SYMBOL_PRINT (yymsg, yykind, yyvaluep, yylocationp);

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YY_USE (yykind);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}


/* Lookahead token kind.  */
int yychar;

/* The semantic value of the lookahead symbol.  */
YYSTYPE yylval;
/* Number of syntax errors so far.  */
int yynerrs;




/*----------.
| yyparse.  |
`----------*/

int
yyparse (void)
{
    yy_state_fast_t yystate = 0;
    /* Number of tokens to shift before error messages enabled.  */
    int yyerrstatus = 0;

    /* Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */

    /* Their size.  */
    YYPTRDIFF_T yystacksize = YYINITDEPTH;

    /* The state stack: array, bottom, top.  */
    yy_state_t yyssa[YYINITDEPTH];
    yy_state_t *yyss = yyssa;
    yy_state_t *yyssp = yyss;

    /* The semantic value stack: array, bottom, top.  */
    YYSTYPE yyvsa[YYINITDEPTH];
    YYSTYPE *yyvs = yyvsa;
    YYSTYPE *yyvsp = yyvs;

  int yyn;
  /* The return value of yyparse.  */
  int yyresult;
  /* Lookahead symbol kind.  */
  yysymbol_kind_t yytoken = SYMBOL_YYEMPTY;
  /* The variables used to return semantic value and location from the
     action routines.  */
  YYSTYPE yyval;



#define YYPOPSTACK(N)   (yyvsp -= (N), yyssp -= (N))

  /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
  int yylen = 0;

  YYDPRINTF ((stderr, "Starting parse\n"));

  yychar = TOKEN_YYEMPTY; /* Cause a token to be read.  */
  goto yysetstate;


/*------------------------------------------------------------.
| yynewstate -- push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
yynewstate:
  /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
  yyssp++;


/*--------------------------------------------------------------------.
| yysetstate -- set current state (the top of the stack) to yystate.  |
`--------------------------------------------------------------------*/
yysetstate:
  YYDPRINTF ((stderr, "Entering state %d\n", yystate));
  YY_ASSERT (0 <= yystate && yystate < YYNSTATES);
  YY_IGNORE_USELESS_CAST_BEGIN
  *yyssp = YY_CAST (yy_state_t, yystate);
  YY_IGNORE_USELESS_CAST_END
  YY_STACK_PRINT (yyss, yyssp);

  if (yyss + yystacksize - 1 <= yyssp)
#if !defined yyoverflow && !defined YYSTACK_RELOCATE
    goto yyexhaustedlab;
#else
    {
      /* Get the current used size of the three stacks, in elements.  */
      YYPTRDIFF_T yysize = yyssp - yyss + 1;

# if defined yyoverflow
      {
        /* Give user a chance to reallocate the stack.  Use copies of
           these so that the &'s don't force the real ones into
           memory.  */
        yy_state_t *yyss1 = yyss;
        YYSTYPE *yyvs1 = yyvs;

        /* Each stack pointer address is followed by the size of the
           data in use in that stack, in bytes.  This used to be a
           conditional around just the two extra args, but that might
           be undefined if yyoverflow is a macro.  */
        yyoverflow (YY_("memory exhausted"),
                    &yyss1, yysize * YYSIZEOF (*yyssp),
                    &yyvs1, yysize * YYSIZEOF (*yyvsp),
                    &yystacksize);
        yyss = yyss1;
        yyvs = yyvs1;
      }
# else /* defined YYSTACK_RELOCATE */
      /* Extend the stack our own way.  */
      if (YYMAXDEPTH <= yystacksize)
        goto yyexhaustedlab;
      yystacksize *= 2;
      if (YYMAXDEPTH < yystacksize)
        yystacksize = YYMAXDEPTH;

      {
        yy_state_t *yyss1 = yyss;
        union yyalloc *yyptr =
          YY_CAST (union yyalloc *,
                   YYSTACK_ALLOC (YY_CAST (YYSIZE_T, YYSTACK_BYTES (yystacksize))));
        if (! yyptr)
          goto yyexhaustedlab;
        YYSTACK_RELOCATE (yyss_alloc, yyss);
        YYSTACK_RELOCATE (yyvs_alloc, yyvs);
#  undef YYSTACK_RELOCATE
        if (yyss1 != yyssa)
          YYSTACK_FREE (yyss1);
      }
# endif

      yyssp = yyss + yysize - 1;
      yyvsp = yyvs + yysize - 1;

      YY_IGNORE_USELESS_CAST_BEGIN
      YYDPRINTF ((stderr, "Stack size increased to %ld\n",
                  YY_CAST (long, yystacksize)));
      YY_IGNORE_USELESS_CAST_END

      if (yyss + yystacksize - 1 <= yyssp)
        YYABORT;
    }
#endif /* !defined yyoverflow && !defined YYSTACK_RELOCATE */

  if (yystate == YYFINAL)
    YYACCEPT;

  goto yybackup;


/*-----------.
| yybackup.  |
`-----------*/
yybackup:
  /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */

  /* First try to decide what to do without reference to lookahead token.  */
  yyn = yypact[yystate];
  if (yypact_value_is_default (yyn))
    goto yydefault;

  /* Not known => get a lookahead token if don't already have one.  */

  /* YYCHAR is either empty, or end-of-input, or a valid lookahead.  */
  if (yychar == TOKEN_YYEMPTY)
    {
      YYDPRINTF ((stderr, "Reading a token\n"));
      yychar = yylex ();
    }

  if (yychar <= TOKEN_YYEOF)
    {
      yychar = TOKEN_YYEOF;
      yytoken = SYMBOL_YYEOF;
      YYDPRINTF ((stderr, "Now at end of input.\n"));
    }
  else if (yychar == TOKEN_YYerror)
    {
      /* The scanner already issued an error message, process directly
         to error recovery.  But do not keep the error token as
         lookahead, it is too special and may lead us to an endless
         loop in error recovery. */
      yychar = TOKEN_YYUNDEF;
      yytoken = SYMBOL_YYerror;
      goto yyerrlab1;
    }
  else
    {
      yytoken = YYTRANSLATE (yychar);
      YY_SYMBOL_PRINT ("Next token is", yytoken, &yylval, &yylloc);
    }

  /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
  yyn += yytoken;
  if (yyn < 0 || YYLAST < yyn || yycheck[yyn] != yytoken)
    goto yydefault;
  yyn = yytable[yyn];
  if (yyn <= 0)
    {
      if (yytable_value_is_error (yyn))
        goto yyerrlab;
      yyn = -yyn;
      goto yyreduce;
    }

  /* Count tokens shifted since error; after three, turn off error
     status.  */
  if (yyerrstatus)
    yyerrstatus--;

  /* Shift the lookahead token.  */
  YY_SYMBOL_PRINT ("Shifting", yytoken, &yylval, &yylloc);
  yystate = yyn;
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END

  /* Discard the shifted token.  */
  yychar = TOKEN_YYEMPTY;
  goto yynewstate;


/*-----------------------------------------------------------.
| yydefault -- do the default action for the current state.  |
`-----------------------------------------------------------*/
yydefault:
  yyn = yydefact[yystate];
  if (yyn == 0)
    goto yyerrlab;
  goto yyreduce;


/*-----------------------------.
| yyreduce -- do a reduction.  |
`-----------------------------*/
yyreduce:
  /* yyn is the number of a rule to reduce with.  */
  yylen = yyr2[yyn];

  /* If YYLEN is nonzero, implement the default value of the action:
     '$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
  yyval = yyvsp[1-yylen];


  YY_REDUCE_PRINT (yyn);
  switch (yyn)
    {
  case 7: /* $@1: %empty  */
                      {/* unary_tests_begin */}
    break;

  case 18: /* $@2: %empty  */
        {/* for_begin */}
    break;

  case 19: /* textual_expression: FOR $@2 iteration_contexts RETURN expression  */
                                                               {/* for */}
    break;

  case 20: /* textual_expression: IF expression THEN expression ELSE expression  */
                                                  {/* if */}
    break;

  case 21: /* $@3: %empty  */
         {/* some_begin */}
    break;

  case 22: /* textual_expression: SOME $@3 quantified_expressions SATISFIES expression  */
                                                                        {/* some */}
    break;

  case 23: /* $@4: %empty  */
          {/* every_begin */}
    break;

  case 24: /* textual_expression: EVERY $@4 quantified_expressions SATISFIES expression  */
                                                                          {/* every */}
    break;

  case 25: /* $@5: %empty  */
                       {/* between_begin */}
    break;

  case 26: /* textual_expression: expression BETWEEN $@5 expression BETWEEN_AND expression  */
                                                                               {/* between */}
    break;

  case 27: /* textual_expression: expression OR expression  */
                             {/* disjunction */}
    break;

  case 28: /* textual_expression: expression AND expression  */
                              {/* conjunction */}
    break;

  case 29: /* textual_expression: expression EQ expression  */
                             {/* comparison_eq */}
    break;

  case 30: /* textual_expression: expression NE expression  */
                             {/* comparison_ne */}
    break;

  case 31: /* textual_expression: expression LT expression  */
                             {/* comparison_lt */}
    break;

  case 32: /* textual_expression: expression LE expression  */
                             {/* comparison_le */}
    break;

  case 33: /* textual_expression: expression GT expression  */
                             {/* comparison_gt */}
    break;

  case 34: /* textual_expression: expression GE expression  */
                             {/* comparison_ge */}
    break;

  case 35: /* textual_expression: expression IN LEFT_PAREN comparison_in  */
                                           {/* comparison_in */}
    break;

  case 36: /* textual_expression: expression IN expression  */
                             {/* comparison_in */}
    break;

  case 37: /* textual_expression: expression PLUS expression  */
                               {/* addition */}
    break;

  case 38: /* textual_expression: expression MINUS expression  */
                                {/* subtraction */}
    break;

  case 39: /* textual_expression: expression MUL expression  */
                              {/* multiplication */}
    break;

  case 40: /* textual_expression: expression DIV expression  */
                              {/* division */}
    break;

  case 41: /* textual_expression: expression EXP expression  */
                              {/* exponentiation */}
    break;

  case 42: /* textual_expression: MINUS expression  */
                                    {/* negation */}
    break;

  case 43: /* $@6: %empty  */
                           {/* type_name */}
    break;

  case 44: /* textual_expression: expression INSTANCE OF $@6 type  */
                                                  {/* instance_of */}
    break;

  case 45: /* textual_expression: expression DOT NAME  */
                        {/* path */}
    break;

  case 46: /* textual_expression: expression LEFT_BRACKET expression RIGHT_BRACKET  */
                                                     {/* filter */}
    break;

  case 50: /* textual_expression: NAME  */
         {/* name */}
    break;

  case 52: /* textual_expressions: textual_expression COMMA textual_expressions  */
                                                 {/* expression_list_tail */}
    break;

  case 53: /* textual_expressions: textual_expression  */
                       {/* expression_list_tail */}
    break;

  case 54: /* unary_tests: MINUS  */
          {/* unary_tests_irrelevant */}
    break;

  case 55: /* unary_tests: NOT LEFT_PAREN positive_unary_tests RIGHT_PAREN  */
                                                    {/* unary_tests_negated */}
    break;

  case 57: /* positive_unary_tests: expression COMMA positive_unary_tests  */
                                          {/* expression_list_tail */}
    break;

  case 58: /* positive_unary_tests: expression  */
               {/* expression_list_tail */}
    break;

  case 59: /* comparison_in: expression COMMA positive_unary_tests RIGHT_PAREN  */
                                                      {/* expression_list_tail */}
    break;

  case 60: /* simple_expression: expression PLUS expression  */
                               {/* addition */}
    break;

  case 61: /* simple_expression: expression MINUS expression  */
                                {/* subtraction */}
    break;

  case 62: /* simple_expression: expression MUL expression  */
                              {/* multiplication */}
    break;

  case 63: /* simple_expression: expression DIV expression  */
                              {/* division */}
    break;

  case 64: /* simple_expression: expression EXP expression  */
                              {/* exponentiation */}
    break;

  case 65: /* simple_expression: MINUS expression  */
                                    {/* negation */}
    break;

  case 67: /* simple_expressions: simple_expression COMMA simple_expressions  */
                                               {/* expression_list_tail */}
    break;

  case 68: /* simple_expressions: simple_expression  */
                      {/* expression_list_tail */}
    break;

  case 69: /* simple_positive_unary_test: LT endpoint  */
                {/* comparison_unary_lt */}
    break;

  case 70: /* simple_positive_unary_test: LE endpoint  */
                {/* comparison_unary_le */}
    break;

  case 71: /* simple_positive_unary_test: GT endpoint  */
                {/* comparison_unary_gt */}
    break;

  case 72: /* simple_positive_unary_test: GE endpoint  */
                {/* comparison_unary_ge */}
    break;

  case 73: /* simple_positive_unary_test: EQ endpoint  */
                {/* comparison_unary_eq */}
    break;

  case 74: /* simple_positive_unary_test: NE endpoint  */
                {/* comparison_unary_ne */}
    break;

  case 76: /* interval: interval_start interval_end  */
                                {/* interval */}
    break;

  case 77: /* interval_start: LEFT_PAREN endpoint ELLIPSIS  */
                                 {/* interval_start */}
    break;

  case 78: /* interval_start: RIGHT_BRACKET endpoint ELLIPSIS  */
                                    {/* interval_start */}
    break;

  case 79: /* interval_start: LEFT_BRACKET endpoint ELLIPSIS  */
                                   {/* interval_start */}
    break;

  case 80: /* interval_end: endpoint RIGHT_PAREN  */
                         {/* interval_end */}
    break;

  case 81: /* interval_end: endpoint LEFT_BRACKET  */
                          {/* interval_end */}
    break;

  case 82: /* interval_end: endpoint RIGHT_BRACKET  */
                           {/* interval_end */}
    break;

  case 87: /* literal: NULL  */
         {/* literal_null */}
    break;

  case 88: /* simple_literal: NUMERIC  */
            {/* literal_numeric */}
    break;

  case 89: /* simple_literal: STRING  */
           {/* literal_string */}
    break;

  case 90: /* simple_literal: BOOLEAN  */
            {/* literal_boolean */}
    break;

  case 91: /* simple_literal: AT STRING  */
              {/* literal_at */}
    break;

  case 92: /* $@7: %empty  */
                              {/* literal_date_time */}
    break;

  case 94: /* $@8: %empty  */
               {/* context_begin */}
    break;

  case 95: /* context: LEFT_BRACE $@8 context_entries  */
                                                     {/* context_end */}
    break;

  case 96: /* context_entries: RIGHT_BRACE  */
                {/* empty_context */}
    break;

  case 97: /* context_entries: context_entry context_entry_tail  */
                                     {/* context_entry_tail */}
    break;

  case 98: /* context_entry: key COLON expression  */
                         {/* context_entry */}
    break;

  case 100: /* context_entry_tail: COMMA context_entry context_entry_tail  */
                                           {/* context_entry_tail */}
    break;

  case 101: /* key: NAME  */
         {/* key_name */}
    break;

  case 102: /* key: STRING  */
           {/* key_string */}
    break;

  case 103: /* list: LEFT_BRACKET list_items  */
                            {/* list */}
    break;

  case 104: /* list_items: RIGHT_BRACKET  */
                            {/* list_empty */}
    break;

  case 105: /* list_items: expression list_tail  */
                         {/* list_tail */}
    break;

  case 107: /* list_tail: COMMA expression list_tail  */
                               {/* list_tail */}
    break;

  case 108: /* parameters: RIGHT_PAREN  */
                {/* function_invocation_no_parameters */}
    break;

  case 109: /* parameters: named_parameters  */
                     {/* function_invocation */}
    break;

  case 110: /* parameters: positional_parameters  */
                          {/* function_invocation */}
    break;

  case 111: /* named_parameters: named_parameter named_parameters_tail  */
                                          {/* named_parameters_tail */}
    break;

  case 112: /* named_parameter: NAME COLON expression  */
                          {/* named_parameter */}
    break;

  case 114: /* named_parameters_tail: COMMA named_parameter named_parameters_tail  */
                                                {/* named_parameters_tail */}
    break;

  case 115: /* positional_parameters: expression positional_parameters_tail  */
                                          {/* positional_parameters_tail */}
    break;

  case 117: /* positional_parameters_tail: COMMA expression positional_parameters_tail  */
                                                {/* positional_parameters_tail */}
    break;

  case 118: /* qualified_name: NAME DOT qualified_name  */
                            {/* qualified_name_tail */}
    break;

  case 119: /* qualified_name: NAME  */
         {/* qualified_name */}
    break;

  case 120: /* type: BUILT_IN_TYPE_NAME  */
                       {/* built_in_type_name */}
    break;

  case 122: /* $@9: %empty  */
            {/* type_name */}
    break;

  case 123: /* type: LIST LT $@9 type GT  */
                                      {/* list_type */}
    break;

  case 124: /* $@10: %empty  */
             {/* type_name */}
    break;

  case 125: /* type: RANGE LT $@10 type GT  */
                                       {/* range_type */}
    break;

  case 127: /* $@11: %empty  */
                                                     {/* type_name */}
    break;

  case 128: /* type: FUNCTION LT function_type_parameters RIGHT_ARROW $@11 type  */
                                                                            {/* function_type */}
    break;

  case 129: /* context_type_entries: context_type_entry context_type_entry_tail  */
                                               {/* context_type_entry_tail */}
    break;

  case 130: /* $@12: %empty  */
               {/* type_name */}
    break;

  case 131: /* context_type_entry: NAME COLON $@12 type  */
                                      {/* context_type_entry */}
    break;

  case 133: /* context_type_entry_tail: COMMA context_type_entry context_type_entry_tail  */
                                                     {/* context_type_entry_tail */}
    break;

  case 134: /* function_type_parameters: GT  */
       {/* function_type_parameters_empty */}
    break;

  case 135: /* $@13: %empty  */
    {/* type_name */}
    break;

  case 136: /* function_type_parameters: $@13 type function_type_parameters_tail  */
                                                         {/* function_type_parameters_tail */}
    break;

  case 138: /* $@14: %empty  */
          {/* type_name */}
    break;

  case 139: /* function_type_parameters_tail: COMMA $@14 type function_type_parameters_tail  */
                                                               {/* function_type_parameters_tail */}
    break;

  case 140: /* iteration_contexts: iteration_context COMMA iteration_contexts  */
                                               {/* iteration_contexts_tail */}
    break;

  case 141: /* iteration_contexts: iteration_context  */
                      {/* iteration_contexts_tail */}
    break;

  case 142: /* $@15: %empty  */
    {/* iteration_context_variable_name_begin */}
    break;

  case 143: /* $@16: %empty  */
                                                       {/* iteration_context_variable_name */}
    break;

  case 145: /* iteration_context_value: expression  */
               {/* iteration_context_value_single */}
    break;

  case 146: /* iteration_context_value: expression ELLIPSIS expression  */
                                   {/* iteration_context_value_range */}
    break;

  case 147: /* quantified_expressions: quantified_expression COMMA quantified_expressions  */
                                                       {/* quantified_expressions_tail */}
    break;

  case 148: /* quantified_expressions: quantified_expression  */
                          {/* quantified_expressions_tail */}
    break;

  case 149: /* $@17: %empty  */
    {/* quantified_expression_variable_name_begin */}
    break;

  case 150: /* $@18: %empty  */
                                                           {/* quantified_expression_variable_name */}
    break;

  case 151: /* quantified_expression: $@17 NAME $@18 IN expression  */
                                                                                                                     {/* quantified_expression */}
    break;

  case 152: /* $@19: %empty  */
                        {/* formal_parameters_begin */}
    break;

  case 153: /* function_definition: FUNCTION LEFT_PAREN $@19 formal_parameters external  */
                                                                                   {/* function_definition */}
    break;

  case 154: /* formal_parameters: RIGHT_PAREN  */
                {/* formal_parameters_empty */}
    break;

  case 155: /* $@20: %empty  */
                     {/* formal_parameters_first */}
    break;

  case 158: /* $@21: %empty  */
                           {/* formal_parameters_tail */}
    break;

  case 160: /* $@22: %empty  */
               {/* type_name */}
    break;

  case 161: /* formal_parameter: NAME COLON $@22 type  */
                                      {/* formal_parameter_with_type */}
    break;

  case 162: /* formal_parameter: NAME  */
         {/* formal_parameter_without_type */}
    break;

  case 163: /* external: EXTERNAL expression  */
                        {/* function_body_external */}
    break;

  case 164: /* external: expression  */
                              {/* function_body */}
    break;

  case 165: /* range_literal: range_literal_start range_literal_end  */
                                          {/* range_literal */}
    break;

  case 166: /* range_literal_start: LEFT_PAREN range_endpoint ELLIPSIS  */
                                       {/* range_literal_start */}
    break;

  case 167: /* range_literal_start: RIGHT_BRACKET range_endpoint ELLIPSIS  */
                                          {/* range_literal_start */}
    break;

  case 168: /* range_literal_start: LEFT_BRACKET range_endpoint ELLIPSIS  */
                                         {/* range_literal_start */}
    break;

  case 169: /* range_literal_end: range_endpoint RIGHT_PAREN  */
                               {/* range_literal_end */}
    break;

  case 170: /* range_literal_end: range_endpoint LEFT_BRACKET  */
                                {/* range_literal_end */}
    break;

  case 171: /* range_literal_end: range_endpoint RIGHT_BRACKET  */
                                 {/* range_literal_end */}
    break;

  case 172: /* range_endpoint: NUMERIC  */
            {/* literal_numeric */}
    break;

  case 173: /* range_endpoint: STRING  */
           {/* literal_string */}
    break;

  case 174: /* $@23: %empty  */
                              {/* literal_date_time */}
    break;



      default: break;
    }
  /* User semantic actions sometimes alter yychar, and that requires
     that yytoken be updated with the new translation.  We take the
     approach of translating immediately before every use of yytoken.
     One alternative is translating here after every semantic action,
     but that translation would be missed if the semantic action invokes
     YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
     if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
     incorrect destructor might then be invoked immediately.  In the
     case of YYERROR or YYBACKUP, subsequent parser actions might lead
     to an incorrect destructor call or verbose syntax error message
     before the lookahead is translated.  */
  YY_SYMBOL_PRINT ("-> $$ =", YY_CAST (yysymbol_kind_t, yyr1[yyn]), &yyval, &yyloc);

  YYPOPSTACK (yylen);
  yylen = 0;

  *++yyvsp = yyval;

  /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */
  {
    const int yylhs = yyr1[yyn] - YYNTOKENS;
    const int yyi = yypgoto[yylhs] + *yyssp;
    yystate = (0 <= yyi && yyi <= YYLAST && yycheck[yyi] == *yyssp
               ? yytable[yyi]
               : yydefgoto[yylhs]);
  }

  goto yynewstate;


/*--------------------------------------.
| yyerrlab -- here on detecting error.  |
`--------------------------------------*/
yyerrlab:
  /* Make sure we have latest lookahead translation.  See comments at
     user semantic actions for why this is necessary.  */
  yytoken = yychar == TOKEN_YYEMPTY ? SYMBOL_YYEMPTY : YYTRANSLATE (yychar);
  /* If not already recovering from an error, report this error.  */
  if (!yyerrstatus)
    {
      ++yynerrs;
      yyerror (YY_("syntax error"));
    }

  if (yyerrstatus == 3)
    {
      /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */

      if (yychar <= TOKEN_YYEOF)
        {
          /* Return failure if at end of input.  */
          if (yychar == TOKEN_YYEOF)
            YYABORT;
        }
      else
        {
          yydestruct ("Error: discarding",
                      yytoken, &yylval);
          yychar = TOKEN_YYEMPTY;
        }
    }

  /* Else will try to reuse lookahead token after shifting the error
     token.  */
  goto yyerrlab1;


/*---------------------------------------------------.
| yyerrorlab -- error raised explicitly by YYERROR.  |
`---------------------------------------------------*/
yyerrorlab:
  /* Pacify compilers when the user code never invokes YYERROR and the
     label yyerrorlab therefore never appears in user code.  */
  if (0)
    YYERROR;

  /* Do not reclaim the symbols of the rule whose action triggered
     this YYERROR.  */
  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);
  yystate = *yyssp;
  goto yyerrlab1;


/*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
yyerrlab1:
  yyerrstatus = 3;      /* Each real token shifted decrements this.  */

  /* Pop stack until we find a state that shifts the error token.  */
  for (;;)
    {
      yyn = yypact[yystate];
      if (!yypact_value_is_default (yyn))
        {
          yyn += SYMBOL_YYerror;
          if (0 <= yyn && yyn <= YYLAST && yycheck[yyn] == SYMBOL_YYerror)
            {
              yyn = yytable[yyn];
              if (0 < yyn)
                break;
            }
        }

      /* Pop the current state because it cannot handle the error token.  */
      if (yyssp == yyss)
        YYABORT;


      yydestruct ("Error: popping",
                  YY_ACCESSING_SYMBOL (yystate), yyvsp);
      YYPOPSTACK (1);
      yystate = *yyssp;
      YY_STACK_PRINT (yyss, yyssp);
    }

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END


  /* Shift the error token.  */
  YY_SYMBOL_PRINT ("Shifting", YY_ACCESSING_SYMBOL (yyn), yyvsp, yylsp);

  yystate = yyn;
  goto yynewstate;


/*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
yyacceptlab:
  yyresult = 0;
  goto yyreturn;


/*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
yyabortlab:
  yyresult = 1;
  goto yyreturn;


#if !defined yyoverflow
/*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
yyexhaustedlab:
  yyerror (YY_("memory exhausted"));
  yyresult = 2;
  goto yyreturn;
#endif


/*-------------------------------------------------------.
| yyreturn -- parsing is finished, clean up and return.  |
`-------------------------------------------------------*/
yyreturn:
  if (yychar != TOKEN_YYEMPTY)
    {
      /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
      yytoken = YYTRANSLATE (yychar);
      yydestruct ("Cleanup: discarding lookahead",
                  yytoken, &yylval);
    }
  /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
  YYPOPSTACK (yylen);
  YY_STACK_PRINT (yyss, yyssp);
  while (yyssp != yyss)
    {
      yydestruct ("Cleanup: popping",
                  YY_ACCESSING_SYMBOL (+*yyssp), yyvsp);
      YYPOPSTACK (1);
    }
#ifndef yyoverflow
  if (yyss != yyssa)
    YYSTACK_FREE (yyss);
#endif

  return yyresult;
}


