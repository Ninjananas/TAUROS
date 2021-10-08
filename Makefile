arch ?= x86_64
kernel := build/tauros-$(arch).bin
iso := build/tauros-$(arch).iso
test-kernel := build/test-tauros-$(arch).bin
test-iso := build/test-tauros-$(arch).iso

tauros_dir := tauros
target ?= $(arch)-tauros
rust_os := $(tauros_dir)/target/$(target)/debug/libtauros.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg

target ?= $(arch)-tauros

assembly_source_files := $(wildcard src/arch/$(arch)/*.S)
assembly_object_files := $(patsubst src/arch/$(arch)/%.S, build/arch/$(arch)/%.o, $(assembly_source_files))


QEMU_RAM = 5G

.PHONY: all clean run run-debug run-debug-gdb iso kernel

all: $(kernel)

clean:
	@rm -r build
	@rm $(rust_os)

run: $(iso)
	@qemu-system-x86_64 -m $(QEMU_RAM) -cdrom $(iso)&
	@vinagre 127.0.0.1:5900; pkill qemu-system-x86

test: $(iso)
	@qemu-system-x86_64 -m $(QEMU_RAM) -cdrom $(iso)&
	@vinagre 127.0.0.1:5900; pkill qemu-system-x86

run-debug: $(iso)
	@qemu-system-x86_64 -m $(QEMU_RAM) -monitor stdio -cdrom $(iso)

run-debug-gdb: $(iso)
	@qemu-system-x86_64 -m $(QEMU_RAM) -S -s -cdrom $(iso)&
	@vinagre 127.0.0.1:5900 && pkill qemu-system-x86

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -d /usr/lib/grub/i386-pc -o $(iso) build/isofiles # 2> /dev/null
	@rm -r build/isofiles

$(kernel): kernel $(assembly_object_files) $(linker_script) $(rust_os)
	@ld -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.S
	@mkdir -p $(shell dirname $@)
	@gcc -c $< -o $@

$(rust_os): kernel

kernel:
	@cd $(tauros_dir); cargo build -Z no-index-update
