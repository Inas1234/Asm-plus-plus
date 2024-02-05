print:
  push rbp
  mov rbp, rsp
  mov rax, 1
  mov rdi, 1
  mov rsi, [rbp + 16]
  mov rdx, 11
  syscall
  mov rsp, rbp
  pop rbp
  ret
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
  push msg
  call print
  add rsp, 8
  push 0
  call exit
  add rsp, 8
  mov rsp, rbp
  pop rbp
  ret
global _start
section .data
  msg db "Hello World" , 10
