#ifndef PCI_MANAGER_H
#define PCI_MANAGER_H

#include <tty/tty.h>
#include <utils/array.h>
#include <utils/singleton.h>
#include <io.h>
#include <string.h>

namespace pci {

  uint16_t config_read(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t bus32  = (uint32_t)bus;
    uint32_t slot32 = (uint32_t)slot;
    uint32_t func32 = (uint32_t)func;
 
    uint32_t address = (uint32_t)((bus32 << 16) | (slot32 << 11) |
        (func32 << 8) | (offset & 0xfc) | ((uint32_t)0x80000000));
 
    io::port::write<uint32_t>(0xCF8, address);
  
    return (uint16_t)((io::port::read<uint32_t>(0xCFC) >> ((offset & 2) * 8)) & 0xffff);
  }

  uint32_t config_read32(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
      uint32_t lo = config_read(bus, slot, func, offset);
      uint32_t hi = config_read(bus, slot, func, offset + 2);
      return hi << 16 | lo;
  }

  struct pci_header {
    uint8_t bus, slot, function;

    uint16_t vendor_id;
    uint16_t device_id;
    uint8_t revision;
    uint8_t prog_if;
    uint8_t subclass_id;
    uint8_t class_id;
    
    uint8_t header_type;

    uint32_t bar[6];

    pci_header(uint8_t _bus, uint8_t _slot, uint8_t _function) :
      bus(_bus), slot(_slot), function(_function) {
      read_config();
    }

    bool is_present() {
      return vendor_id != 0xffff;
    }
  private:
    void read_config() {
      vendor_id = config_read(bus, slot, function, 0x00);
      if(vendor_id == 0xffff) {
        return;
      }
      device_id = config_read(bus, slot, function, 0x02);
      revision = config_read(bus, slot, function, 0x08) & 0x00FF;
      prog_if = (config_read(bus, slot, function, 0x08) & 0xFF00) >> 8;
      subclass_id = config_read(bus, slot, function, 0x0A) & 0x00FF;
      class_id = (config_read(bus, slot, function, 0x0A) & 0xFF00) >> 8;
      header_type = config_read(bus, slot, function, 0x0E) & 0x00FF;
      memset(bar, 0, 4*5);

      // TODO: other header types
      if (header_type == 0x00) {
        for (int i = 0; i < 6; i++) {
          bar[i] = config_read32(bus, slot, function, 0x10 + i * 4);
        }
      }
    }
  };
  
  class pci_manager : public singleton<pci_manager> {
    array<pci_header> pci_devices;
    
    void probe() {
      for(uint8_t bus = 0; bus < 255; bus++) {
        for(uint8_t slot = 0; slot < 32; slot++) {
           for(uint8_t function = 0; function < 8; function++) {
            pci_header pci{bus, slot, function};

            if (pci.is_present()) {
#if DEBUG
              tty::out.hex() << "VendorID:" << pci.vendor_id;
              tty::out.hex() << " DeviceID:" << pci.device_id;
              tty::out.hex() << " ClassID:" << (uint16_t)pci.class_id;
              tty::out.hex() << " SubclassID: " << (uint16_t)pci.subclass_id;
              tty::out.hex() << " Header: " << (uint16_t)pci.header_type;
              tty::out.hex() << " ProgIF:" << (uint16_t)pci.prog_if;
              tty::out.hex() << " BAR0: " << pci.bar[0] << "\n\n";
#endif
              pci_devices.add(pci);
            }
          }
        }
      }
    }
  public:
    void init() {
      probe();
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
