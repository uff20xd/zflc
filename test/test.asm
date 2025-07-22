format ELF64 executable

segment readable executable
entry main
main:
  jmp print

print:
  mov rax, 1
  mov rdi, 1
  mov rsi, msg
  mov rdx, msg_len
  syscall
  jmp exit

exit:
  mov rax, 60
  mov rdi, 0
  syscall


segment readable writable

msg db "Hello, World!", 10
msg_len = $ - msg
