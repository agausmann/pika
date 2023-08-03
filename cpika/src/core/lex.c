#include "lex.h"
#include "pika.tab.h"
#include <string.h>
#include "keywords.h"

int ident_or_keyword(const char *s)
{
    const struct keyword_entry *lookup_result = in_keyword_set(s, strlen(s));
    if (lookup_result)
    {
        return lookup_result->keyword_type;
    }
    return IDENT;
}
