; Set up the multiboot header
MBALIGN  equ  1<<0              ; align loaded modules on page boundaries
MEMINFO  equ  1<<1              ; provide memory map
FLAGS    equ  MBALIGN | MEMINFO ; this is the Multiboot 'flag' field
MAGIC    equ  0x1BADB002        ; 'magic number' lets bootloader find the header
CHECKSUM equ -(MAGIC + FLAGS)   ; checksum of above, to prove we are multiboot

section .multiboot
align 4
  dd MAGIC
  dd FLAGS
  dd CHECKSUM

section .bss
align 4
global stack_bottom
stack_bottom:
resb 16384 ; 16 KiB
global stack_top
stack_top:

; The linker script specifies _start as the entry point to the kernel and the
; bootloader will jump to this position once the kernel has been loaded. It
; doesn't make sense to return from this function as the bootloader is gone.
; Declare _start as a function symbol with the given symbol size.
section .text
global _start:function (_start.end - _start)
_start:
  ; Set up the stack.
  mov esp, stack_top

  ; Start the kernel.
  extern kernel_main
  call kernel_main

  cli
.hang: hlt
  jmp .hang

.end:

