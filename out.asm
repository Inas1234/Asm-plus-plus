exit:
  ; Argument: arg1
  mov rax, 60
  mov rdi, 69
  syscall
  ret
_start:
  call exit
  push rax
  syscall
  ret
global _start
