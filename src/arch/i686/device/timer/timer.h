#pragma once

#include <device/device_manager.h>
#include <cpu.h>
#include <io.h>

class timer_handler : public cpu::interrupt_handler {
  uint32_t tick = 0;
public:
  void handle_interrupt(uint8_t, cpu::interrupt_frame*) {
    tick++;
  }
};

class timer_device : public device::driver {
  timer_handler ih;
public:
  void init() override {
    uint32_t divisor = 1193180 / 50;
    io::port::write(0x43, (uint8_t)0x36);
    uint8_t l = (uint8_t)(divisor & 0xFF);
    uint8_t h = (uint8_t)( (divisor>>8) & 0xFF);
    
    io::port::write(0x40, l);
    io::port::write(0x40, h);

    cpu::interrupt_manager::instance().register_handler(32, &ih);
  }
  
  bool is_present() override {
    return true;
  }

  const char* name() override {
    return "PIT timer";
  }
};