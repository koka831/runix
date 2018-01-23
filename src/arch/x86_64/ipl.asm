global start

section .text
bits 32

start:
    mov dword [0xb8000], 0x2f4b2f4f

error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dowrd [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
fin:
    hlt
    jmp fin

section .bss
stack_bottom:
    resb    64
stack_top:
