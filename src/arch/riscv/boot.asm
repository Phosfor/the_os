# We only load the global pointer and the stack pointer and switch to rust
# NOTE: other harts must be disabled and bss must be initialized in rust

.text
.section .text.init
.globl _start
_start:
.cfi_startproc
.cfi_undefined ra
.option push
.option norelax
  la gp, _global_pointer
.option pop
  la sp, _stack_end

  add s0, sp, zero
  j rust_boot
.cfi_endproc

.global abort
abort:
  j abort
