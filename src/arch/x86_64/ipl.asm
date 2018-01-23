global start

section .text
bits 32

start:
    mov esp, stack_top

    call check_multiboot
    call check_cpuid
    call check_long_mode

    mov dword [0xb8000], 0x2f4b2f4f

check_multiboot: 
    ; make sure kernel got loaded by a mutliboot-compliant bootloader
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "0"
    jmp error


check_cpuid:
    ; copy flags
    pushfd
    pop eax

    mov ecx, eax

    push eax
    popfd

    pushfd
    pop eax

    push ecx
    popfd

    cmp eax, ecx
    je .no_cpuid
.no_cpuid:
    mov al, "1"
    jmp error

check_long_mode:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode

    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode
    ret
.no_long_mode:
    mov al, "2"
    jmp error

error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
fin:
    hlt
    jmp fin

section .bss
stack_bottom:
    resb    64
stack_top:
