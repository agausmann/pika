#include "fnv.h"

#define FNV_32_PRIME 0x01000193ul
#define FNV_32_OFFSET_BASIS 0x811c9dc5ul
#define FNV_64_PRIME 0x00000100000001B3ull
#define FNV_64_OFFSET_BASIS 0xcbf29ce484222325ull

uint32_t fnv_1a_32_str(const char *str)
{
    uint32_t hash = FNV_32_OFFSET_BASIS;
    while (*str)
    {
        hash ^= *str;
        hash *= FNV_32_PRIME;
        ++str;
    }
    return hash;
}

uint64_t fnv_1a_64_str(const char *str)
{
    uint64_t hash = FNV_64_OFFSET_BASIS;
    while (*str)
    {
        hash ^= *str;
        hash *= FNV_64_PRIME;
        ++str;
    }
    return hash;
}
