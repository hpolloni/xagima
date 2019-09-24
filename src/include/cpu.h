#ifndef CPU_H
#define CPU_H

#include <stdint.h>
#include <utils/singleton.h>
#include <utils/array.h>

namespace cpu {
  void init();

  struct interrupt_frame;
  class interrupt_handler {
  public:
    virtual void handle_interrupt(uint8_t, interrupt_frame*) = 0;
  };

  class interrupt_manager : public singleton<interrupt_manager> {
    array<interrupt_handler*, 256> handlers;
  public:
    void register_handler(interrupt_handler *handler) {
      handlers.add(handler);
    }

    void handle_interrupt(uint8_t int_no, interrupt_frame* frame) {
      handlers[int_no]->handle_interrupt(int_no, frame);
    }
  };
}

#endif
