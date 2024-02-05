exit:
  push rbp
  mov rbp, rsp
  mov rax, 60
  mov rdi, [rbp + 24]
  syscall
  mov rsp, rbp
  pop rbp
  ret
_start:
  push rbp
  mov rbp, rsp
  push 44
  push 23
  call exit
  add rsp, 16
  mov rsp, rbp
  pop rbp
  ret
global _start
