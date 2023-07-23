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

int main()
{
    RUN_TEST(it_works);
    return 0;
}
