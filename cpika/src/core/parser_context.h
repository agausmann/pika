#pragma once

#include "intern.h"

typedef struct
{
    Intern intern;
} ParserContext;

void ctx_init(ParserContext *ctx);
void ctx_free(ParserContext *ctx);

IString ctx_intern(ParserContext *ctx, const char *text);
