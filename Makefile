# TODO: Make this portable
export SYSROOT=$(shell pwd)/sysroot
export ARCH=i686
export HOST=$(ARCH)-elf
export AR=$(HOST)-ar
export CC=$(HOST)-gcc
export CXX=$(HOST)-g++
export AS=$(HOST)-as
export CFLAGS=-g -mgeneral-regs-only -ffreestanding -fstack-protector-all -O2 -Wall -Wextra --sysroot=$(SYSROOT) -I$(SYSROOT)/usr/include
export CXXFLAGS=$(CFLAGS) -fno-exceptions -fno-rtti -std=c++17

all: kernel 

sysroot:
	mkdir -p $(SYSROOT)/usr/include
	cp -rf src/include/* $(SYSROOT)/usr/include
	cp -rf src/arch/$(ARCH)/include/* $(SYSROOT)/usr/include

kernel: sysroot
	make -C src
	
clean:
	rm -fr $(SYSROOT)
	make -C src clean
	rm -fr isodir
	rm -fr xagima.iso

iso: 
	mkdir -p isodir
	mkdir -p isodir/boot
	mkdir -p isodir/boot/grub
	cp sysroot/boot/kernel.bin isodir/boot/kernel.bin
	cp grub.cfg isodir/boot/grub/grub.cfg
	grub-mkrescue -o xagima.iso isodir

.PHONY: all clean sysroot iso


