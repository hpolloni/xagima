# TODO: Make this portable
ARCHDIR=arch/$(ARCH)
OUTDIR=$(SYSROOT)/boot
LDFLAGS=--sysroot=$(SYSROOT) -ffreestanding -O2 -nostdlib -lgcc

# Order is important: crti, crtbegin, kernel, crtend, crtn
CRTBEGIN_OBJ:=$(ARCHDIR)/crti.o $(shell $(CC) $(CFLAGS) -print-file-name=crtbegin.o)
CRTEND_OBJ:=$(shell $(CC) $(CFLAGS) -print-file-name=crtend.o) $(ARCHDIR)/crtn.o 

include $(ARCHDIR)/make.config

OBJS=$(ARCH_OBJS) \
	kernel/kernel.o \
	kernel/gcc.o \
	kernel/abort.o \
	kernel/errno.o \
	kernel/malloc.o \
	kernel/sbrk.o \
	kernel/string.o \
	
all: kernel.bin

kernel.bin: $(OBJS) $(ARCHDIR)/linker.ld $(ARCHDIR)/crti.o $(ARCHDIR)/crtn.o
	mkdir -p $(OUTDIR)
	$(CC) -T $(ARCHDIR)/linker.ld -o $(OUTDIR)/$@ $(CRTBEGIN_OBJ) $(OBJS) $(CRTEND_OBJ) $(LDFLAGS)
	grub-file --is-x86-multiboot $(OUTDIR)/$@

$(ARCHDIR)/device/cpu/idt.o:
	$(CXX) -mgeneral-regs-only -c $(ARCHDIR)/device/cpu/idt.cpp  -o $@ $(CXXFLAGS)

kernel/malloc.o:
	$(CC) -c kernel/malloc.c -o $@ $(CFLAGS) -DHAVE_MMAP=0 -DLACKS_SYS_TYPES_H -DLACKS_TIME_H -DLACKS_UNISTD_H -DHAVE_MORECORE -DLACKS_SYS_PARAM_H -DUSE_LOCKS=0 -DNO_MALLOC_STATS

.cpp.o:
	$(CXX) -c $< -o $@ $(CXXFLAGS)

.S.o:
	$(AS) $< -o $@

clean:
	rm -f kernel.bin
	find . -name \*.o -exec rm '{}' \;

.PHONY: all prepare clean
