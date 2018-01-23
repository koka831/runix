section .multiboot_header
header_start:
    DD 0xe85250d6                   ; magic number for multiboot 
    DD 0                            ; architecture
    DD header_end - header_start    ; header length
    DD 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) ; checksum

    DW 0            ; type
    DW 0            ; flags
    DD 8            ; size
header_end:
