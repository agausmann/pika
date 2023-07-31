%{
    #include "pika.tab.h"
    #include "pika.yy.h"
    #include "parse.h"
%}
%token
    BREAK   "break"
    ELSE    "else"
    ENUM    "enum"
    FN      "fn"
    FOR     "for"
    IF      "if"
    IN      "in"
    LET     "let"
    MUT     "mut"
    RETURN  "return"
    STRUCT  "struct"

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
%left "&&"
%left "=="
%left "+" "-"
%left "*" "/"
%precedence NEG

%start module

%define parse.error detailed

%%

module:
    funcdef

expr:
    IDENT
    | INT_LITERAL
    | expr "+" expr

funcdef:
    "fn" IDENT "(" argdef_list ")" returntype block

argdef_list:
    argdef
    | argdef_trailing

argdef_trailing: 
    %empty
    | argdef ","

argdef: argdef_trailing IDENT ":" IDENT

returntype:
    %empty
    | "->" IDENT

block:
    "{" expr "}"

%%
