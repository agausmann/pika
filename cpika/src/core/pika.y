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
    %empty  { $$ = NULL; }
    | defs  { $$ = $1; }

defs:
    def         { $$ = cons_def(NULL, $1); }
    | defs def  { $$ = cons_def($1, $2); }

def:
    structdef   { $$ = def_structdef($1); }
    | enumdef   { $$ = def_enumdef($1); }
    | funcdef   { $$ = def_funcdef($1); }

structdef:
    STRUCT IDENT "{" argdef_list "}"        { $$ = structdef($2, $4); }

enumdef:
    ENUM IDENT "{" enum_variant_list "}"    { $$ = enumdef($2, $4); }

enum_variant_list:
    enum_variant                { $$ = $1; }
    | enum_variant_trailing     { $$ = $1; }

enum_variant_trailing:
    %empty              { $$ = NULL; }
    | enum_variant ","  { $$ = $1; }

enum_variant:
    enum_variant_trailing IDENT { $$ = cons_enum_variant($1, $2); }

funcdef:
    FN IDENT "(" argdef_list ")" returntype block   { $$ = funcdef($2, $4, $6, $7); }

argdef_list:
    argdef              { $$ = $1; }
    | argdef_trailing   { $$ = $1; }

argdef_trailing: 
    %empty          { $$ = NULL; }
    | argdef ","    { $$ = $1; }

argdef: argdef_trailing IDENT ":" type  { $$ = cons_argdef($1, $2, $4); }

returntype:
    %empty          { $$ = type_nil(); }
    | "->" type     { $$ = $2; }

block:
    "{" statement_list trailing_expr "}" { $$ = block($2, $3); }

statement_list:
    %empty                      { $$ = NULL; }
    | statement_list statement  { $$ = stmt_cons($1, $2); }

statement:
    expr ";"                                { $$ = stmt_expr($1); }
    | LET mut_specifier IDENT "=" expr ";"  { $$ = stmt_let($2, $3, $5); }
    | BREAK maybe_expr ";"                  { $$ = stmt_break($2); }
    | RETURN maybe_expr ";"                 { $$ = stmt_return($2); }

mut_specifier:
    %empty      { $$ = false; }
    | "mut"     { $$ = true; }

trailing_expr:
    %empty
    | expr

expr:
    "(" expr ")"                    { $$ = $2; }
    | path maybe_struct_init        { $$ = expr_path_or_struct($1, $2); }
    | "[" expr ";" INT_LITERAL "]"  { $$ = expr_array_init_fill($2, $4); }
    | INT_LITERAL                   { $$ = $1; }
    | FALSE                         { $$ = $1; }
    | TRUE                          { $$ = $1; }
    | expr "+" expr                 { $$ = expr_add($1, $3); }
    | expr "-" expr                 { $$ = expr_sub($1, $3); }
    | expr "." IDENT                { $$ = expr_field_access($1, $3); }
    | expr "=" expr                 { $$ = expr_assign($1, $3); }
    | expr EQ2 expr                 { $$ = expr_eq($1, $3); }
    | expr AND2 expr                { $$ = expr_and($1, $3); }
    | "!" expr                      { $$ = expr_not($2); }
    | "-" expr %prec NEG            { $$ = expr_neg($2); }
    | expr "[" expr "]" %prec "["   { $$ = expr_index($1, $3); }
    | if_expr                       { $$ = $1; }
    | for_expr                      { $$ = $1; }

maybe_expr:
    %empty      { $$ = literal_nil(); }
    | expr      { $$ = $1; }

maybe_struct_init:
    %empty                              { $$ = NULL; }
    | "{" struct_init_arg_list "}"      { $$ = $2; }

if_expr:
    if_case_list else_case { $$ = expr_if_else($1, $2); }

if_case_list:
    if_case                         { $$ = expr_cons_if(NULL, $1); }
    | if_case_list "else" if_case   { $$ = expr_cons_if(if_case_list, $3); }

if_case:
    IF "(" expr ")" block           { $$ = expr_if_case($3, $5); }

else_case:
    %empty                          { $$ = literal_nil(); }
    | ELSE block                    { $$ = $2; }

for_expr:
    FOR "(" IDENT IN expr DOT2 expr ")" block { $$ = expr_for($3, $5, $7, $9); }

struct_init_arg_list:
    struct_init_arg             { $$ = $1; }
    | struct_init_arg_trailing  { $$ = $1; }

struct_init_arg_trailing: 
    %empty                  { $$ = NULL; }
    | struct_init_arg ","   { $$ = $1; }

struct_init_arg: struct_init_arg_trailing IDENT ":" expr { $$ = cons_struct_init_arg($1, $2, $4); }

path:
    IDENT { $$ = cons_path(NULL, $1); }
    | path COLON2 IDENT { $$ = cons_path($1, $3); }

type:
    path                            { $$ = type_path($1); }
    | "[" type ";" INT_LITERAL "]"  { $$ = type_array($2, $4); }

%%
