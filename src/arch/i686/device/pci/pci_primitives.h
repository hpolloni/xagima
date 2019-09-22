#ifndef PCI_HPP
#define PCI_HPP

#include <io.h>
#include <utils/array.h>
#include <device/device_manager.h>
#include <stdlib.h>

namespace pci {
uint16_t read_word(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t lbus  = (uint32_t)bus;
    uint32_t lslot = (uint32_t)slot;
    uint32_t lfunc = (uint32_t)func;
 
    uint32_t address = (uint32_t)((lbus << 16) | (lslot << 11) |
              (lfunc << 8) | (offset & 0xfc) | ((uint32_t)0x80000000));
 
    io::port::write<uint32_t>(0xCF8, address);
    
    return (uint16_t)((io::port::read<uint32_t>(0xCFC) >> ((offset & 2) * 8)) & 0xffff);
}

uint16_t get_vendor_id(uint16_t bus, uint16_t device, uint16_t function) {
    return read_word(bus,device,function,0);
}

uint16_t get_device_id(uint16_t bus, uint16_t device, uint16_t function) {
    return read_word(bus,device,function,2);
}

uint16_t get_class_id(uint16_t bus, uint16_t device, uint16_t function) {
    uint16_t r0 = read_word(bus,device,function,0xA);
    return (r0 & ~0x00FF) >> 8;
}

uint16_t get_subclass_id(uint16_t bus, uint16_t device, uint16_t function) {
    uint32_t r0 = read_word(bus,device,function,0xA);
    return (r0 & ~0xFF00);
}

}

#endif