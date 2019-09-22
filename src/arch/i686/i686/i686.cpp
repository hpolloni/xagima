#include <cpu.h>
#include <stdint.h>
#include "gdt.h"
#include "idt.h"
#include "interrupt_frame.h"
#include <assert.h>
#include <io.h>

#define EXCEPTION_HANDLER_ENTRY(x) set_interrupt(x, static_cast<uint32_t>(reinterpret_cast<uintptr_t>(exception_handler_ ## x)));
#define EXCEPTION_HANDLER_WITH_ERROR_CODE(x) extern "C" __attribute__((interrupt)) \
void exception_handler_ ## x (interrupt_frame*, uint32_t ) { \
    assert(!"Exception caught: " # x); \
}

#define EXCEPTION_HANDLER_NO_ERROR_CODE(x) extern "C" __attribute__((interrupt)) \
void exception_handler_ ## x (interrupt_frame*) { \
    assert(!"Exception caught: " # x); \
}

static void timer_callback(interrupt_frame*, uint32_t) {
	io::port::write(0x20, 0x20);

	tty::out << "Tick\n";
}

void init_timer(uint32_t frequency)
{
	uint32_t divisor = 1193180 / frequency;
    io::port::write(0x43, 0x36);

   	// Divisor has to be sent byte-wise, so split here into upper/lower bytes.
   	uint8_t l = (uint8_t)(divisor & 0xFF);
   	uint8_t h = (uint8_t)((divisor>>8) & 0xFF );

   	// Send the frequency divisor.
	io::port::write(0x40, l);
	io::port::write(0x40, h);
	tty::out << "Timer initialized\n";
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

namespace cpu {
    global_descriptor_table gdt;
    interrupt_descriptor_table idt;

    idt_entry create_idt_entry(uint32_t base, uint16_t cs, uint8_t flags) {
        idt_entry entry = {};
        entry.fields.base_low = base & 0xFFFF;
        entry.fields.base_high = (base >> 16) & 0xFFFF;
        entry.fields.cs = cs;
        entry.fields.zero = 0;
        entry.fields.flags = flags;
        return entry;
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
    }

    void init() {
		gdt.flush();
        exceptions_init();
        idt.flush();

		io::port::write(0x20, 0x11);
		io::port::write(0xA0, 0x11);
		io::port::write(0x21, 0x20);
		io::port::write(0xA1, 0x28);
		io::port::write(0x21, 0x04);
		io::port::write(0xA1, 0x02);
		io::port::write(0x21, 0x01);
		io::port::write(0xA1, 0x01);
		io::port::write(0x21, 0x0);
		io::port::write(0xA1, 0x0);

		set_interrupt(32, static_cast<uint32_t>(reinterpret_cast<uintptr_t>(timer_callback)));
		init_timer(50);
	}

    void set_interrupt(int interrupt_number, uint32_t interrupt_address) {
        idt.set(interrupt_number, create_idt_entry(interrupt_address, 0x08, 0x8E));
    }
}
