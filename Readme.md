# Assembly++
## Introduction
Assembly++ is a simplified version of x86-64 assembly
## Usage
To use Assembly++ you need to have a file with the extension .asmpp and you have to have NASM installed on your computer. To compile the file you have to run the following command in the terminal:
```bash
asmpp -f <format> -o <output file name> <input file name>
```
## Syntax
The syntax is very similar to x86-64 assembly, but with some differences. Here is an example of a simple program that prints out a triangle of asterisks:
```c
global _start

section .data
    star: "*"
    newline: "\n"

fn print(message, length){
    mov rax, 1
    mov rdi, 1
    mov rsi, message
    mov rdx, length
    syscall
}

fn exit(code) {
    mov rax, 60
    mov rdi, code
    syscall
}

fn _start() 
{
    xor r12 , r12 
    while (r12  lt 10){
        xor rbx, rbx
        while (rbx lt r12){
            call print(star, len(star))
            add rbx, 1
        }
        call print(newline, len(newline))
        add r12, 1
    }


    call exit(0)
}
```
## Features
- [x] If statements
- [x] While loops
- [x] Functions
- [x] Comments
- [ ] All x86-64 instructions
- [ ] Macros

