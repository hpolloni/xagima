#ifndef MM_H
#define MM_H

#include <stddef.h>

namespace mm {
  void init(size_t, size_t);
  void* allocate(size_t);
  void* allocate_aligned(size_t);
}

#endif