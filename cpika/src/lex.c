#include "lex.h"
#include "pika.tab.h"
#include <string.h>

int ident_or_keyword(const char *s)
{
    if (strcmp(s, "fn") == 0)
    {
        return FN;
    }

    return IDENT;
}
