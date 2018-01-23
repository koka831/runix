
default:
	make kernel.bin

kernel.bin: multiboot.o ipl.o
	ld -n -o kernel.bin -T linker.ld multiboot.o ipl.o

multiboot.o: multiboot.asm
	nasm -f elf64 multiboot.asm

ipl.o: ipl.asm
	nasm -f elf64 ipl.asm

clean:
	rm kernel.bin *.o
