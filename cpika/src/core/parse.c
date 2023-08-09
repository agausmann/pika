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
