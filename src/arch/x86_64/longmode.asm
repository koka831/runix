global long_mode

section .text
bits 64

long_mode:
    ;; init segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ;; call Rust object
    extern boot
    call boot

    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt
