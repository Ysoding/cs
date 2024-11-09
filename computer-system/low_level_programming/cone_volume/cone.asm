default rel
section .text
global volume

; xmm0 = radius (and return)
; xmm1 = height

volume:
  mulss xmm0, xmm0 ; r^2
  mulss xmm0, xmm1 ; r^2 * h
  mulss xmm0, [pi_3] ; r^2 * h * pi / 3
  ret
pi_3:
  dd 1.0472