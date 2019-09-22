## XagimaOS
A toy OS

Current status: it boots and setup GDT and IDT. Does PCI probing

### Building
You will need to setup a cross compiler (I have GCC 8.1 setup). The Makefile assumes you have i686-elf-gXX and i686-elf-as in the PATH.

Once you have a cross compiler targetting i686-elf, just do:
```
make
```

You can test using qemu, by running:
```
./tools/qemu.sh
```

You can use `make iso` to create a bootable iso (I haven't tested this in real hw only with qemu).
```
./tools/qemu-iso.sh
```
