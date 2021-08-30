; File:        fib.s
; Author:      Zakhary Kaplan <https://zakharykaplan.ca>
; Created:     22 Jul 2021
; Version:     0.1.0
; SPDX-License-Identifier: MIT

_setup:
    mov r13, 0d1  ; prepare the stack pointer...
    lsl r13, 0d14 ; ...to save each sequence number

_main:
    mov r0, 0d1   ; use r0, r1 to store the current...
    mov r1, 0d0   ; ... and previous numbers in the sequence
    mov r3, 0d15  ; count down remaining to calculate
loop:
    mov r2, r1    ; move previous number into r2
    mov r1, r0    ; move current number into r1
    add r0, r2    ; compute the next number
    push r2       ; save the previous on the stack
    sub r3, 0d1   ; check if we're done...
    bne loop      ; ... loop until then
end:
    b   end      ; loop forever
