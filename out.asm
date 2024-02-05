exit:
  push rbp
  mov rbp, rsp
  mov rax, 60
  mov rdi, rdi
  syscall
  mov rsp, rbp
  pop rbp
  ret
_start:
  push rbp
  mov rbp, rsp
  mov rdi, 69
  call exit
  syscall
  mov rsp, rbp
  pop rbp
  ret
global _start
