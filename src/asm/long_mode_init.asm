global long_mode_start

; defined in src/lib.rs
extern _start

section .text
bits 64
long_mode_start:
  ; load null into all data segment registers
  mov ax, 0
  mov ss, ax
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  
  ; print `OKAY`
  mov rax, 0x2f592f412f4b2f4f
  mov qword [0xb8000], rax
  
  ; call the _start() function
  call _start
  
  ; should not happen
  hlt