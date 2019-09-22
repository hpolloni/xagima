#include <stdlib.h>

extern "C" void abort(void) {
    for(;;);
}
