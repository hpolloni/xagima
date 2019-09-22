#ifndef DESCRIPTORS_H
#define DESCRIPTORS_H

#include <stdint.h>
#include <string.h>

struct descriptor_ptr {
    const uint16_t limit;
    const uint32_t base;
} __attribute__((packed));

template<typename child, typename entry, size_t size>
class descriptor_table {
protected:
    mutable uint64_t entries[size];
    const descriptor_ptr ptr;
public:
    constexpr descriptor_table() : ptr{sizeof(uint64_t) * size - 1, reinterpret_cast<uint32_t>(&entries)}{
        memset(entries, 0, size);
    }

    constexpr void set(int id, entry e) const noexcept {
        entries[id] = e.bits;
    }

    constexpr void flush() const noexcept {
        static_cast<const child* const>(this)->flush_table();
    }
};

#endif