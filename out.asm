exit:
  push rbp
  mov rbp, rsp
  mov rax, 60
  mov rdi, [rbp + 16]
  syscall
  mov rsp, rbp
  pop rbp
  ret
_start:
  push rbp
  mov rbp, rsp
  push 23
  call exit
  add rsp, 8
  mov rsp, rbp
  pop rbp
  ret
global _start
