#include <stdlib.h>
#include <mm.h>

extern "C" void* sbrk(long incr) {
  return mm::allocate(incr);
}
