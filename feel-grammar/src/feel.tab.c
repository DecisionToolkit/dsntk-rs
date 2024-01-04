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
    TOKEN_START_TEXTUAL_EXPRESSION = 260, /* START_TEXTUAL_EXPRESSION  */
    TOKEN_START_TEXTUAL_EXPRESSIONS = 261, /* START_TEXTUAL_EXPRESSIONS  */
    TOKEN_START_UNARY_TESTS = 262, /* START_UNARY_TESTS  */
    TOKEN_START_SIMPLE_EXPRESSION = 263, /* START_SIMPLE_EXPRESSION  */
    TOKEN_START_SIMPLE_EXPRESSIONS = 264, /* START_SIMPLE_EXPRESSIONS  */
    TOKEN_START_SIMPLE_VALUE = 265, /* START_SIMPLE_VALUE  */
    TOKEN_START_RANGE_LITERAL = 266, /* START_RANGE_LITERAL  */
    TOKEN_AT = 267,                /* AT  */
    TOKEN_NOT = 268,               /* NOT  */
    TOKEN_COLON = 269,             /* COLON  */
    TOKEN_COMMA = 270,             /* COMMA  */
    TOKEN_EVERY = 271,             /* EVERY  */
    TOKEN_FOR = 272,               /* FOR  */
    TOKEN_LEFT_BRACE = 273,        /* LEFT_BRACE  */
    TOKEN_NULL = 274,              /* NULL  */
    TOKEN_RIGHT_ARROW = 275,       /* RIGHT_ARROW  */
    TOKEN_OF = 276,                /* OF  */
    TOKEN_LIST = 277,              /* LIST  */
    TOKEN_RANGE = 278,             /* RANGE  */
    TOKEN_CONTEXT = 279,           /* CONTEXT  */
    TOKEN_THEN = 280,              /* THEN  */
    TOKEN_FUNCTION = 281,          /* FUNCTION  */
    TOKEN_EXTERNAL = 282,          /* EXTERNAL  */
    TOKEN_IF = 283,                /* IF  */
    TOKEN_RIGHT_BRACE = 284,       /* RIGHT_BRACE  */
    TOKEN_RETURN = 285,            /* RETURN  */
    TOKEN_SOME = 286,              /* SOME  */
    TOKEN_NUMERIC = 287,           /* NUMERIC  */
    TOKEN_STRING = 288,            /* STRING  */
    TOKEN_BOOLEAN = 289,           /* BOOLEAN  */
    TOKEN_SATISFIES = 290,         /* SATISFIES  */
    TOKEN_ELSE = 291,              /* ELSE  */
    TOKEN_OR = 292,                /* OR  */
    TOKEN_AND = 293,               /* AND  */
    TOKEN_EQ = 294,                /* EQ  */
    TOKEN_NE = 295,                /* NE  */
    TOKEN_LT = 296,                /* LT  */
    TOKEN_LE = 297,                /* LE  */
    TOKEN_GT = 298,                /* GT  */
    TOKEN_GE = 299,                /* GE  */
    TOKEN_BETWEEN = 300,           /* BETWEEN  */
    TOKEN_BETWEEN_AND = 301,       /* BETWEEN_AND  */
    TOKEN_IN = 302,                /* IN  */
    TOKEN_MINUS = 303,             /* MINUS  */
    TOKEN_PLUS = 304,              /* PLUS  */
    TOKEN_MUL = 305,               /* MUL  */
    TOKEN_DIV = 306,               /* DIV  */
    TOKEN_EXP = 307,               /* EXP  */
    TOKEN_PREC_NEG = 308,          /* PREC_NEG  */
    TOKEN_INSTANCE = 309,          /* INSTANCE  */
    TOKEN_NAME = 310,              /* NAME  */
    TOKEN_NAME_DATE_TIME = 311,    /* NAME_DATE_TIME  */
    TOKEN_BUILT_IN_TYPE_NAME = 312, /* BUILT_IN_TYPE_NAME  */
    TOKEN_LEFT_PAREN = 313,        /* LEFT_PAREN  */
    TOKEN_RIGHT_PAREN = 314,       /* RIGHT_PAREN  */
    TOKEN_LEFT_BRACKET = 315,      /* LEFT_BRACKET  */
    TOKEN_RIGHT_BRACKET = 316,     /* RIGHT_BRACKET  */
    TOKEN_ELLIPSIS = 317,          /* ELLIPSIS  */
    TOKEN_DOT = 318                /* DOT  */
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
  SYMBOL_START_TEXTUAL_EXPRESSION = 5,     /* START_TEXTUAL_EXPRESSION  */
  SYMBOL_START_TEXTUAL_EXPRESSIONS = 6,    /* START_TEXTUAL_EXPRESSIONS  */
  SYMBOL_START_UNARY_TESTS = 7,            /* START_UNARY_TESTS  */
  SYMBOL_START_SIMPLE_EXPRESSION = 8,      /* START_SIMPLE_EXPRESSION  */
  SYMBOL_START_SIMPLE_EXPRESSIONS = 9,     /* START_SIMPLE_EXPRESSIONS  */
  SYMBOL_START_SIMPLE_VALUE = 10,          /* START_SIMPLE_VALUE  */
  SYMBOL_START_RANGE_LITERAL = 11,         /* START_RANGE_LITERAL  */
  SYMBOL_AT = 12,                          /* AT  */
  SYMBOL_NOT = 13,                         /* NOT  */
  SYMBOL_COLON = 14,                       /* COLON  */
  SYMBOL_COMMA = 15,                       /* COMMA  */
  SYMBOL_EVERY = 16,                       /* EVERY  */
  SYMBOL_FOR = 17,                         /* FOR  */
  SYMBOL_LEFT_BRACE = 18,                  /* LEFT_BRACE  */
  SYMBOL_NULL = 19,                        /* NULL  */
  SYMBOL_RIGHT_ARROW = 20,                 /* RIGHT_ARROW  */
  SYMBOL_OF = 21,                          /* OF  */
  SYMBOL_LIST = 22,                        /* LIST  */
  SYMBOL_RANGE = 23,                       /* RANGE  */
  SYMBOL_CONTEXT = 24,                     /* CONTEXT  */
  SYMBOL_THEN = 25,                        /* THEN  */
  SYMBOL_FUNCTION = 26,                    /* FUNCTION  */
  SYMBOL_EXTERNAL = 27,                    /* EXTERNAL  */
  SYMBOL_IF = 28,                          /* IF  */
  SYMBOL_RIGHT_BRACE = 29,                 /* RIGHT_BRACE  */
  SYMBOL_RETURN = 30,                      /* RETURN  */
  SYMBOL_SOME = 31,                        /* SOME  */
  SYMBOL_NUMERIC = 32,                     /* NUMERIC  */
  SYMBOL_STRING = 33,                      /* STRING  */
  SYMBOL_BOOLEAN = 34,                     /* BOOLEAN  */
  SYMBOL_SATISFIES = 35,                   /* SATISFIES  */
  SYMBOL_ELSE = 36,                        /* ELSE  */
  SYMBOL_OR = 37,                          /* OR  */
  SYMBOL_AND = 38,                         /* AND  */
  SYMBOL_EQ = 39,                          /* EQ  */
  SYMBOL_NE = 40,                          /* NE  */
  SYMBOL_LT = 41,                          /* LT  */
  SYMBOL_LE = 42,                          /* LE  */
  SYMBOL_GT = 43,                          /* GT  */
  SYMBOL_GE = 44,                          /* GE  */
  SYMBOL_BETWEEN = 45,                     /* BETWEEN  */
  SYMBOL_BETWEEN_AND = 46,                 /* BETWEEN_AND  */
  SYMBOL_IN = 47,                          /* IN  */
  SYMBOL_MINUS = 48,                       /* MINUS  */
  SYMBOL_PLUS = 49,                        /* PLUS  */
  SYMBOL_MUL = 50,                         /* MUL  */
  SYMBOL_DIV = 51,                         /* DIV  */
  SYMBOL_EXP = 52,                         /* EXP  */
  SYMBOL_PREC_NEG = 53,                    /* PREC_NEG  */
  SYMBOL_INSTANCE = 54,                    /* INSTANCE  */
  SYMBOL_NAME = 55,                        /* NAME  */
  SYMBOL_NAME_DATE_TIME = 56,              /* NAME_DATE_TIME  */
  SYMBOL_BUILT_IN_TYPE_NAME = 57,          /* BUILT_IN_TYPE_NAME  */
  SYMBOL_LEFT_PAREN = 58,                  /* LEFT_PAREN  */
  SYMBOL_RIGHT_PAREN = 59,                 /* RIGHT_PAREN  */
  SYMBOL_LEFT_BRACKET = 60,                /* LEFT_BRACKET  */
  SYMBOL_RIGHT_BRACKET = 61,               /* RIGHT_BRACKET  */
  SYMBOL_ELLIPSIS = 62,                    /* ELLIPSIS  */
  SYMBOL_DOT = 63,                         /* DOT  */
  SYMBOL_YYACCEPT = 64,                    /* $accept  */
  SYMBOL_feel = 65,                        /* feel  */
  SYMBOL_66_1 = 66,                        /* $@1  */
  SYMBOL_expression = 67,                  /* expression  */
  SYMBOL_boxed_expression = 68,            /* boxed_expression  */
  SYMBOL_textual_expression = 69,          /* textual_expression  */
  SYMBOL_70_2 = 70,                        /* $@2  */
  SYMBOL_71_3 = 71,                        /* $@3  */
  SYMBOL_72_4 = 72,                        /* $@4  */
  SYMBOL_73_5 = 73,                        /* $@5  */
  SYMBOL_74_6 = 74,                        /* $@6  */
  SYMBOL_textual_expressions = 75,         /* textual_expressions  */
  SYMBOL_unary_tests = 76,                 /* unary_tests  */
  SYMBOL_positive_unary_tests = 77,        /* positive_unary_tests  */
  SYMBOL_comparison_in = 78,               /* comparison_in  */
  SYMBOL_simple_expression = 79,           /* simple_expression  */
  SYMBOL_simple_expressions = 80,          /* simple_expressions  */
  SYMBOL_simple_positive_unary_test = 81,  /* simple_positive_unary_test  */
  SYMBOL_interval = 82,                    /* interval  */
  SYMBOL_interval_start = 83,              /* interval_start  */
  SYMBOL_interval_end = 84,                /* interval_end  */
  SYMBOL_endpoint = 85,                    /* endpoint  */
  SYMBOL_simple_value = 86,                /* simple_value  */
  SYMBOL_literal = 87,                     /* literal  */
  SYMBOL_simple_literal = 88,              /* simple_literal  */
  SYMBOL_89_7 = 89,                        /* $@7  */
  SYMBOL_context = 90,                     /* context  */
  SYMBOL_91_8 = 91,                        /* $@8  */
  SYMBOL_context_entries = 92,             /* context_entries  */
  SYMBOL_context_entry = 93,               /* context_entry  */
  SYMBOL_context_entry_tail = 94,          /* context_entry_tail  */
  SYMBOL_key = 95,                         /* key  */
  SYMBOL_list = 96,                        /* list  */
  SYMBOL_list_items = 97,                  /* list_items  */
  SYMBOL_list_tail = 98,                   /* list_tail  */
  SYMBOL_parameters = 99,                  /* parameters  */
  SYMBOL_named_parameters = 100,           /* named_parameters  */
  SYMBOL_named_parameter = 101,            /* named_parameter  */
  SYMBOL_named_parameters_tail = 102,      /* named_parameters_tail  */
  SYMBOL_positional_parameters = 103,      /* positional_parameters  */
  SYMBOL_positional_parameters_tail = 104, /* positional_parameters_tail  */
  SYMBOL_qualified_name = 105,             /* qualified_name  */
  SYMBOL_type = 106,                       /* type  */
  SYMBOL_107_9 = 107,                      /* $@9  */
  SYMBOL_108_10 = 108,                     /* $@10  */
  SYMBOL_109_11 = 109,                     /* $@11  */
  SYMBOL_context_type_entries = 110,       /* context_type_entries  */
  SYMBOL_context_type_entry = 111,         /* context_type_entry  */
  SYMBOL_112_12 = 112,                     /* $@12  */
  SYMBOL_context_type_entry_tail = 113,    /* context_type_entry_tail  */
  SYMBOL_function_type_parameters = 114,   /* function_type_parameters  */
  SYMBOL_115_13 = 115,                     /* $@13  */
  SYMBOL_function_type_parameters_tail = 116, /* function_type_parameters_tail  */
  SYMBOL_117_14 = 117,                     /* $@14  */
  SYMBOL_iteration_contexts = 118,         /* iteration_contexts  */
  SYMBOL_iteration_context = 119,          /* iteration_context  */
  SYMBOL_120_15 = 120,                     /* $@15  */
  SYMBOL_121_16 = 121,                     /* $@16  */
  SYMBOL_iteration_context_value = 122,    /* iteration_context_value  */
  SYMBOL_quantified_expressions = 123,     /* quantified_expressions  */
  SYMBOL_quantified_expression = 124,      /* quantified_expression  */
  SYMBOL_125_17 = 125,                     /* $@17  */
  SYMBOL_126_18 = 126,                     /* $@18  */
  SYMBOL_function_definition = 127,        /* function_definition  */
  SYMBOL_128_19 = 128,                     /* $@19  */
  SYMBOL_formal_parameters = 129,          /* formal_parameters  */
  SYMBOL_130_20 = 130,                     /* $@20  */
  SYMBOL_formal_parameters_tail = 131,     /* formal_parameters_tail  */
  SYMBOL_132_21 = 132,                     /* $@21  */
  SYMBOL_formal_parameter = 133,           /* formal_parameter  */
  SYMBOL_134_22 = 134,                     /* $@22  */
  SYMBOL_external = 135,                   /* external  */
  SYMBOL_range_literal = 136,              /* range_literal  */
  SYMBOL_range_literal_start = 137,        /* range_literal_start  */
  SYMBOL_range_literal_end = 138,          /* range_literal_end  */
  SYMBOL_range_endpoint = 139,             /* range_endpoint  */
  SYMBOL_140_23 = 140                      /* $@23  */
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
#define YYFINAL  69
/* YYLAST -- Last index in YYTABLE.  */
#define YYLAST   1059

/* YYNTOKENS -- Number of terminals.  */
#define YYNTOKENS  64
/* YYNNTS -- Number of nonterminals.  */
#define YYNNTS  77
/* YYNRULES -- Number of rules.  */
#define YYNRULES  175
/* YYNSTATES -- Number of states.  */
#define YYNSTATES  329

/* YYMAXUTOK -- Last valid token kind.  */
#define YYMAXUTOK   318


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
      55,    56,    57,    58,    59,    60,    61,    62,    63
};

#if YYDEBUG
  /* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
static const yytype_int16 yyrline[] =
{
       0,    61,    61,    62,    63,    64,    65,    65,    66,    67,
      68,    69,    73,    74,    78,    79,    80,    84,    84,    85,
      86,    86,    87,    87,    88,    88,    89,    90,    91,    92,
      93,    94,    95,    96,    97,    98,    99,   100,   101,   102,
     103,   104,   105,   105,   106,   107,   108,   109,   110,   111,
     112,   116,   117,   121,   122,   123,   127,   128,   132,   136,
     137,   138,   139,   140,   141,   142,   146,   147,   151,   152,
     153,   154,   155,   156,   157,   161,   165,   166,   167,   171,
     172,   173,   177,   181,   182,   186,   187,   191,   192,   193,
     194,   195,   195,   199,   199,   203,   204,   208,   212,   213,
     217,   218,   222,   226,   227,   231,   232,   236,   237,   238,
     242,   246,   250,   251,   255,   259,   260,   264,   265,   269,
     270,   271,   271,   272,   272,   273,   274,   274,   278,   282,
     282,   286,   287,   291,   292,   292,   296,   297,   297,   301,
     302,   306,   306,   306,   310,   311,   315,   316,   320,   320,
     320,   324,   324,   328,   329,   329,   333,   334,   334,   338,
     338,   339,   343,   344,   348,   352,   353,   354,   358,   359,
     360,   364,   365,   366,   367,   367
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
  "START_BOXED_EXPRESSION", "START_TEXTUAL_EXPRESSION",
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
     315,   316,   317,   318
};
#endif

#define YYPACT_NINF (-181)

#define yypact_value_is_default(Yyn) \
  ((Yyn) == YYPACT_NINF)

#define YYTABLE_NINF (-119)

#define yytable_value_is_error(Yyn) \
  ((Yyn) == YYTABLE_NINF)

  /* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
static const yytype_int16 yypact[] =
{
    1048,   392,    89,   392,   392,  -181,   439,   439,    33,    59,
      29,   -15,  -181,  -181,  -181,  -181,   -17,   392,  -181,  -181,
    -181,  -181,   392,   392,   392,   392,   392,   392,   392,  -181,
      -9,   392,   486,   392,   867,  -181,  -181,  -181,  -181,   392,
    -181,  -181,  -181,  -181,  -181,   486,  -181,   867,    42,    25,
    -181,   248,   392,    23,   894,  -181,  -181,    28,  -181,    49,
    -181,    20,  -181,  -181,    14,    14,    14,  -181,    14,  -181,
    -181,  -181,  -181,   104,  -181,   704,  -181,    24,  -181,  -181,
    -181,  -181,  -181,  -181,    85,  -181,   759,    38,   705,   563,
      70,  -181,    76,   392,   392,   392,   392,   392,   392,   392,
     392,  -181,   533,   392,   392,   392,   392,   392,    69,   295,
     392,    91,  -181,    51,   563,   392,    82,   392,   656,  -181,
    -181,   121,    98,   392,   392,   392,   392,   392,   439,   111,
    -181,  -181,    96,   108,   115,   126,  -181,   101,   123,   151,
     127,   162,   182,   143,  -181,  -181,  -181,  -181,    19,   189,
      92,   392,   171,   295,  -181,  -181,   392,  -181,  -181,  -181,
    -181,   920,   945,   970,   970,   970,   970,   970,   970,   392,
     392,   987,   174,   174,   120,   120,    85,  -181,   194,  -181,
     594,  -181,  -181,     9,  -181,   786,  -181,  -181,  -181,  -181,
    -181,   392,   392,  -181,    21,   113,    22,   141,   142,  -181,
    -181,  -181,  -181,  -181,  -181,  -181,  -181,  -181,   392,  -181,
    -181,   392,  -181,  -181,   -16,  -181,  -181,   392,   195,  -181,
     345,  -181,   732,   392,  -181,   563,   813,   625,  -181,   163,
     392,   392,  -181,  -181,   156,  -181,  -181,  -181,   153,  -181,
     295,   867,  -181,   166,   867,  -181,   170,    19,   867,  -181,
     392,   867,  -181,    18,   392,   867,  -181,   392,   392,   186,
     190,   192,   199,  -181,  -181,  -181,   867,   594,   194,     9,
    -181,  -181,   392,   392,  -181,   163,   867,   168,  -181,  -181,
     867,   987,   176,  -181,  -181,   187,   198,  -181,  -181,   867,
     840,  -181,  -181,  -181,  -181,   163,   163,   229,  -181,    -8,
    -181,   224,   163,   392,    18,   203,   204,  -181,   187,  -181,
    -181,  -181,    17,   867,  -181,  -181,  -181,   163,    -8,   163,
    -181,  -181,  -181,  -181,  -181,  -181,   163,    17,  -181
};

  /* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
static const yytype_uint8 yydefact[] =
{
       0,     0,     0,     0,     0,     6,     0,     0,     0,     0,
       0,     0,    22,    17,    93,    86,     0,     0,    20,    87,
      88,    89,     0,     0,     0,     0,     0,     0,     0,    49,
       0,     0,     0,     0,     2,    12,    13,    48,    74,     0,
      47,    85,    16,    14,    15,     0,     3,     0,    13,    13,
       5,     0,     0,    49,     0,     8,    65,    85,    83,    67,
       9,   118,    10,    84,     0,     0,     0,    11,     0,     1,
      90,   148,   141,     0,   151,     0,   148,    82,    72,    73,
      68,    69,    70,    71,    41,    91,    82,     0,   103,    82,
       0,   102,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    24,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    75,     0,     0,     0,     0,    53,    57,     7,
      55,    41,     0,     0,     0,     0,     0,     0,     0,     0,
     171,   172,     0,     0,     0,     0,   164,     0,     0,   147,
       0,     0,   140,     0,    95,   101,   100,    94,     0,     0,
       0,     0,     0,     0,    50,    76,     0,   105,   104,    78,
      77,    26,    27,    28,    29,    30,    31,    32,    33,     0,
       0,    35,    37,    36,    38,    39,    40,    42,    49,   107,
       0,    46,   108,     0,   109,     0,    44,    79,    80,    81,
      51,     0,     0,   117,    37,    36,    38,    39,    40,    66,
     173,   174,   165,   167,   166,   168,   169,   170,     0,   148,
     149,     0,   141,   142,     0,    98,    96,     0,   161,   153,
       0,   154,     0,     0,    92,     0,     0,    82,    34,     0,
       0,     0,   115,   114,     0,   112,   110,    45,     0,    56,
       0,    23,   146,     0,    18,   139,     0,     0,    97,   159,
       0,   163,   152,     0,     0,    21,   106,     0,     0,     0,
       0,     0,     0,   119,   120,    43,   111,     0,     0,     0,
      54,   175,     0,     0,    99,     0,   162,     0,   156,   155,
      19,    25,     0,   121,   123,     0,   134,   116,   113,   150,
     144,   143,   160,   157,    58,     0,     0,     0,   125,     0,
     133,     0,     0,     0,     0,     0,     0,   129,     0,   131,
     128,   126,     0,   145,   158,   122,   124,     0,     0,     0,
     137,   136,   135,   130,   132,   127,     0,     0,   138
};

  /* YYPGOTO[NTERM-NUM].  */
