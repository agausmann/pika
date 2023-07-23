#pragma once

#include <stdio.h>

#define ASSERT(EXPR)  \
    if (!(EXPR))      \
    {                 \
        return #EXPR; \
    }

#define RUN_TEST(NAME) run_test(#NAME, NAME)

static void run_test(const char *name, const char *(*test)())
{
    printf("%s ... ", name);
    fflush(stdout);
    const char *result = test();
    if (result)
    {
        printf("\033[31mERROR:\033[0m %s\n", result);
    }
    else
    {
        printf("OK\n");
    }
}
