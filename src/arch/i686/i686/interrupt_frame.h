#ifndef INTERRUPT_FRAME_H
#define INTERRUPT_FRAME_H

#include <stdint.h>

struct interrupt_frame {
    uint32_t eip, cs, eflags;
};

#endif