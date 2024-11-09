  section .text
  global fib

fib:
  mov rax, rdi
  cmp rdi, 1
  jle .end ; if n <= 1 return n

  sub rdi, 1
  push rdi
  call fib
  pop rdi ; fib(n-1)

  push rax
  sub rdi, 1
  call fib ; fib(n-2)

  pop rcx
  add rax, rcx
.end:
  ret
