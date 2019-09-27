#include <device/device_manager.h>
#include "pci/pci.h"
#include <tty/tty.h>

#include "net/e1000.h"
#include "timer/timer.h"

namespace device {
  void init() {
    pci::pci_manager::instance().init();

    // TODO:these need to be dynamically loaded
    array<driver*> drivers;
    drivers.add(new timer_device{});
    drivers.add(new e1000_device{});
    ///
    
    for (auto dev : drivers) {
      if (dev->is_present()) {
        tty::out << "Detected \"" << dev->name() << "\"\n";
        tty::out << "Initializing \"" << dev->name() << "\"\n";
        dev->init();
      }
    }
  }
}
