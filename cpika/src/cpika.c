#include <parse.h>
#include <pika.tab.h>

int main()
{
    ParserContext ctx;
    ctx_init(&ctx);

    yyparse(&ctx);

    ctx_free(&ctx);
    return 0;
}
