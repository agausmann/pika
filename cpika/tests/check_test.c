#include "test.h"

char const *fail()
{
    ASSERT(0);

    return NULL;
}

int main()
{
    int errors = 0;
    errors += RUN_TEST(fail);

    return errors ? 1 : 0;
}
