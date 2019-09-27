#ifndef IO_UTILS_H
#define IO_UTILS_H

#include <stdint.h>

template<bool Cond, class T = void> struct enable_if {};
template<class T> struct enable_if<true, T> { typedef T type; };

namespace io {
  namespace mm {
    template<typename type>
    type read(uint64_t address) {
      return *((volatile type*)(address));
    }

    template<typename type>
    void write(uint64_t address, type value) {
      (*((volatile type*)(address))) = (value); 
    }
  }

  namespace port {
    template<typename T, 
         typename enable_if<(sizeof(T) == 4 || sizeof(T) == 2 || sizeof(T) == 1), int>::type = 0>
    void write(uint16_t port, T value) {
      if (sizeof(T) == 4) {
        uint32_t val = (uint32_t)value;
        asm volatile ("outl %1, %0" : : "dN" (port), "a" (val));
      }
      if (sizeof(T) == 2) {
        uint16_t val = (uint16_t)value;
        asm volatile ("outw %1, %0" : : "dN" (port), "a" (val));
      }
      if (sizeof(T) == 1) {
        uint8_t val = (uint8_t) value;
        asm volatile ("outb %1, %0" : : "dN" (port), "a" (val));
      }
    }

    template<typename T,
         typename enable_if<(sizeof(T) == 4 || sizeof(T) == 2 || sizeof(T) == 1), int>::type = 0>
    T read(uint16_t port) {
      T value;
      if (sizeof(T) == 4) {
        asm volatile("inl %1, %0" : "=a" (value) : "dN" (port));
      }
      if (sizeof(T) == 2) {
        asm volatile ("inw %1, %0" : "=a" (value) : "dN" (port));
      }
      if (sizeof(T) == 1) {
        asm volatile("inb %1, %0" : "=a" (value) : "dN" (port));
      }
      return value;
    }
  }
}
#endif
