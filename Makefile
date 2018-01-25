arch ?= x86_64
target ?= $(arch)-runix
iso := build/$(arch).iso
kernel := build/$(arch).bin
runix := target/$(target)/debug/librunix.a

linker_ld := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
asm_src := $(wildcard src/arch/$(arch)/*.asm)
asm_obj := $(patsubst src/arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(asm_src))

.PHONY: all clean run kernel

all: $(kernel)

clean:
	@rm -r build

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/iso/boot/grub
	@cp $(kernel) /build/iso/boot/kernel.bin
	@cp $(grub_cfg) /build/iso/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null

$(kernel): kernel $(runix) $(asm_obj) $(linker_ld)
	@ld -n -T $(linker_ld) -o $(kernel) $(asm_obj) $(runix)

kernel:
	@xargo build --target $(target)


build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@


