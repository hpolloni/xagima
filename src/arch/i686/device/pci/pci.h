#ifndef PCI_MANAGER_H
#define PCI_MANAGER_H

#include <tty/tty.h>
#include <utils/array.h>
#include <utils/singleton.h>

#include <device/device_manager.h>
#include "pci_primitives.h"

namespace pci {
    class pci_manager : public singleton<pci_manager> {
        struct pci_info {
            uint16_t vendor_id;
            uint16_t device_id;
        };
        array<pci_info> pci_devices;
        
        array<pci_info> probe() {
            array<pci_info> pci_devices;
            for(uint8_t bus = 0; bus < 255; bus++) {
                for(uint8_t slot = 0; slot < 32; slot++) {
                   for(uint8_t function = 0; function < 8; function++) {
                        uint16_t vendor = get_vendor_id(bus, slot, function);
                        if(vendor == 0xffff) continue;
                        uint16_t device = get_device_id(bus, slot, function);
                        pci_info pci;
                        pci.vendor_id = vendor;
                        pci.device_id = device;
                        pci_devices.add(pci);
                    }
                }
            }
            return pci_devices;
        }
    public:
        void init() {
            this->pci_devices = probe();
        }

        bool is_present(uint16_t vendor_id, uint16_t device_id) const {
            for (auto it : pci_devices) {
                if (it.vendor_id == vendor_id && it.device_id == device_id) {
                    return true;
                }
            }
            return false;
        }

        const array<pci_info> get_pci_info() const {
            return pci_devices;
        }
    };
}
#endif