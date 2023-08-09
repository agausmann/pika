#include <parse.h>
#include <pika.tab.h>
#include <pika.yy.h>

int main()
{
    ParserContext ctx;
    ctx_init(&ctx);

    void *scanner;
    yylex_init_extra(&ctx, &scanner);

    yyparse(scanner);

    yylex_destroy(scanner);
    ctx_free(&ctx);
    return 0;
}
