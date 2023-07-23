#include "intern.h"

#include "fnv.h"
#include <stdlib.h>
#include <string.h>

static IString *table_lookup(IString *hash_table, size_t capacity, const char *value)
{
    if (!value)
    {
        return NULL;
    }

    uint32_t hash = fnv_1a_32_str(value);
    size_t index = hash % capacity;
    while (hash_table[index].value && strcmp(value, hash_table[index].value) != 0)
    {
        index = (index + 1) % capacity;
    }
    return &hash_table[index];
}

void intern_init(Intern *self)
{
    self->hash_table = NULL;
    self->capacity = 0;
    self->occupied = 0;
}

void intern_free(Intern *self)
{
    for (size_t i = 0; i < self->capacity; i++)
    {
        free(self->hash_table[i].value);
    }
    free(self->hash_table);
}

IString intern_get(Intern *self, const char *value)
{
    if (!value)
    {
        IString empty;
        empty.value = NULL;
        return empty;
    }

    if (self->occupied >= self->capacity / 2)
    {
        // Increase table size when load factor is too high

        size_t new_capacity = self->capacity * 2;
        if (new_capacity < 16)
        {
            new_capacity = 16;
        }

        IString *new_table = calloc(new_capacity, sizeof(IString));
        for (size_t i = 0; i < new_capacity; i++)
        {
            new_table[i].value = NULL;
        }

        for (size_t i = 0; i < self->capacity; i++)
        {
            IString *entry = table_lookup(new_table, new_capacity, self->hash_table[i].value);
            *entry = self->hash_table[i];
        }

        free(self->hash_table);
        self->hash_table = new_table;
        self->capacity = new_capacity;
    }

    IString *entry = table_lookup(self->hash_table, self->capacity, value);
    if (!entry->value)
    {
        entry->value = strdup(value);
        self->occupied += 1;
    }
    return *entry;
}
