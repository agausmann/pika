#pragma once

#include <stdint.h>

/**
 * @brief Hash a string with 32-bit FNV-1a.
 * @param str A null-terminated character array to hash.
 * @return The 32-bit FNV-1a hash of the provided string.
 */
uint32_t fnv_1a_32_str(const char *str);

/**
 * @brief Hash a string with 64-bit FNV-1a.
 * @param str A null-terminated character array to hash.
 * @return The 64-bit FNV-1a hash of the provided string.
 */
uint64_t fnv_1a_64_str(const char *str);
