KERNEL_ELF := target/riscv64gc-unknown-none-elf/release/async-os
KERNEL_BIN := $(KERNEL_ELF).bin
BOOTLOADER := bootloader/rustsbi-qemu-2024-03-24.bin
KERNEL_ENTRY_PA := 0x80200000

LOG ?= INFO

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

elf:
	LOG=$(LOG) cargo build --release --target riscv64gc-unknown-none-elf

$(KERNEL_BIN): elf
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

QEMU_ARGS := -d in_asm,int,mmu,pcall,cpu_reset,guest_errors \
	        -D /tmp/qemu.log \
			-machine virt \
			-smp 4 \
			-m 4G \
			-nographic \
			-bios $(BOOTLOADER) \
			-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \

run: $(KERNEL_BIN)
	@qemu-system-riscv64 $(QEMU_ARGS)

gdbserver: $(KERNEL_BIN)
	@qemu-system-riscv64 $(QEMU_ARGS) -s -S

gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

clean:
	@cargo clean