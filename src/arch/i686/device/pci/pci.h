#ifndef PCI_MANAGER_H
#define PCI_MANAGER_H

#include <tty/tty.h>
#include <utils/array.h>
#include <utils/singleton.h>
#include <io.h>

namespace pci {

  struct pci_info {
    uint8_t bus, slot, function;

    uint16_t vendor_id;
    uint16_t device_id;
  };

  uint16_t read_word(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t lbus  = (uint32_t)bus;
    uint32_t lslot = (uint32_t)slot;
    uint32_t lfunc = (uint32_t)func;
 
    uint32_t address = (uint32_t)((lbus << 16) | (lslot << 11) |
        (lfunc << 8) | (offset & 0xfc) | ((uint32_t)0x80000000));
 
    io::port::write<uint32_t>(0xCF8, address);
  
    return (uint16_t)((io::port::read<uint32_t>(0xCFC) >> ((offset & 2) * 8)) & 0xffff);
  }

  bool fill_pci_info(pci_info& pci, uint8_t bus, uint8_t slot, uint8_t function) {
    pci.bus = bus;
    pci.slot = slot;
    pci.function = function;

    pci.vendor_id = read_word(bus, slot, function, 0);
    if(pci.vendor_id == 0xffff) {
      return false;
    }
    pci.device_id = read_word(bus, slot, function, 2);
    return true;
  }

  class pci_manager : public singleton<pci_manager> {
    array<pci_info> pci_devices;
    
    array<pci_info> probe() {
      array<pci_info> pci_devices;
      for(uint8_t bus = 0; bus < 255; bus++) {
        for(uint8_t slot = 0; slot < 32; slot++) {
           for(uint8_t function = 0; function < 8; function++) {
            pci_info pci;
            if (fill_pci_info(pci, bus, slot, function)){
              tty::out << "Adding: " << static_cast<uint32_t>(pci.vendor_id) << " " << (uint32_t)pci.device_id << "\n";
              pci_devices.add(pci);
            }
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