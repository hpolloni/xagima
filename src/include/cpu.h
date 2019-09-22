#ifndef CPU_H
#define CPU_H

#include <stdint.h>

namespace cpu {
  void init();
  void set_interrupt(int interrupt_number, uint32_t interrupt_address);
}

#endif