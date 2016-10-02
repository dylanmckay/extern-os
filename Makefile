TARGET = i686-unknown-unknown

BOOT_OBJ = $(shell find target/$(TARGET) -name "boot.o")
KERNEL_LIB = target/$(TARGET)/debug/libextern_os.a
LINKER_SCRIPT = support/linker.ld

all: kernel

clean:
	cargo clean

rebuild: clean all

kernel:
	cargo build --target $(TARGET).json
	i686-elf-gcc -T $(LINKER_SCRIPT) -o kernel.elf -ffreestanding -O2 -nostdlib $(BOOT_OBJ) $(KERNEL_LIB) -lgcc

run: kernel
	qemu-system-i386 -kernel kernel.elf

