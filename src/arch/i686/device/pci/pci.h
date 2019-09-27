#ifndef PCI_MANAGER_H
#define PCI_MANAGER_H

#include <tty/tty.h>
#include <utils/array.h>
#include <utils/singleton.h>
#include <io.h>

namespace pci {

  struct pci_header {
    uint8_t bus, slot, function;

    uint16_t vendor_id;
    uint16_t device_id;
    uint8_t revision;
    uint8_t prog_if;
    uint8_t subclass_id;
    uint8_t class_id;
    
    uint8_t header_type;
  };

  uint16_t config_read(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t bus32  = (uint32_t)bus;
    uint32_t slot32 = (uint32_t)slot;
    uint32_t func32 = (uint32_t)func;
 
    uint32_t address = (uint32_t)((bus32 << 16) | (slot32 << 11) |
        (func32 << 8) | (offset & 0xfc) | ((uint32_t)0x80000000));
 
    io::port::write<uint32_t>(0xCF8, address);
  
    return (uint16_t)((io::port::read<uint32_t>(0xCFC) >> ((offset & 2) * 8)) & 0xffff);
  }

  bool fill_pci_info(pci_header& pci, uint8_t bus, uint8_t slot, uint8_t function) {
    pci.bus = bus;
    pci.slot = slot;
    pci.function = function;

    pci.vendor_id = config_read(bus, slot, function, 0x00);
    if(pci.vendor_id == 0xffff) {
      return false;
    }
    pci.device_id = config_read(bus, slot, function, 0x02);
    pci.revision = config_read(bus, slot, function, 0x08) & 0x00FF;
    pci.prog_if = (config_read(bus, slot, function, 0x08) & 0xFF00) >> 8;
    pci.subclass_id = config_read(bus, slot, function, 0x0A) & 0x00FF;
    pci.class_id = (config_read(bus, slot, function, 0x0A) & 0xFF00) >> 8;
    pci.header_type = config_read(bus, slot, function, 0x0E) & 0x00FF;
    return true;
  }

  class pci_manager : public singleton<pci_manager> {
    array<pci_header> pci_devices;
    
    array<pci_header> probe() {
      array<pci_header> pci_devices;
      for(uint8_t bus = 0; bus < 255; bus++) {
        for(uint8_t slot = 0; slot < 32; slot++) {
           for(uint8_t function = 0; function < 8; function++) {
            pci_header pci;
            if (fill_pci_info(pci, bus, slot, function)) {
              tty::out.hex() << "VendorID:" << pci.vendor_id;
              tty::out.hex() << " DeviceID:" << pci.device_id;
              tty::out.hex() << " ClassID:" << (uint16_t)pci.class_id;
              tty::out.hex() << " SubclassID: " << (uint16_t)pci.subclass_id;
              tty::out.hex() << " Header: " << (uint16_t)pci.header_type;
              tty::out.hex() << " ProgIF:" << (uint16_t)pci.prog_if << "\n\n";
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
  };
}
#endif