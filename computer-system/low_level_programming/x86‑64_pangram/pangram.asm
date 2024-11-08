; bool is_pangram(char *line) {
;   int val = 0;

;   while (*line != '\n') {
;     if (isalpha(*line)) {
;       val |= 1 << (*line - 'a');
;     }
;     line++;
;   }

;   return val == 0x3ffffff;
; }


; #define MASK 0x07fffffe
; bool is_pangram(char *s) {
;   uint32_t bs = 0;
;   char c;
;   while ((c = *s++) != '\0') {
;     if (c < '@')
;       continue; // ignore first 64 chars in ascii table
;     bs |= 1 << (c & 0x1f);
;   }
;   return (bs & MASK) == MASK;
; }



  %define MASK 0x07fffffe
  section .text
  global pangram
pangram:
; rdi: source string
  xor ecx, ecx ; val

.loop:
  movzx edx, byte[rdi]
  cmp edx, 0
  je .end
  add rdi, 1
  cmp edx, '@'
  jl .loop
  bts ecx, edx
  jmp .loop

.end:
  xor eax, eax
  and ecx, MASK
  cmp ecx, MASK
  sete al ; set al to 1 if equal, 0 otherwise
  ret

