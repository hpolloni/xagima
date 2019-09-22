#include <device/device_manager.h>
#include "pci/pci.h"
#include <tty/tty.h>

#include "net/e1000.h"

namespace device {
    void init() {
        pci::pci_manager::instance().init();

        // TODO: load driver from somewhere?
       
        auto dev = new e1000_device{};
        if (dev->is_present()) {
            tty::out << "Detected \"" << dev->name() << "\"\n";
            tty::out << "Initializing \"" << dev->name() << "\"\n";
            dev->init();
        }
    }
}
