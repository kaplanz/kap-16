; File:        fib.s
; Author:      Zakhary Kaplan <https://zakhary.dev>
; Created:     22 Jul 2021
; Version:     0.1.0
; SPDX-License-Identifier: MIT

.entry
.func
_main:
    ldr sp, =0x4000
    mov g0, 0d1         ; let cur:  g0 = 1
    mov g1, 0d0         ; let fib1: g1 = 0
    mov g2, 0d0         ; let fib0: g2 = 0
    mov g3, 0d25        ; let idx:  g3 = 25
    cmp g3, 0d0
loop:
    ifeq                ; while !idx:
    goto end
    mov g2, g1          ;   fib0 = fib1
    mov g1, g0          ;   fib1 = cur
    add g0, g2          ;   cur = fib0 + fib1
    push g2             ;   // save the previous on the stack
    sub g3, 0d1         ;   idx--
    goto loop
end:
    hlt                 ; halt the processor
.end
