#include <cpu.h>
#include <stdint.h>
#include "gdt.h"
#include "idt.h"
#include "interrupt_frame.h"
#include <assert.h>
#include <io.h>

#define EXCEPTION_HANDLER_ENTRY(x) set_exception_interrupt(x, static_cast<uint32_t>(reinterpret_cast<uintptr_t>(exception_handler_ ## x)));
#define EXCEPTION_HANDLER_WITH_ERROR_CODE(x) extern "C" __attribute__((interrupt)) \
void exception_handler_ ## x (cpu::interrupt_frame* frame, uint32_t) { \
  cpu::interrupt_manager::instance().handle_interrupt(x, frame); \
}

#define EXCEPTION_HANDLER_NO_ERROR_CODE(x) extern "C" __attribute__((interrupt)) \
void exception_handler_ ## x (cpu::interrupt_frame* frame) { \
  cpu::interrupt_manager::instance().handle_interrupt(x, frame); \
}

EXCEPTION_HANDLER_NO_ERROR_CODE(0)
EXCEPTION_HANDLER_NO_ERROR_CODE(1)
EXCEPTION_HANDLER_NO_ERROR_CODE(2)
EXCEPTION_HANDLER_NO_ERROR_CODE(3)
EXCEPTION_HANDLER_NO_ERROR_CODE(4)
EXCEPTION_HANDLER_NO_ERROR_CODE(5)
EXCEPTION_HANDLER_NO_ERROR_CODE(6)
EXCEPTION_HANDLER_NO_ERROR_CODE(7)
EXCEPTION_HANDLER_WITH_ERROR_CODE(8)
EXCEPTION_HANDLER_NO_ERROR_CODE(9)
EXCEPTION_HANDLER_WITH_ERROR_CODE(10)
EXCEPTION_HANDLER_WITH_ERROR_CODE(11)
EXCEPTION_HANDLER_WITH_ERROR_CODE(12)
EXCEPTION_HANDLER_WITH_ERROR_CODE(13)
EXCEPTION_HANDLER_WITH_ERROR_CODE(14)
EXCEPTION_HANDLER_NO_ERROR_CODE(16)
EXCEPTION_HANDLER_WITH_ERROR_CODE(17)
EXCEPTION_HANDLER_NO_ERROR_CODE(18)
EXCEPTION_HANDLER_NO_ERROR_CODE(19)
EXCEPTION_HANDLER_NO_ERROR_CODE(20)
EXCEPTION_HANDLER_WITH_ERROR_CODE(30)
EXCEPTION_HANDLER_WITH_ERROR_CODE(32)

namespace cpu {
  global_descriptor_table gdt;
  interrupt_descriptor_table idt;

  class exception_interrupt_handler : public interrupt_handler {
  public:
    void handle_interrupt(uint8_t int_no, interrupt_frame*) {
      tty::out << "Interrupt: " << static_cast<uint32_t>(int_no) << "\n";
    }
  };

  exception_interrupt_handler eih;
  idt_entry create_idt_entry(uint32_t base, uint16_t cs, uint8_t flags) {
    idt_entry entry = {};
    entry.fields.base_low = base & 0xFFFF;
    entry.fields.base_high = (base >> 16) & 0xFFFF;
    entry.fields.cs = cs;
    entry.fields.zero = 0;
    entry.fields.flags = flags;
    return entry;
  }

  void handle_interrupt(int int_no) {
    tty::out << "Interrupt: " << static_cast<uint32_t>(int_no) << "\n";
  }

  void set_exception_interrupt(int interrupt_number, uint32_t interrupt_address) {
    idt.set(interrupt_number, create_idt_entry(interrupt_address, 0x08, 0x8E));
    interrupt_manager::instance().register_handler(&eih);
  }

  void exceptions_init() {
    EXCEPTION_HANDLER_ENTRY(0)
    EXCEPTION_HANDLER_ENTRY(1)
    EXCEPTION_HANDLER_ENTRY(2)
    EXCEPTION_HANDLER_ENTRY(3)
    EXCEPTION_HANDLER_ENTRY(4)
    EXCEPTION_HANDLER_ENTRY(5)
    EXCEPTION_HANDLER_ENTRY(6)
    EXCEPTION_HANDLER_ENTRY(7)
    EXCEPTION_HANDLER_ENTRY(8)
    EXCEPTION_HANDLER_ENTRY(9)
    EXCEPTION_HANDLER_ENTRY(10)
    EXCEPTION_HANDLER_ENTRY(11)
    EXCEPTION_HANDLER_ENTRY(12)
    EXCEPTION_HANDLER_ENTRY(13)
    EXCEPTION_HANDLER_ENTRY(14)
    EXCEPTION_HANDLER_ENTRY(16)
    EXCEPTION_HANDLER_ENTRY(17)
    EXCEPTION_HANDLER_ENTRY(18)
    EXCEPTION_HANDLER_ENTRY(19)
    EXCEPTION_HANDLER_ENTRY(20)
    EXCEPTION_HANDLER_ENTRY(30)
    EXCEPTION_HANDLER_ENTRY(32)
  }

  void init() {
    exceptions_init();

    io::port::write_and_wait(0x20, 0x11);
    io::port::write_and_wait(0xA0, 0x11);
    io::port::write_and_wait(0x21, 0x20);
    io::port::write_and_wait(0xA1, 0x28);
    io::port::write_and_wait(0x21, 0x04);
    io::port::write_and_wait(0xA1, 0x02);
    io::port::write_and_wait(0x21, 0x01);
    io::port::write_and_wait(0xA1, 0x01);
    io::port::write_and_wait(0x21, 0x0);
    io::port::write_and_wait(0xA1, 0x0);
   
    uint32_t divisor = 1193180 / 50;
    io::port::write(0x43, 0x36);
    uint8_t l = (uint8_t)(divisor & 0xFF);
    uint8_t h = (uint8_t)( (divisor>>8) & 0xFF);
    
    io::port::write_and_wait(0x40, l);
    io::port::write_and_wait(0x40, h);

    gdt.flush();
    idt.flush();
  }
}
