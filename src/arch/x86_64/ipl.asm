global start

section .text
bits 32

start:
    MOV DWORD [0xb8000], 0x2f4b2f4f
fin:
    hlt
    jmp fin
