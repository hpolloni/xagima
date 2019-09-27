#ifndef CPU_H
#define CPU_H

#include <stdint.h>
#include <utils/singleton.h>
#include <utils/array.h>
#include <string.h>

namespace cpu {
  void init();

  struct interrupt_frame;
  class interrupt_handler {
  public:
    virtual void handle_interrupt(uint8_t, interrupt_frame*) = 0;
  };

  class interrupt_manager : public singleton<interrupt_manager> {
   interrupt_handler* handlers[256];
  public:
    interrupt_manager() {
      memset(handlers, 0, 256);
    }

    void register_handler(uint8_t int_no, interrupt_handler *handler) {
      handlers[int_no] = handler;
    }

    void handle_interrupt(uint8_t int_no, interrupt_frame* frame) {
      if (handlers[int_no]) {
        handlers[int_no]->handle_interrupt(int_no, frame);
      }
    }
  };
}

#endif
