arch ?= x86_64


default:
	make kernel.iso

run: kernel.iso
	@qemu-system-x86_64 -cdrom kernel.iso

kernel.iso: kernel.bin grub.cfg
	mkdir -p build/boot/grub
	cp kernel.bin build/boot/kernel.bin
	cp grub.cfg build/boot/grub/grub.cfg
	grub-mkrescue -o kernel.iso build/ 2> /dev/null

kernel.bin: multiboot.o ipl.o
	ld -n -o kernel.bin -T linker.ld multiboot.o ipl.o

multiboot.o: multiboot.asm
	nasm -f elf64 multiboot.asm

ipl.o: ipl.asm
	nasm -f elf64 ipl.asm

clean:
	rm *.bin *.o *.iso
	rm -rf build
