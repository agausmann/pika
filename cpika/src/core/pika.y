%{
    #include "parse.h"
    #include "pika.tab.h"
    #include "pika.yy.h"

    #define ctx yyget_extra(scanner)
%}

%define api.pure
%parse-param { void *scanner }
%lex-param { void *scanner }
%define parse.error detailed

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

%union {
    int todo;

    ParseModule module;
    ParseItems items;
    ParseItem item;
    ParseArgdefList argdef_list;
    ParseVariantList variant_list;
    ParseStatement statement;
    ParseStatementList statement_list;
    ParseExpr expr;
    ParseStructInitList struct_init_list;
    ParseType type;
    ParsePath path;
    ParseIfCase if_case;
    ParseIfCaseList if_case_list;

    IString token;
    Ident ident;
    IntLiteral int_literal;

    bool mut_specifier;
}

%type <module> module;
%type <items> items;
%type <item> item;
%type <argdef_list>
    argdef_list
    argdef_trailing
    argdef;
%type <variant_list>
    enum_variant_list
    enum_variant_trailing
    enum_variant;
%type <statement> statement;
%type <statement_list> statement_list;
%type <expr>
    expr
    maybe_expr
    block
    if_expr
    for_expr
    else_case;
%type <struct_init_list>
    struct_init_arg_list
    struct_init_arg_trailing
    struct_init_arg;
%type <type> type returntype;
%type <path> path;
%type <if_case> if_case;
%type <if_case_list> if_case_list;

%type <ident> ident;
%type <int_literal> int_literal;
%type <mut_specifier> mut_specifier;

%%

ident: IDENT { $$ = parse_ident(ctx, $<token>1); }
int_literal: INT_LITERAL { $$ = parse_int_literal(ctx, $<token>1); }

module:
    %empty  { $$ = parse_module_empty(ctx); }
    | items  { $$ = parse_module(ctx, $1); }

items:
    item { $$ = parse_items_start(ctx); }
    | items item  { $$ = parse_items_cons(ctx, $1, $2); }

item:
    STRUCT ident "{" argdef_list "}" { $$ = parse_item_struct(ctx, $2, $4); }
    | ENUM ident "{" enum_variant_list "}" { $$ = parse_item_enum(ctx, $2, $4); }
    | FN ident "(" argdef_list ")" returntype block { $$ = parse_item_func(ctx, $2, $4, $6, $7); }

enum_variant_list:
    enum_variant                { $$ = $1; }
    | enum_variant_trailing     { $$ = $1; }

enum_variant_trailing:
    %empty              { $$ = parse_enum_variant_start(ctx); }
    | enum_variant ","  { $$ = $1; }

enum_variant:
    enum_variant_trailing ident { $$ = parse_enum_variant_cons(ctx, $1, $2); }

argdef_list:
    argdef              { $$ = $1; }
    | argdef_trailing   { $$ = $1; }

argdef_trailing: 
    %empty          { $$ = parse_argdef_start(ctx); }
    | argdef ","    { $$ = $1; }

argdef: argdef_trailing ident ":" type  { $$ = parse_argdef_cons(ctx, $1, $2, $4); }

returntype:
    %empty          { $$ = type_nil(ctx); }
    | "->" type     { $$ = $2; }

block:
    "{" statement_list maybe_expr "}" { $$ = parse_block(ctx, $2, $3); }

statement_list:
    %empty                      { $$ = parse_stmt_list_start(ctx); }
    | statement_list statement  { $$ = parse_stmt_list_cons(ctx, $1, $2); }

statement:
    expr ";"                                { $$ = parse_stmt_expr(ctx, $1); }
    | LET mut_specifier ident "=" expr ";"  { $$ = parse_stmt_let(ctx, $2, $3, $5); }
    | BREAK maybe_expr ";"                  { $$ = parse_stmt_break(ctx, $2); }
    | RETURN maybe_expr ";"                 { $$ = parse_stmt_return(ctx, $2); }

mut_specifier:
    %empty      { $$ = false; }
    | "mut"     { $$ = true; }

expr:
    "(" expr ")"                            { $$ = $2; }
    | path                                  { $$ = parse_expr_path(ctx, $1); }
    | path "{" struct_init_arg_list "}"     { $$ = parse_expr_struct_init(ctx, $1, $3); }
    | "[" expr ";" int_literal "]"          { $$ = parse_expr_array_fill(ctx, $2, $4); }
    | int_literal                           { $$ = parse_expr_int_literal(ctx, $1); }
    | FALSE                                 { $$ = parse_expr_false(ctx); }
    | TRUE                                  { $$ = parse_expr_true(ctx); }
    | expr "+" expr                         { $$ = parse_expr_add(ctx, $1, $3); }
    | expr "-" expr                         { $$ = parse_expr_sub(ctx, $1, $3); }
    | expr "." ident                        { $$ = parse_expr_field_access(ctx, $1, $3); }
    | expr "=" expr                         { $$ = parse_expr_assign(ctx, $1, $3); }
    | expr EQ2 expr                         { $$ = parse_expr_eq(ctx, $1, $3); }
    | expr AND2 expr                        { $$ = parse_expr_and(ctx, $1, $3); }
    | "!" expr                              { $$ = parse_expr_not(ctx, $2); }
    | "-" expr %prec NEG                    { $$ = parse_expr_neg(ctx, $2); }
    | expr "[" expr "]" %prec "["           { $$ = parse_expr_index(ctx, $1, $3); }
    | if_expr                               { $$ = $1; }
    | for_expr                              { $$ = $1; }
    | block                                 { $$ = $1; }

maybe_expr:
    %empty      { $$ = literal_nil(ctx); }
    | expr      { $$ = $1; }

if_expr:
    if_case_list else_case { $$ = parse_if_expr(ctx, $1, $2); }

if_case_list:
    if_case                         { $$ = parse_if_case_start(ctx, $1); }
    | if_case_list "else" if_case   { $$ = parse_if_case_cons(ctx, $1, $3); }

if_case:
    IF "(" expr ")" block           { $$ = parse_if_case(ctx, $3, $5); }

else_case:
    %empty                          { $$ = literal_nil(ctx); }
    | ELSE block                    { $$ = $2; }

for_expr:
    FOR "(" ident IN expr DOT2 expr ")" block { $$ = parse_for_expr(ctx, $3, $5, $7, $9); }

struct_init_arg_list:
    struct_init_arg             { $$ = $1; }
    | struct_init_arg_trailing  { $$ = $1; }

struct_init_arg_trailing: 
    %empty                  { $$ = parse_struct_init_start(ctx); }
    | struct_init_arg ","   { $$ = $1; }

struct_init_arg:
    struct_init_arg_trailing ident ":" expr { $$ = parse_struct_init_cons(ctx, $1, $2, $4); }

path:
    ident { $$ = parse_path_start(ctx, $1); }
    | path COLON2 ident { $$ = parse_path_cons(ctx, $1, $3); }

type:
    path                            { $$ = parse_type_path(ctx, $1); }
    | "[" type ";" int_literal "]"  { $$ = parse_type_array(ctx, $2, $4); }

%%
