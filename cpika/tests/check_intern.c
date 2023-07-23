#include <intern.h>
#include "test.h"

const char *it_works()
{
    Intern intern;
    intern_init(&intern);
    IString a = intern_get(&intern, "Test");
    IString b = intern_get(&intern, "Test");
    IString c = intern_get(&intern, "Test2");
    ASSERT(istring_eq(a, b));
    ASSERT(!istring_eq(a, c));
    ASSERT(!istring_eq(b, c));

    intern_free(&intern);
    return NULL;
}

const char *resize()
{
    Intern intern;
    intern_init(&intern);
    IString a = intern_get(&intern, "Test");
    IString b = intern_get(&intern, "Test");
    IString c = intern_get(&intern, "Test2");

    char buf[16];
    for (int i = 0; i < 100; i++)
    {
        snprintf(buf, sizeof(buf), "Extra%d", i);
        intern_get(&intern, buf);
    }

    ASSERT(istring_eq(a, b));
    ASSERT(!istring_eq(a, c));
    ASSERT(!istring_eq(b, c));

    intern_free(&intern);
    return NULL;
}

int main()
{
    int errors = 0;
    errors += RUN_TEST(it_works);
    errors += RUN_TEST(resize);

    return errors ? 1 : 0;
}
