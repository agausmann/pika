#pragma once

#include "parser_context.h"
#include <stdbool.h>

int ident_or_keyword(const char *s);

void yyerror(ParserContext *ctx, const char *s);
int yywrap();

typedef struct
{
    IString text;
} IntLiteral;

typedef struct
{
    IString text;
} Ident;

// TODO specify these types
typedef int ParseModule;
typedef int ParseItems;
typedef int ParseItem;
typedef int ParseArgdefList;
typedef int ParseVariantList;
typedef int ParseStatement;
typedef int ParseStatementList;
typedef int ParseExpr;
typedef int ParseStructInitList;
typedef int ParseType;
typedef int ParsePath;
typedef int ParseIfCase;
typedef int ParseIfCaseList;

ParseExpr literal_nil(ParserContext *ctx);
ParseType type_nil(ParserContext *ctx);

Ident parse_ident(ParserContext *ctx, IString text);
IntLiteral parse_int_literal(ParserContext *ctx, IString text);

ParseModule parse_module_empty(ParserContext *ctx);
ParseModule parse_module(ParserContext *ctx, ParseItems items);

ParseItems parse_items_start(ParserContext *ctx);
ParseItems parse_items_cons(ParserContext *ctx, ParseItems base, ParseItem next);

ParseItem parse_item_struct(ParserContext *ctx, Ident name, ParseArgdefList fields);
ParseItem parse_item_enum(ParserContext *ctx, Ident name, ParseVariantList variants);
ParseItem parse_item_func(ParserContext *ctx, Ident name, ParseArgdefList args, ParseType return_type, ParseExpr body);

ParseArgdefList parse_argdef_start(ParserContext *ctx);
ParseArgdefList parse_argdef_cons(ParserContext *ctx, ParseArgdefList base, Ident arg_name, ParseType arg_type);

ParseVariantList parse_enum_variant_start(ParserContext *ctx);
ParseVariantList parse_enum_variant_cons(ParserContext *ctx, ParseVariantList base, Ident variant);

ParseExpr parse_block(ParserContext *ctx, ParseStatementList statements, ParseExpr trailing_expr);

ParseStatementList parse_stmt_list_start(ParserContext *ctx);
ParseStatementList parse_stmt_list_cons(ParserContext *ctx, ParseStatementList base, ParseStatement next);

ParseStatement parse_stmt_expr(ParserContext *ctx, ParseExpr expr);
ParseStatement parse_stmt_let(ParserContext *ctx, bool mut_specifier, Ident binding, ParseExpr value);
ParseStatement parse_stmt_break(ParserContext *ctx, ParseExpr value);
ParseStatement parse_stmt_return(ParserContext *ctx, ParseExpr value);

ParseExpr parse_expr_path(ParserContext *ctx, ParsePath path);
ParseExpr parse_expr_struct_init(ParserContext *ctx, ParsePath struct_path, ParseStructInitList init_list);
ParseExpr parse_expr_array_fill(ParserContext *ctx, ParseExpr value, IntLiteral size);
ParseExpr parse_expr_int_literal(ParserContext *ctx, IntLiteral value);
ParseExpr parse_expr_false(ParserContext *ctx);
ParseExpr parse_expr_true(ParserContext *ctx);
ParseExpr parse_expr_add(ParserContext *ctx, ParseExpr lhs, ParseExpr rhs);
ParseExpr parse_expr_sub(ParserContext *ctx, ParseExpr lhs, ParseExpr rhs);
ParseExpr parse_expr_field_access(ParserContext *ctx, ParseExpr lhs, Ident field);
ParseExpr parse_expr_assign(ParserContext *ctx, ParseExpr lhs, ParseExpr rhs);
ParseExpr parse_expr_eq(ParserContext *ctx, ParseExpr lhs, ParseExpr rhs);
ParseExpr parse_expr_and(ParserContext *ctx, ParseExpr lhs, ParseExpr rhs);
ParseExpr parse_expr_not(ParserContext *ctx, ParseExpr inner);
ParseExpr parse_expr_neg(ParserContext *ctx, ParseExpr inner);
ParseExpr parse_expr_index(ParserContext *ctx, ParseExpr base, ParseExpr index);

ParseExpr parse_if_expr(ParserContext *ctx, ParseIfCaseList if_case_list, ParseExpr else_block);
ParseIfCaseList parse_if_case_start(ParserContext *ctx, ParseIfCase if_case);
ParseIfCaseList parse_if_case_cons(ParserContext *ctx, ParseIfCaseList base, ParseIfCase if_case);
ParseIfCase parse_if_case(ParserContext *ctx, ParseExpr condition, ParseExpr block);

ParseExpr parse_for_expr(ParserContext *ctx, Ident binding, ParseExpr start, ParseExpr end, ParseExpr block);

ParseStructInitList parse_struct_init_start(ParserContext *ctx);
ParseStructInitList parse_struct_init_cons(ParserContext *ctx, ParseStructInitList base, Ident ident, ParseExpr expr);

ParsePath parse_path_start(ParserContext *ctx, Ident element);
ParsePath parse_path_cons(ParserContext *ctx, ParsePath base, Ident element);

ParsePath parse_type_path(ParserContext *ctx, ParsePath path);
ParseType parse_type_array(ParserContext *ctx, ParseType element_type, IntLiteral size);
