global start

section .text
bits 32

start:
    MOV esp, stack_top
    ; print 'OK' to screen
    MOV DWORD [0xb8000], 0x2f4b2f4f
chekck_boot:
    cmp EAX, 0x36d76289
    JNE .no_multiboot
    RET
.no_multiboot:
    MOV al, "0"
    jmp error

error:
    MOV DWORD [0xb8000], 0x4f524f45
    MOV DWORD [0xb8004], 0x4f3a4f52
    MOV DWORD [0xb8008], 0x4f204f20
    MOV BYTE  [0xb800a], al
fin:
    hlt
    jmp fin

section .bss
stack_bottom:
    resb    64
stack_top:
