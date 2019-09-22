#ifndef IDT_H
#define IDT_H

#include "descriptors.h"

union idt_entry {
    struct {
        uint16_t base_low;
        uint16_t cs;
        uint8_t  zero;
        uint8_t  flags;
        uint16_t base_high;
    } __attribute__((packed)) fields;
    uint64_t bits;
};

extern "C" void idt_flush(uint32_t);

class interrupt_descriptor_table : public descriptor_table<interrupt_descriptor_table, idt_entry, 256> {
public:
    constexpr interrupt_descriptor_table() : descriptor_table() {}
    void flush_table() const noexcept {
        idt_flush(static_cast<uint32_t>(reinterpret_cast<uintptr_t>(&ptr)));
    }
};

#endif
