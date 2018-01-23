global start

section .text
bits 32

start:
    mov esp, stack_top

    call check_multiboot
    call check_cpuid
    call check_long_mode

    call init_page_table
    call init_paging

    mov dword [0xb8000], 0x2f4b2f4f
    hlt

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
    xor eax, 1 << 21
    push eax
    popfd

    pushfd
    pop eax

    push ecx
    popfd
    xor eax, ecx
    jz .no_cpuid
    ret
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

init_page_table:
    mov eax, p3_table   ; map first p4 entry to p3
    or eax, 0b11
    mov [p4_table], eax

    mov eax, p2_table   ; map first p3 to p2
    or eax, 0b11
    mov [p3_table], eax

    mov ecx, 0

.map_p2_table:
    mov eax, 0x200000   ; 2MiB
    mul ecx
    or eax, 0b10000011
    mov [p2_table + ecx * 8], eax

    inc ecx
    cmp ecx, 512        ; check if whole P2(512MiB) is mapped
    jne .map_p2_table

    ret

init_paging:
    mov eax, p4_table
    mov cr3, eax

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    mov ecx, cr0
    or eax, 1 << 31
    mov cr0, eax
    ret

error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
fin:
    hlt
    jmp fin

section .bss
align 4096
;; page table
p4_table:
    resb 4096   ;; Page-Map Level-4 table / PML4
p3_table:
    resb 4096   ;; Page-Directory Pointer table / PDP
p2_table:
    resb 4096   ;; Page-Directory Table / PD
p1_table:
    resb 4096   ;; Page Table / PT

stack_bottom:
    resb    64
stack_top:
