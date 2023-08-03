%{
    #include "pika.tab.h"
    #include "pika.yy.h"
    #include "parse.h"
%}

%token
    BREAK   "break"
    ELSE    "else"
    ENUM    "enum"
    FALSE   "false"
    FN      "fn"
    FOR     "for"
    IF      "if"
    IN      "in"
    LET     "let"
    MUT     "mut"
    RETURN  "return"
    STRUCT  "struct"
    TRUE    "true"

    R_ARROW "->"
    DOT2    ".."
    COLON2  "::"
    EQ2     "=="
    AND2    "&&"
    PLUS    "+"
    MINUS   "-"
    DOT     "."
    COMMA   ","
    COLON   ":"
    SCLN    ";"
    EQ      "="
    BANG    "!"
    L_PAR   "("
    R_PAR   ")"
    L_BRK   "["
    R_BRK   "]"
    L_BRC   "{"
    R_BRC   "}"

    IDENT       "identifier"
    INT_LITERAL "integer literal"

// Expression operators
%left BREAK RETURN
%right "="
%left AND2
%left EQ2
%left "+" "-"
%left "*" "/"
%precedence "!" NEG
%left "." "["

%start module

%define parse.error detailed

%%

module:
    %empty
    | defs

defs:
    def
    | defs def

def:
    structdef
    | enumdef
    | funcdef

structdef:
    STRUCT IDENT "{" argdef_list "}"

enumdef:
    ENUM IDENT "{" enum_variant_list "}"

enum_variant_list:
    enum_variant
    | enum_variant_trailing

enum_variant_trailing:
    %empty
    | enum_variant ","

enum_variant:
    enum_variant_trailing IDENT

funcdef:
    FN IDENT "(" argdef_list ")" returntype block

argdef_list:
    argdef
    | argdef_trailing

argdef_trailing: 
    %empty
    | argdef ","

argdef: argdef_trailing IDENT ":" type

returntype:
    %empty
    | "->" type

block:
    "{" statement_list trailing_expr "}"

statement_list:
    %empty
    | statement_list statement

statement:
    expr ";"
    | LET mut_specifier IDENT "=" expr ";"
    | BREAK maybe_expr ";"
    | RETURN maybe_expr ";"

mut_specifier:
    %empty
    | "mut"

trailing_expr:
    %empty
    | expr

expr:
    "(" expr ")"
    | path maybe_struct_init
    | "[" expr ";" INT_LITERAL "]"
    | INT_LITERAL
    | FALSE
    | TRUE
    | expr "+" expr
    | expr "-" expr
    | expr "." IDENT
    | expr "=" expr
    | expr EQ2 expr
    | expr AND2 expr
    | "!" expr
    | "-" expr %prec NEG
    | expr "[" expr "]" %prec "["
    | if_expr
    | for_expr

maybe_expr:
    %empty
    | expr

maybe_struct_init:
    %empty
    | "{" struct_init_arg_list "}"

if_expr:
    if_case_list else_case

if_case_list:
    if_case
    | if_case_list "else" if_case

if_case:
    IF "(" expr ")" block

else_case:
    %empty
    | ELSE block

for_expr:
    FOR "(" IDENT IN expr DOT2 expr ")" block

struct_init_arg_list:
    struct_init_arg
    | struct_init_arg_trailing

struct_init_arg_trailing: 
    %empty
    | struct_init_arg ","

struct_init_arg: struct_init_arg_trailing IDENT ":" expr

path:
    IDENT
    | path COLON2 IDENT

type:
    path
    | "[" type ";" INT_LITERAL "]"

%%
