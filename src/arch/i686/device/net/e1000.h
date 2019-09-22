#ifndef E1000_H
#define E1000_H

#include <device/device_manager.h>
#include "../pci/pci.h"

class e1000_device : public device::driver {
public:
  void init() override {
    tty::out << "Inside e1000 init function\n";
  }
  
  bool is_present() override {
    return pci::pci_manager::instance().is_present(0x8086, 0x100e);
  }

  const char* name() override {
    return "e1000 ethernet controller";
  }
};

#endif
