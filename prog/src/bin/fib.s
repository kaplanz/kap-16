; File:        fib.s
; Author:      Zakhary Kaplan <https://zakharykaplan.ca>
; Created:     22 Jul 2021
; Version:     0.1.0
; SPDX-License-Identifier: MIT

_setup:
    mov sp, 0b1         ; let sp = 0x4000
    lsl sp, 0d14

_main:
    mov r0, 0d1         ; let cur:   r0 = 1
    mov r1, 0d0         ; let fib1:  r1 = 0
    mov r2, 0d0         ; let fib0:  r2 = 0
    mov r3, 0d25        ; let count: r3 = 25
_main__loop:
    mov r2, r1          ; fib0 = fib1
    mov r1, r0          ; fib1 = curr
    add r0, r2          ; cur += fib0 // cur <- fib0 + fib1
    push r2             ; // save the previous on the stack
    sub r3, 0d1         ; count--
    bne _main__loop     ; if count != 0: goto loop
_main__end:
    b   _main__end      ; goto end // loop forever
