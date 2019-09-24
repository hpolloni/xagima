#include <tty/tty.h>
#include <multiboot.h>
#include <stdlib.h>
#include <assert.h>
#include <mm.h>
#include <cpu.h>
#include <device/device_manager.h>

#define __VERSION "0.0.1"

#define CHECK_FLAG(flags,bit) ((flags) & (1 << (bit)))

extern "C" void kernel_main(uint32_t magic, multiboot_info_t* mbi) {

  assert (magic == MULTIBOOT_BOOTLOADER_MAGIC);
  assert(CHECK_FLAG (mbi->flags, 0));
  
  cpu::init();
  mm::init(mbi->mem_lower * 1024, mbi->mem_upper * 1024);
  
  tty::out << "Welcome to XagimaOS v" << __VERSION << '\n';
  if (CHECK_FLAG (mbi->flags, 2)) {
    tty::out << "Cmdline: " << reinterpret_cast<const char*>(static_cast<uintptr_t>(mbi->cmdline)) << '\n';
  }

  device::init();
}
