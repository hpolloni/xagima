#ifndef STDLIB_H
#define STDLIB_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>

void abort(void);
void free(void* ptr);
void* malloc(size_t size);
void* realloc(void* ptr, size_t size);
void* sbrk(long incr);

#ifdef __cplusplus
}
#endif
#endif // STDLIB_H
