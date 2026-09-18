.section .multiboot2_header, "a", @progbits
.align 8
header_start:
    .long 0xe85250d6
    .long 0
    .long header_end - header_start
    .long 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    .align 8
    .word 0
    .word 0
    .long 8
header_end:

.section .text
.code32
.global start
.extern kernel_main

start:
    mov esp, offset stack_top
    call kernel_main
    cli
    hlt

.section .bss
.align 16
stack_bottom:
    .space 65536
stack_top:
