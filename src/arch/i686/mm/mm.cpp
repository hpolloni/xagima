#include <mm.h>
#include <stdint.h>
#include <assert.h>

extern uint32_t kernel_end;

namespace mm {
    uint32_t placement = kernel_end;
    uint32_t upper_limit;

    void init(size_t /* lower */, size_t upper) {
        upper_limit = upper;
    }

    void* allocate(size_t size) {
        void* tmp = (void*)placement;
        placement += size;
        assert(placement < upper_limit);
        return (void*)tmp;
    }

    void* allocate_aligned(size_t size) {
        if (placement & 0xFFFFF000) {
            placement &= 0xFFFFF000;
            placement += 0x1000;
        }
        return allocate(size);
    }
}