#include "parse.h"

#include "pika.tab.h"
#include "keywords.h"

#include <stdio.h>
#include <string.h>

int ident_or_keyword(const char *s)
{
    const struct keyword_entry *lookup_result = in_keyword_set(s, strlen(s));
    if (lookup_result)
    {
        return lookup_result->keyword_type;
    }
    return IDENT;
}

void yyerror(ParserContext *ctx, const char *s)
{
    fprintf(stderr, "%s\n", s);
}

int yywrap()
{
    return 1;
}

ParseExpr literal_nil(ParserContext *ctx)
{
    return 0; // TODO
}

ParseType type_nil(ParserContext *ctx)
{
    return 0; // TODO
}

Ident parse_ident(ParserContext *ctx, IString text)
{
    Ident result;
    result.text = text;
    return result;
}

IntLiteral parse_int_literal(ParserContext *ctx, IString text)
{
    IntLiteral result;
    result.text = text;
    return result;
}

ParseModule parse_module_empty(ParserContext *ctx)
{
    return 0; // TODO
}

ParseModule parse_module(ParserContext *ctx, ParseItems items)
{
    return 0; // TODO
}

ParseItems parse_items_start(ParserContext *ctx)
{
    return 0; // TODO
}

ParseItems parse_items_cons(ParserContext *ctx, ParseItems base, ParseItem next)
{
    return 0; // TODO
}

ParseItem parse_item_struct(ParserContext *ctx, Ident name, ParseArgdefList fields)
{
    return 0; // TODO
}

ParseItem parse_item_enum(ParserContext *ctx, Ident name, ParseVariantList variants)
{
    return 0; // TODO
}

ParseItem parse_item_func(ParserContext *ctx, Ident name, ParseArgdefList args, ParseType return_type, ParseExpr body)
{
    return 0; // TODO
}

ParseArgdefList parse_argdef_start(ParserContext *ctx)
{
    return 0; // TODO
}

ParseArgdefList parse_argdef_cons(ParserContext *ctx, ParseArgdefList base, Ident arg_name, ParseType arg_type)
{
    return 0; // TODO
}

ParseVariantList parse_enum_variant_start(ParserContext *ctx)
{
    return 0; // TODO
}

ParseVariantList parse_enum_variant_cons(ParserContext *ctx, ParseVariantList base, Ident variant)
{
    return 0; // TODO
}

ParseExpr parse_block(ParserContext *ctx, ParseStatementList statements, ParseExpr trailing_expr)
{
    return 0; // TODO
}

ParseStatementList parse_stmt_list_start(ParserContext *ctx)
{
    return 0; // TODO
}

ParseStatementList parse_stmt_list_cons(ParserContext *ctx, ParseStatementList base, ParseStatement next)
{
    return 0; // TODO
}

ParseStatement parse_stmt_expr(ParserContext *ctx, ParseExpr expr)
{
    return 0; // TODO
}

ParseStatement parse_stmt_let(ParserContext *ctx, bool mut_specifier, Ident binding, ParseExpr value)
{
    return 0; // TODO
}

ParseStatement parse_stmt_break(ParserContext *ctx, ParseExpr value)
{
    return 0; // TODO
}

ParseStatement parse_stmt_return(ParserContext *ctx, ParseExpr value)
{
    return 0; // TODO
}

ParseExpr parse_expr_path(ParserContext *ctx, ParsePath path)
{
    return 0; // TODO
}

ParseExpr parse_expr_struct_init(ParserContext *ctx, ParsePath path, ParseStructInitList init_list)
{
    return 0; // TODO
}

ParseExpr parse_expr_array_fill(ParserContext *ctx, ParseExpr value, IntLiteral size)
{
    return 0; // TODO
}

ParseExpr parse_expr_int_literal(ParserContext *ctx, IntLiteral value)
{
    return 0; // TODO
}

ParseExpr parse_expr_false(ParserContext *ctx)
{
    return 0; // TODO
}

ParseExpr parse_expr_true(ParserContext *ctx)
{
    return 0; // TODO
}

ParseExpr parse_expr_add(ParserContext *ctx, ParseExpr lhs, ParseExpr rhs)
{
    return 0; // TODO
}

ParseExpr parse_expr_sub(ParserContext *ctx, ParseExpr lhs, ParseExpr rhs)
{
    return 0; // TODO
}

ParseExpr parse_expr_field_access(ParserContext *ctx, ParseExpr lhs, Ident field)
{
    return 0; // TODO
}

ParseExpr parse_expr_assign(ParserContext *ctx, ParseExpr lhs, ParseExpr rhs)
{
    return 0; // TODO
}

ParseExpr parse_expr_eq(ParserContext *ctx, ParseExpr lhs, ParseExpr rhs)
{
    return 0; // TODO
}

ParseExpr parse_expr_and(ParserContext *ctx, ParseExpr lhs, ParseExpr rhs)
{
    return 0; // TODO
}

ParseExpr parse_expr_not(ParserContext *ctx, ParseExpr inner)
{
    return 0; // TODO
}

ParseExpr parse_expr_neg(ParserContext *ctx, ParseExpr inner)
{
    return 0; // TODO
}

ParseExpr parse_expr_index(ParserContext *ctx, ParseExpr base, ParseExpr index)
{
    return 0; // TODO
}

ParseExpr parse_if_expr(ParserContext *ctx, ParseIfCaseList if_case_list, ParseExpr else_block)
{
    return 0; // TODO
}

ParseIfCaseList parse_if_case_start(ParserContext *ctx, ParseIfCase if_case)
{
    return 0; // TODO
}

ParseIfCaseList parse_if_case_cons(ParserContext *ctx, ParseIfCaseList base, ParseIfCase next)
{
    return 0; // TODO
}

ParseIfCase parse_if_case(ParserContext *ctx, ParseExpr condition, ParseExpr block)
{
    return 0; // TODO
}

ParseExpr parse_for_expr(ParserContext *ctx, Ident binding, ParseExpr start, ParseExpr end, ParseExpr block)
{
    return 0; // TODO
}

ParseStructInitList parse_struct_init_start(ParserContext *ctx)
{
    return 0; // TODO
}

ParseStructInitList parse_struct_init_cons(ParserContext *ctx, ParseStructInitList base, Ident field, ParseExpr value)
{
    return 0; // TODO
}

ParsePath parse_path_start(ParserContext *ctx, Ident element)
{
    return 0; // TODO
}

ParsePath parse_path_cons(ParserContext *ctx, ParsePath base, Ident element)
{
    return 0; // TODO
}

ParseType parse_type_path(ParserContext *ctx, ParsePath path)
{
    return 0; // TODO
}

ParseType parse_type_array(ParserContext *ctx, ParseType element_type, IntLiteral size)
{
    return 0; // TODO
}
