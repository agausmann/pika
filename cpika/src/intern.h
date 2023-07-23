#pragma once

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

/**
 * @brief Interned string value.
 *
 * Values can be obtained with intern_get() and compared with istring_eq().
 * Cleanup is a no-op since the strings are owned and managed by the interning
 * context.
 */
typedef struct
{
    char *value;
} IString;

/**
 * @brief String interning context.
 *
 * Construct with intern_init() and clean up with intern_free().
 *
 * Strings can be cheaply stored and then cheaply compared with intern_get() and
 * istring_eq().
 */
typedef struct
{
    IString *hash_table;
    size_t capacity;
    size_t occupied;
} Intern;

/**
 * @brief Initialize an interning context.
 */
void intern_init(Intern *self);

/**
 * @brief Clean up an interning context.
 *
 * All memory will be freed and all IString values belonging to this context will be invalidated
 */
void intern_free(Intern *self);

/**
 * @brief Store a string in an interning context.
 *
 * @param value A string to lookup or insert. The string this points to does not need to outlive the
 * interning context. If the value doesn't exist in the table yet, then the contents of the string
 * will be duplicated in the storage of the context.
 *
 * @return IString A value representing the string stored in the interning context.
 */
IString intern_get(Intern *self, const char *value);

/**
 * @brief Compare two interned strings for equality.
 *
 * This is very cheap; it just has to perform a pointer equality.
 *
 * @return true if and only if the strings belong to the same intern context and
 * have the same value.
 */
static inline bool istring_eq(IString a, IString b)
{
    return a.value == b.value;
}
