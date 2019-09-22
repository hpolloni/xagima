#ifndef GDT_H
#define GDT_H

#include "descriptors.h"

struct gdt_entry {
    uint64_t bits;
};

extern "C" void gdt_flush(uint32_t);
extern "C" void flush_segments();

class global_descriptor_table : public descriptor_table<global_descriptor_table, gdt_entry, 5> {
public:
    constexpr global_descriptor_table() : descriptor_table() {
        set(0, { 0 });
        set(1, { 0x00CF9A000000FFFF });
        set(2, { 0x00CF92000000FFFF });
        set(3, { 0x00CFFA000000FFFF });
        set(4, { 0x00CFF2000000FFFF });
    }

    void flush_table() const noexcept {
        gdt_flush((uint32_t)&ptr);
        flush_segments();
    }
};

#endif