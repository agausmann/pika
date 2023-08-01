#include "parse.h"
#include <stdio.h>

void yyerror(const char *s)
{
    fprintf(stderr, "%s\n", s);
}

int yywrap()
{
    return 1;
}
