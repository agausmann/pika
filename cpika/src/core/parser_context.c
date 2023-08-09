#include "parser_context.h"

void ctx_init(ParserContext *ctx)
{
    intern_init(&ctx->intern);
}

void ctx_free(ParserContext *ctx)
{
    intern_free(&ctx->intern);
}

IString ctx_intern(ParserContext *ctx, const char *text)
{
    return intern_get(&ctx->intern, text);
}