static const yytype_int16 yypgoto[] =
{
    -181,  -181,  -181,    -1,   246,    16,  -181,  -181,  -181,  -181,
    -181,   135,  -181,  -180,  -181,   245,   124,  -181,  -181,  -181,
    -181,    30,   247,  -181,     2,  -181,  -181,  -181,  -181,    40,
      11,  -181,  -181,  -181,    34,  -149,  -181,    35,    -6,  -181,
      -5,     7,   -81,  -181,  -181,  -181,  -181,   -40,  -181,   -48,
    -181,  -181,   -54,  -181,    63,  -181,  -181,  -181,  -181,   -75,
    -181,  -181,  -181,  -181,  -181,  -181,  -181,   -27,  -181,     1,
    -181,  -181,  -181,  -181,  -181,    -7,  -181
};

  /* YYDEFGOTO[NTERM-NUM].  */
static const yytype_int16 yydefgoto[] =
{
       0,    10,    51,    77,    35,    36,    72,    76,    71,   169,
     229,    50,   119,   120,   228,    59,    60,    37,    38,    39,
     112,    87,    56,    40,    41,   153,    42,    73,   147,   148,
     216,   149,    43,    91,   158,   181,   182,   183,   236,   184,
     233,   264,   265,   295,   296,   319,   298,   299,   317,   310,
     301,   302,   322,   326,   141,   142,   143,   246,   291,   138,
     139,   140,   243,    44,   150,   220,   253,   279,   304,   221,
     275,   252,    67,    68,   136,   133,   240
};

  /* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
static const yytype_int16 yytable[] =
{
      34,   152,    47,    47,   224,    54,    54,   308,    57,    57,
      63,   238,   239,    58,    58,    58,    75,   145,    70,    48,
      49,   -60,   -61,  -118,   234,   -52,   129,    84,   -84,    69,
      86,    89,   320,   277,   214,   309,   -60,   -61,  -118,   146,
     115,    74,    -4,   -84,   114,    11,   130,   131,   215,    85,
     118,   121,    78,    79,    80,    81,    82,    83,   134,   135,
     321,   137,    90,    92,   128,    19,    20,    21,   235,   113,
     132,   105,   106,   107,   107,   108,   108,   278,   282,   109,
     109,   110,   110,   122,   111,   111,   122,   111,    61,    30,
     177,   271,   161,   162,   163,   164,   165,   166,   167,   168,
     155,   171,   172,   173,   174,   175,   176,    14,   180,   185,
     187,   188,   189,   -59,    47,    16,    84,    64,    92,    65,
      66,   -64,   194,   195,   196,   197,   198,    54,   -59,   193,
      57,    49,   159,   144,   242,    58,   -64,   145,   160,   108,
     191,   -62,   -63,   109,   200,   110,   186,   218,   111,    45,
     222,   219,   180,    61,   201,   225,   -62,   -63,   208,   146,
     205,   206,   207,   105,   106,   107,   209,   108,   226,   227,
     202,   109,   107,   110,   108,   108,   111,   203,   109,   109,
     110,   110,   210,   111,   111,   259,   260,   261,   204,   262,
     118,   118,   211,   107,   292,   108,   108,   212,   213,   109,
     109,   110,   110,   217,   111,   111,   223,   241,   230,   249,
     244,   268,   270,   272,   305,   306,   248,   273,    61,   251,
     263,   312,   255,   218,   105,   106,   107,   283,   108,   266,
     267,   284,   109,   285,   110,   294,   323,   111,   325,   180,
     286,   300,   297,   307,   311,   327,   315,   316,    46,   276,
     190,    55,   199,   280,   247,    62,   281,   118,   274,   256,
      11,   116,   287,   288,    12,    13,    14,    15,   318,   269,
     324,   289,   290,   328,    16,   245,    17,   314,   293,    18,
      19,    20,    21,     0,     0,     0,     0,    22,    23,    24,
      25,    26,    27,     0,     0,     0,   117,     0,     0,     0,
       0,     0,   313,    29,    30,     0,    31,    11,    32,    33,
       0,    12,    13,    14,    15,     0,     0,     0,     0,     0,
       0,    16,     0,    17,     0,     0,    18,    19,    20,    21,
       0,     0,     0,     0,    22,    23,    24,    25,    26,    27,
       0,     0,     0,    28,     0,     0,     0,     0,     0,     0,
     178,    30,     0,    31,   179,    32,    33,    11,     0,     0,
       0,    12,    13,    14,    15,     0,     0,     0,     0,     0,
       0,    16,   250,    17,     0,     0,    18,    19,    20,    21,
       0,     0,     0,     0,    22,    23,    24,    25,    26,    27,
       0,     0,     0,    28,     0,     0,     0,     0,     0,     0,
      29,    30,     0,    31,    11,    32,    33,     0,    12,    13,
      14,    15,     0,     0,     0,     0,     0,     0,    16,     0,
      17,     0,     0,    18,    19,    20,    21,     0,     0,     0,
       0,    22,    23,    24,    25,    26,    27,     0,     0,     0,
      28,     0,     0,     0,     0,     0,     0,    29,    30,     0,
      31,    11,    32,    33,     0,    12,    13,    14,    15,     0,
       0,     0,     0,     0,     0,    16,     0,    17,     0,     0,
      18,    19,    20,    21,     0,     0,     0,     0,    22,    23,
      24,    25,    26,    27,     0,     0,     0,    52,     0,     0,
       0,     0,     0,     0,    53,    30,     0,    31,    11,    32,
      33,     0,    12,    13,    14,    15,     0,     0,     0,     0,
       0,     0,    16,     0,    17,     0,     0,    18,    19,    20,
      21,     0,     0,     0,     0,    22,    23,    24,    25,    26,
      27,     0,     0,     0,    28,     0,     0,     0,     0,     0,
       0,    29,    30,     0,    31,    11,    32,    88,     0,    12,
      13,    14,    15,     0,     0,     0,     0,     0,     0,    16,
       0,    17,     0,     0,    18,    19,    20,    21,     0,     0,
       0,     0,    22,    23,    24,    25,    26,    27,   156,     0,
       0,    28,     0,     0,     0,     0,     0,     0,    29,    30,
       0,   170,     0,    32,    33,     0,     0,     0,     0,     0,
      93,    94,    95,    96,    97,    98,    99,   100,   101,   231,
     102,   103,   104,   105,   106,   107,     0,   108,     0,     0,
       0,   109,     0,   110,   157,     0,   111,     0,     0,     0,
       0,    93,    94,    95,    96,    97,    98,    99,   100,   101,
     258,   102,   103,   104,   105,   106,   107,     0,   108,     0,
       0,     0,   109,   232,   110,     0,     0,   111,     0,     0,
       0,     0,    93,    94,    95,    96,    97,    98,    99,   100,
     101,   192,   102,   103,   104,   105,   106,   107,     0,   108,
       0,     0,     0,   109,   154,   110,     0,     0,   111,     0,
       0,     0,     0,    93,    94,    95,    96,    97,    98,    99,
     100,   101,     0,   102,   103,   104,   105,   106,   107,     0,
     108,     0,     0,     0,   109,     0,   110,    11,     0,   111,
       0,    12,    13,    14,    15,     0,     0,     0,     0,   151,
       0,    16,     0,    17,     0,     0,    18,    19,    20,    21,
       0,    93,    94,    95,    96,    97,    98,    99,   100,   101,
       0,   102,   103,   104,   105,   106,   107,     0,   108,     0,
      29,    30,   109,     0,   110,     0,     0,   111,   254,    93,
      94,    95,    96,    97,    98,    99,   100,   101,     0,   102,
     103,   104,   105,   106,   107,     0,   108,     0,     0,     0,
     109,     0,   110,     0,     0,   111,    93,    94,    95,    96,
      97,    98,    99,   100,   101,     0,   102,   103,   104,   105,
     106,   107,     0,   108,     0,     0,     0,   109,   154,   110,
       0,     0,   111,    93,    94,    95,    96,    97,    98,    99,
     100,   101,     0,   102,   103,   104,   105,   106,   107,     0,
     108,     0,     0,     0,   109,     0,   110,   237,     0,   111,
      93,    94,    95,    96,    97,    98,    99,   100,   101,   257,
     102,   103,   104,   105,   106,   107,     0,   108,     0,     0,
       0,   109,     0,   110,     0,     0,   111,    93,    94,    95,
      96,    97,    98,    99,   100,   101,     0,   102,   103,   104,
     105,   106,   107,     0,   108,     0,     0,     0,   109,     0,
     110,     0,   303,   111,    93,    94,    95,    96,    97,    98,
      99,   100,   101,     0,   102,   103,   104,   105,   106,   107,
       0,   108,     0,     0,     0,   109,     0,   110,     0,     0,
     111,    93,    94,    95,    96,    97,    98,    99,   100,   101,
       0,   102,   123,   124,   125,   126,   127,     0,   108,     0,
       0,     0,   109,     0,   110,     0,     0,   111,    94,    95,
      96,    97,    98,    99,   100,   101,     0,   102,   103,   104,
     105,   106,   107,     0,   108,     0,     0,     0,   109,     0,
     110,     0,     0,   111,    95,    96,    97,    98,    99,   100,
     101,     0,   102,   103,   104,   105,   106,   107,     0,   108,
       0,     0,     0,   109,     0,   110,     0,     0,   111,  -119,
    -119,  -119,  -119,  -119,  -119,   101,     0,   102,   103,   104,
     105,   106,   107,     0,   108,     0,     0,     0,   109,     0,
     110,     0,     0,   111,   102,   103,   104,   105,   106,   107,
       0,   108,     0,     0,     0,   109,     0,   110,     0,     0,
     111,     1,     2,     3,     4,     5,     6,     7,     8,     9
};

static const yytype_int16 yycheck[] =
{
       1,    76,     3,     4,   153,     6,     7,    15,     6,     7,
       8,   191,   192,     6,     7,     8,    17,    33,    33,     3,
       4,     0,     0,     0,    15,     0,    12,    28,     0,     0,
      31,    32,    15,    15,    15,    43,    15,    15,    15,    55,
      15,    58,     0,    15,    45,    12,    32,    33,    29,    58,
      51,    52,    22,    23,    24,    25,    26,    27,    65,    66,
      43,    68,    32,    33,    15,    32,    33,    34,    59,    39,
      56,    50,    51,    52,    52,    54,    54,    59,   258,    58,
      58,    60,    60,    63,    63,    63,    63,    63,    55,    56,
      21,   240,    93,    94,    95,    96,    97,    98,    99,   100,
      62,   102,   103,   104,   105,   106,   107,    18,   109,   110,
      59,    60,    61,     0,   115,    26,   117,    58,    88,    60,
      61,     0,   123,   124,   125,   126,   127,   128,    15,   122,
     128,   115,    62,    29,   209,   128,    15,    33,    62,    54,
      58,     0,     0,    58,    33,    60,    55,    55,    63,    60,
     151,    59,   153,    55,    58,   156,    15,    15,    35,    55,
      59,    60,    61,    50,    51,    52,    15,    54,   169,   170,
      62,    58,    52,    60,    54,    54,    63,    62,    58,    58,
      60,    60,    55,    63,    63,    22,    23,    24,    62,    26,
     191,   192,    30,    52,   275,    54,    54,    15,    55,    58,
      58,    60,    60,    14,    63,    63,    35,   208,    14,    14,
     211,    55,    59,    47,   295,   296,   217,    47,    55,   220,
      57,   302,   223,    55,    50,    51,    52,    41,    54,   230,
     231,    41,    58,    41,    60,    59,   317,    63,   319,   240,
      41,    43,    55,    14,    20,   326,    43,    43,     2,   250,
     115,     6,   128,   254,   214,     8,   257,   258,   247,   225,
      12,    13,   267,   269,    16,    17,    18,    19,   308,   234,
     318,   272,   273,   327,    26,   212,    28,   304,   277,    31,
      32,    33,    34,    -1,    -1,    -1,    -1,    39,    40,    41,
      42,    43,    44,    -1,    -1,    -1,    48,    -1,    -1,    -1,
      -1,    -1,   303,    55,    56,    -1,    58,    12,    60,    61,
      -1,    16,    17,    18,    19,    -1,    -1,    -1,    -1,    -1,
      -1,    26,    -1,    28,    -1,    -1,    31,    32,    33,    34,
      -1,    -1,    -1,    -1,    39,    40,    41,    42,    43,    44,
      -1,    -1,    -1,    48,    -1,    -1,    -1,    -1,    -1,    -1,
      55,    56,    -1,    58,    59,    60,    61,    12,    -1,    -1,
      -1,    16,    17,    18,    19,    -1,    -1,    -1,    -1,    -1,
      -1,    26,    27,    28,    -1,    -1,    31,    32,    33,    34,
      -1,    -1,    -1,    -1,    39,    40,    41,    42,    43,    44,
      -1,    -1,    -1,    48,    -1,    -1,    -1,    -1,    -1,    -1,
      55,    56,    -1,    58,    12,    60,    61,    -1,    16,    17,
      18,    19,    -1,    -1,    -1,    -1,    -1,    -1,    26,    -1,
      28,    -1,    -1,    31,    32,    33,    34,    -1,    -1,    -1,
      -1,    39,    40,    41,    42,    43,    44,    -1,    -1,    -1,
      48,    -1,    -1,    -1,    -1,    -1,    -1,    55,    56,    -1,
      58,    12,    60,    61,    -1,    16,    17,    18,    19,    -1,
      -1,    -1,    -1,    -1,    -1,    26,    -1,    28,    -1,    -1,
      31,    32,    33,    34,    -1,    -1,    -1,    -1,    39,    40,
      41,    42,    43,    44,    -1,    -1,    -1,    48,    -1,    -1,
      -1,    -1,    -1,    -1,    55,    56,    -1,    58,    12,    60,
      61,    -1,    16,    17,    18,    19,    -1,    -1,    -1,    -1,
      -1,    -1,    26,    -1,    28,    -1,    -1,    31,    32,    33,
      34,    -1,    -1,    -1,    -1,    39,    40,    41,    42,    43,
      44,    -1,    -1,    -1,    48,    -1,    -1,    -1,    -1,    -1,
      -1,    55,    56,    -1,    58,    12,    60,    61,    -1,    16,
      17,    18,    19,    -1,    -1,    -1,    -1,    -1,    -1,    26,
      -1,    28,    -1,    -1,    31,    32,    33,    34,    -1,    -1,
      -1,    -1,    39,    40,    41,    42,    43,    44,    15,    -1,
      -1,    48,    -1,    -1,    -1,    -1,    -1,    -1,    55,    56,
      -1,    58,    -1,    60,    61,    -1,    -1,    -1,    -1,    -1,
      37,    38,    39,    40,    41,    42,    43,    44,    45,    15,
      47,    48,    49,    50,    51,    52,    -1,    54,    -1,    -1,
      -1,    58,    -1,    60,    61,    -1,    63,    -1,    -1,    -1,
      -1,    37,    38,    39,    40,    41,    42,    43,    44,    45,
      15,    47,    48,    49,    50,    51,    52,    -1,    54,    -1,
      -1,    -1,    58,    59,    60,    -1,    -1,    63,    -1,    -1,
      -1,    -1,    37,    38,    39,    40,    41,    42,    43,    44,
      45,    15,    47,    48,    49,    50,    51,    52,    -1,    54,
      -1,    -1,    -1,    58,    59,    60,    -1,    -1,    63,    -1,
      -1,    -1,    -1,    37,    38,    39,    40,    41,    42,    43,
      44,    45,    -1,    47,    48,    49,    50,    51,    52,    -1,
      54,    -1,    -1,    -1,    58,    -1,    60,    12,    -1,    63,
      -1,    16,    17,    18,    19,    -1,    -1,    -1,    -1,    25,
      -1,    26,    -1,    28,    -1,    -1,    31,    32,    33,    34,
      -1,    37,    38,    39,    40,    41,    42,    43,    44,    45,
      -1,    47,    48,    49,    50,    51,    52,    -1,    54,    -1,
      55,    56,    58,    -1,    60,    -1,    -1,    63,    36,    37,
      38,    39,    40,    41,    42,    43,    44,    45,    -1,    47,
      48,    49,    50,    51,    52,    -1,    54,    -1,    -1,    -1,
      58,    -1,    60,    -1,    -1,    63,    37,    38,    39,    40,
      41,    42,    43,    44,    45,    -1,    47,    48,    49,    50,
      51,    52,    -1,    54,    -1,    -1,    -1,    58,    59,    60,
      -1,    -1,    63,    37,    38,    39,    40,    41,    42,    43,
      44,    45,    -1,    47,    48,    49,    50,    51,    52,    -1,
      54,    -1,    -1,    -1,    58,    -1,    60,    61,    -1,    63,
      37,    38,    39,    40,    41,    42,    43,    44,    45,    46,
      47,    48,    49,    50,    51,    52,    -1,    54,    -1,    -1,
      -1,    58,    -1,    60,    -1,    -1,    63,    37,    38,    39,
      40,    41,    42,    43,    44,    45,    -1,    47,    48,    49,
      50,    51,    52,    -1,    54,    -1,    -1,    -1,    58,    -1,
      60,    -1,    62,    63,    37,    38,    39,    40,    41,    42,
      43,    44,    45,    -1,    47,    48,    49,    50,    51,    52,
      -1,    54,    -1,    -1,    -1,    58,    -1,    60,    -1,    -1,
      63,    37,    38,    39,    40,    41,    42,    43,    44,    45,
      -1,    47,    48,    49,    50,    51,    52,    -1,    54,    -1,
      -1,    -1,    58,    -1,    60,    -1,    -1,    63,    38,    39,
      40,    41,    42,    43,    44,    45,    -1,    47,    48,    49,
      50,    51,    52,    -1,    54,    -1,    -1,    -1,    58,    -1,
      60,    -1,    -1,    63,    39,    40,    41,    42,    43,    44,
      45,    -1,    47,    48,    49,    50,    51,    52,    -1,    54,
      -1,    -1,    -1,    58,    -1,    60,    -1,    -1,    63,    39,
      40,    41,    42,    43,    44,    45,    -1,    47,    48,    49,
      50,    51,    52,    -1,    54,    -1,    -1,    -1,    58,    -1,
      60,    -1,    -1,    63,    47,    48,    49,    50,    51,    52,
      -1,    54,    -1,    -1,    -1,    58,    -1,    60,    -1,    -1,
      63,     3,     4,     5,     6,     7,     8,     9,    10,    11
};

  /* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static const yytype_uint8 yystos[] =
{
       0,     3,     4,     5,     6,     7,     8,     9,    10,    11,
      65,    12,    16,    17,    18,    19,    26,    28,    31,    32,
      33,    34,    39,    40,    41,    42,    43,    44,    48,    55,
      56,    58,    60,    61,    67,    68,    69,    81,    82,    83,
      87,    88,    90,    96,   127,    60,    68,    67,    69,    69,
      75,    66,    48,    55,    67,    79,    86,    88,   105,    79,
      80,    55,    86,    88,    58,    60,    61,   136,   137,     0,
      33,    72,    70,    91,    58,    67,    71,    67,    85,    85,
      85,    85,    85,    85,    67,    58,    67,    85,    61,    67,
      85,    97,    85,    37,    38,    39,    40,    41,    42,    43,
      44,    45,    47,    48,    49,    50,    51,    52,    54,    58,
      60,    63,    84,    85,    67,    15,    13,    48,    67,    76,
      77,    67,    63,    48,    49,    50,    51,    52,    15,    12,
      32,    33,    56,   139,   139,   139,   138,   139,   123,   124,
     125,   118,   119,   120,    29,    33,    55,    92,    93,    95,
     128,    25,   123,    89,    59,    62,    15,    61,    98,    62,
      62,    67,    67,    67,    67,    67,    67,    67,    67,    73,
      58,    67,    67,    67,    67,    67,    67,    21,    55,    59,
      67,    99,   100,   101,   103,    67,    55,    59,    60,    61,
      75,    58,    15,   105,    67,    67,    67,    67,    67,    80,
      33,    58,    62,    62,    62,    59,    60,    61,    35,    15,
      55,    30,    15,    55,    15,    29,    94,    14,    55,    59,
     129,   133,    67,    35,    99,    67,    67,    67,    78,    74,
      14,    15,    59,   104,    15,    59,   102,    61,    77,    77,
     140,    67,   123,   126,    67,   118,   121,    93,    67,    14,
      27,    67,   135,   130,    36,    67,    98,    46,    15,    22,
      23,    24,    26,    57,   105,   106,    67,    67,    55,   101,
      59,    99,    47,    47,    94,   134,    67,    15,    59,   131,
      67,    67,    77,    41,    41,    41,    41,   104,   102,    67,
      67,   122,   106,   133,    59,   107,   108,    55,   110,   111,
      43,   114,   115,    62,   132,   106,   106,    14,    15,    43,
     113,    20,   106,    67,   131,    43,    43,   112,   111,   109,
      15,    43,   116,   106,   113,   106,   117,   106,   116
};

  /* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static const yytype_uint8 yyr1[] =
{
       0,    64,    65,    65,    65,    65,    66,    65,    65,    65,
      65,    65,    67,    67,    68,    68,    68,    70,    69,    69,
      71,    69,    72,    69,    73,    69,    69,    69,    69,    69,
      69,    69,    69,    69,    69,    69,    69,    69,    69,    69,
      69,    69,    74,    69,    69,    69,    69,    69,    69,    69,
      69,    75,    75,    76,    76,    76,    77,    77,    78,    79,
      79,    79,    79,    79,    79,    79,    80,    80,    81,    81,
      81,    81,    81,    81,    81,    82,    83,    83,    83,    84,
      84,    84,    85,    86,    86,    87,    87,    88,    88,    88,
      88,    89,    88,    91,    90,    92,    92,    93,    94,    94,
      95,    95,    96,    97,    97,    98,    98,    99,    99,    99,
     100,   101,   102,   102,   103,   104,   104,   105,   105,   106,
     106,   107,   106,   108,   106,   106,   109,   106,   110,   112,
     111,   113,   113,   114,   115,   114,   116,   117,   116,   118,
     118,   120,   121,   119,   122,   122,   123,   123,   125,   126,
     124,   128,   127,   129,   130,   129,   131,   132,   131,   134,
     133,   133,   135,   135,   136,   137,   137,   137,   138,   138,
     138,   139,   139,   139,   140,   139
};

  /* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static const yytype_int8 yyr2[] =
{
       0,     2,     2,     2,     2,     2,     0,     3,     2,     2,
       2,     2,     1,     1,     1,     1,     1,     0,     5,     6,
       0,     5,     0,     5,     0,     6,     3,     3,     3,     3,
       3,     3,     3,     3,     4,     3,     3,     3,     3,     3,
       3,     2,     0,     5,     3,     4,     3,     1,     1,     1,
       3,     3,     1,     1,     4,     1,     3,     1,     4,     3,
       3,     3,     3,     3,     2,     1,     3,     1,     2,     2,
       2,     2,     2,     2,     1,     2,     3,     3,     3,     2,
       2,     2,     1,     1,     1,     1,     1,     1,     1,     1,
       2,     0,     4,     0,     3,     1,     2,     3,     1,     3,
       1,     1,     2,     1,     2,     1,     3,     1,     1,     1,
       2,     3,     1,     3,     2,     1,     3,     3,     1,     1,
       1,     0,     5,     0,     5,     3,     0,     6,     2,     0,
       4,     1,     3,     1,     0,     3,     1,     0,     4,     3,
       1,     0,     0,     5,     1,     3,     3,     1,     0,     0,
       5,     0,     5,     1,     0,     3,     1,     0,     4,     0,
       4,     1,     2,     1,     2,     3,     3,     3,     2,     2,
       2,     1,     1,     2,     0,     4
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
  case 6: /* $@1: %empty  */
                      {/* unary_tests_begin */}
    break;

  case 17: /* $@2: %empty  */
        {/* for_begin */}
    break;

  case 18: /* textual_expression: FOR $@2 iteration_contexts RETURN expression  */
                                                               {/* for */}
    break;

  case 19: /* textual_expression: IF expression THEN expression ELSE expression  */
                                                  {/* if */}
    break;

  case 20: /* $@3: %empty  */
         {/* some_begin */}
    break;

  case 21: /* textual_expression: SOME $@3 quantified_expressions SATISFIES expression  */
                                                                        {/* some */}
    break;

  case 22: /* $@4: %empty  */
          {/* every_begin */}
    break;

  case 23: /* textual_expression: EVERY $@4 quantified_expressions SATISFIES expression  */
                                                                          {/* every */}
    break;

  case 24: /* $@5: %empty  */
                       {/* between_begin */}
    break;

  case 25: /* textual_expression: expression BETWEEN $@5 expression BETWEEN_AND expression  */
                                                                               {/* between */}
    break;

  case 26: /* textual_expression: expression OR expression  */
                             {/* disjunction */}
    break;

  case 27: /* textual_expression: expression AND expression  */
                              {/* conjunction */}
    break;

  case 28: /* textual_expression: expression EQ expression  */
                             {/* comparison_eq */}
    break;

  case 29: /* textual_expression: expression NE expression  */
                             {/* comparison_ne */}
    break;

  case 30: /* textual_expression: expression LT expression  */
                             {/* comparison_lt */}
    break;

  case 31: /* textual_expression: expression LE expression  */
                             {/* comparison_le */}
    break;

  case 32: /* textual_expression: expression GT expression  */
                             {/* comparison_gt */}
    break;

  case 33: /* textual_expression: expression GE expression  */
                             {/* comparison_ge */}
    break;

  case 34: /* textual_expression: expression IN LEFT_PAREN comparison_in  */
                                           {/* comparison_in */}
    break;

  case 35: /* textual_expression: expression IN expression  */
                             {/* comparison_in */}
    break;

  case 36: /* textual_expression: expression PLUS expression  */
                               {/* addition */}
    break;

  case 37: /* textual_expression: expression MINUS expression  */
                                {/* subtraction */}
    break;

  case 38: /* textual_expression: expression MUL expression  */
                              {/* multiplication */}
    break;

  case 39: /* textual_expression: expression DIV expression  */
                              {/* division */}
    break;

  case 40: /* textual_expression: expression EXP expression  */
                              {/* exponentiation */}
    break;

  case 41: /* textual_expression: MINUS expression  */
                                    {/* negation */}
    break;

  case 42: /* $@6: %empty  */
                           {/* type_name */}
    break;

  case 43: /* textual_expression: expression INSTANCE OF $@6 type  */
                                                  {/* instance_of */}
    break;

  case 44: /* textual_expression: expression DOT NAME  */
                        {/* path */}
    break;

  case 45: /* textual_expression: expression LEFT_BRACKET expression RIGHT_BRACKET  */
                                                     {/* filter */}
    break;

  case 49: /* textual_expression: NAME  */
         {/* name */}
    break;

  case 51: /* textual_expressions: textual_expression COMMA textual_expressions  */
                                                 {/* expression_list_tail */}
    break;

  case 52: /* textual_expressions: textual_expression  */
                       {/* expression_list_tail */}
    break;

  case 53: /* unary_tests: MINUS  */
          {/* unary_tests_irrelevant */}
    break;

  case 54: /* unary_tests: NOT LEFT_PAREN positive_unary_tests RIGHT_PAREN  */
                                                    {/* unary_tests_negated */}
    break;

  case 56: /* positive_unary_tests: expression COMMA positive_unary_tests  */
                                          {/* expression_list_tail */}
    break;

  case 57: /* positive_unary_tests: expression  */
               {/* expression_list_tail */}
    break;

  case 58: /* comparison_in: expression COMMA positive_unary_tests RIGHT_PAREN  */
                                                      {/* expression_list_tail */}
    break;

  case 59: /* simple_expression: expression PLUS expression  */
                               {/* addition */}
    break;

  case 60: /* simple_expression: expression MINUS expression  */
                                {/* subtraction */}
    break;

  case 61: /* simple_expression: expression MUL expression  */
                              {/* multiplication */}
    break;

  case 62: /* simple_expression: expression DIV expression  */
                              {/* division */}
    break;

  case 63: /* simple_expression: expression EXP expression  */
                              {/* exponentiation */}
    break;

  case 64: /* simple_expression: MINUS expression  */
                                    {/* negation */}
    break;

  case 66: /* simple_expressions: simple_expression COMMA simple_expressions  */
                                               {/* expression_list_tail */}
    break;

  case 67: /* simple_expressions: simple_expression  */
                      {/* expression_list_tail */}
    break;

  case 68: /* simple_positive_unary_test: LT endpoint  */
                {/* comparison_unary_lt */}
    break;

  case 69: /* simple_positive_unary_test: LE endpoint  */
                {/* comparison_unary_le */}
    break;

  case 70: /* simple_positive_unary_test: GT endpoint  */
                {/* comparison_unary_gt */}
    break;

  case 71: /* simple_positive_unary_test: GE endpoint  */
                {/* comparison_unary_ge */}
    break;

  case 72: /* simple_positive_unary_test: EQ endpoint  */
                {/* comparison_unary_eq */}
    break;

  case 73: /* simple_positive_unary_test: NE endpoint  */
                {/* comparison_unary_ne */}
    break;

  case 75: /* interval: interval_start interval_end  */
                                {/* interval */}
    break;

  case 76: /* interval_start: LEFT_PAREN endpoint ELLIPSIS  */
                                 {/* interval_start */}
    break;

  case 77: /* interval_start: RIGHT_BRACKET endpoint ELLIPSIS  */
                                    {/* interval_start */}
    break;

  case 78: /* interval_start: LEFT_BRACKET endpoint ELLIPSIS  */
                                   {/* interval_start */}
    break;

  case 79: /* interval_end: endpoint RIGHT_PAREN  */
                         {/* interval_end */}
    break;

  case 80: /* interval_end: endpoint LEFT_BRACKET  */
                          {/* interval_end */}
    break;

  case 81: /* interval_end: endpoint RIGHT_BRACKET  */
                           {/* interval_end */}
    break;

  case 86: /* literal: NULL  */
         {/* literal_null */}
    break;

  case 87: /* simple_literal: NUMERIC  */
            {/* literal_numeric */}
    break;

  case 88: /* simple_literal: STRING  */
           {/* literal_string */}
    break;

  case 89: /* simple_literal: BOOLEAN  */
            {/* literal_boolean */}
    break;

  case 90: /* simple_literal: AT STRING  */
              {/* literal_at */}
    break;

  case 91: /* $@7: %empty  */
                              {/* literal_date_time */}
    break;

  case 93: /* $@8: %empty  */
               {/* context_begin */}
    break;

  case 94: /* context: LEFT_BRACE $@8 context_entries  */
                                                     {/* context_end */}
    break;

  case 95: /* context_entries: RIGHT_BRACE  */
                {/* empty_context */}
    break;

  case 96: /* context_entries: context_entry context_entry_tail  */
                                     {/* context_entry_tail */}
    break;

  case 97: /* context_entry: key COLON expression  */
                         {/* context_entry */}
    break;

  case 99: /* context_entry_tail: COMMA context_entry context_entry_tail  */
                                           {/* context_entry_tail */}
    break;

  case 100: /* key: NAME  */
         {/* key_name */}
    break;

  case 101: /* key: STRING  */
           {/* key_string */}
    break;

  case 102: /* list: LEFT_BRACKET list_items  */
                            {/* list */}
    break;

  case 103: /* list_items: RIGHT_BRACKET  */
                            {/* list_empty */}
    break;

  case 104: /* list_items: expression list_tail  */
                         {/* list_tail */}
    break;

  case 106: /* list_tail: COMMA expression list_tail  */
                               {/* list_tail */}
    break;

  case 107: /* parameters: RIGHT_PAREN  */
                {/* function_invocation_no_parameters */}
    break;

  case 108: /* parameters: named_parameters  */
                     {/* function_invocation */}
    break;

  case 109: /* parameters: positional_parameters  */
                          {/* function_invocation */}
    break;

  case 110: /* named_parameters: named_parameter named_parameters_tail  */
                                          {/* named_parameters_tail */}
    break;

  case 111: /* named_parameter: NAME COLON expression  */
                          {/* named_parameter */}
    break;

  case 113: /* named_parameters_tail: COMMA named_parameter named_parameters_tail  */
                                                {/* named_parameters_tail */}
    break;

  case 114: /* positional_parameters: expression positional_parameters_tail  */
                                          {/* positional_parameters_tail */}
    break;

  case 116: /* positional_parameters_tail: COMMA expression positional_parameters_tail  */
                                                {/* positional_parameters_tail */}
    break;

  case 117: /* qualified_name: NAME DOT qualified_name  */
                            {/* qualified_name_tail */}
    break;

  case 118: /* qualified_name: NAME  */
         {/* qualified_name */}
    break;

  case 119: /* type: BUILT_IN_TYPE_NAME  */
                       {/* built_in_type_name */}
    break;

  case 121: /* $@9: %empty  */
            {/* type_name */}
    break;

  case 122: /* type: LIST LT $@9 type GT  */
                                      {/* list_type */}
    break;

  case 123: /* $@10: %empty  */
             {/* type_name */}
    break;

  case 124: /* type: RANGE LT $@10 type GT  */
                                       {/* range_type */}
    break;

  case 126: /* $@11: %empty  */
                                                     {/* type_name */}
    break;

  case 127: /* type: FUNCTION LT function_type_parameters RIGHT_ARROW $@11 type  */
                                                                            {/* function_type */}
    break;

  case 128: /* context_type_entries: context_type_entry context_type_entry_tail  */
                                               {/* context_type_entry_tail */}
    break;

  case 129: /* $@12: %empty  */
               {/* type_name */}
    break;

  case 130: /* context_type_entry: NAME COLON $@12 type  */
                                      {/* context_type_entry */}
    break;

  case 132: /* context_type_entry_tail: COMMA context_type_entry context_type_entry_tail  */
                                                     {/* context_type_entry_tail */}
    break;

  case 133: /* function_type_parameters: GT  */
       {/* function_type_parameters_empty */}
    break;

  case 134: /* $@13: %empty  */
    {/* type_name */}
    break;

  case 135: /* function_type_parameters: $@13 type function_type_parameters_tail  */
                                                         {/* function_type_parameters_tail */}
    break;

  case 137: /* $@14: %empty  */
          {/* type_name */}
    break;

  case 138: /* function_type_parameters_tail: COMMA $@14 type function_type_parameters_tail  */
                                                               {/* function_type_parameters_tail */}
    break;

  case 139: /* iteration_contexts: iteration_context COMMA iteration_contexts  */
                                               {/* iteration_contexts_tail */}
    break;

  case 140: /* iteration_contexts: iteration_context  */
                      {/* iteration_contexts_tail */}
    break;

  case 141: /* $@15: %empty  */
    {/* iteration_context_variable_name_begin */}
    break;

  case 142: /* $@16: %empty  */
                                                       {/* iteration_context_variable_name */}
    break;

  case 144: /* iteration_context_value: expression  */
               {/* iteration_context_value_single */}
    break;

  case 145: /* iteration_context_value: expression ELLIPSIS expression  */
                                   {/* iteration_context_value_range */}
    break;

  case 146: /* quantified_expressions: quantified_expression COMMA quantified_expressions  */
                                                       {/* quantified_expressions_tail */}
    break;

  case 147: /* quantified_expressions: quantified_expression  */
                          {/* quantified_expressions_tail */}
    break;

  case 148: /* $@17: %empty  */
    {/* quantified_expression_variable_name_begin */}
    break;

  case 149: /* $@18: %empty  */
                                                           {/* quantified_expression_variable_name */}
    break;

  case 150: /* quantified_expression: $@17 NAME $@18 IN expression  */
                                                                                                                     {/* quantified_expression */}
    break;

  case 151: /* $@19: %empty  */
                        {/* formal_parameters_begin */}
    break;

  case 152: /* function_definition: FUNCTION LEFT_PAREN $@19 formal_parameters external  */
                                                                                   {/* function_definition */}
    break;

  case 153: /* formal_parameters: RIGHT_PAREN  */
                {/* formal_parameters_empty */}
    break;

  case 154: /* $@20: %empty  */
                     {/* formal_parameters_first */}
    break;

  case 157: /* $@21: %empty  */
                           {/* formal_parameters_tail */}
    break;

  case 159: /* $@22: %empty  */
               {/* type_name */}
    break;

  case 160: /* formal_parameter: NAME COLON $@22 type  */
                                      {/* formal_parameter_with_type */}
    break;

  case 161: /* formal_parameter: NAME  */
         {/* formal_parameter_without_type */}
    break;

  case 162: /* external: EXTERNAL expression  */
                        {/* function_body_external */}
    break;

  case 163: /* external: expression  */
                              {/* function_body */}
    break;

  case 164: /* range_literal: range_literal_start range_literal_end  */
                                          {/* range_literal */}
    break;

  case 165: /* range_literal_start: LEFT_PAREN range_endpoint ELLIPSIS  */
                                       {/* range_literal_start */}
    break;

  case 166: /* range_literal_start: RIGHT_BRACKET range_endpoint ELLIPSIS  */
                                          {/* range_literal_start */}
    break;

  case 167: /* range_literal_start: LEFT_BRACKET range_endpoint ELLIPSIS  */
                                         {/* range_literal_start */}
    break;

  case 168: /* range_literal_end: range_endpoint RIGHT_PAREN  */
                               {/* range_literal_end */}
    break;

  case 169: /* range_literal_end: range_endpoint LEFT_BRACKET  */
                                {/* range_literal_end */}
    break;

  case 170: /* range_literal_end: range_endpoint RIGHT_BRACKET  */
                                 {/* range_literal_end */}
    break;

  case 171: /* range_endpoint: NUMERIC  */
            {/* literal_numeric */}
    break;

  case 172: /* range_endpoint: STRING  */
           {/* literal_string */}
    break;

  case 173: /* range_endpoint: AT STRING  */
              {/* literal_at */}
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


