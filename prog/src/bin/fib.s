; File:        fib.s
; Author:      Zakhary Kaplan <https://zakharykaplan.ca>
; Created:     22 Jul 2021
; Version:     0.1.0
; SPDX-License-Identifier: MIT

_main:
    mov r0, 0d1  ; use r0, r1 to store the current...
    mov r1, 0d0  ; ... and previous numbers in the sequence
    mov r3, 0d15 ; use r3 as a counter
loop:
    mov r2, r1   ; move previous number into r2
    mov r1, r0   ; move current number into r1
    add r0, r2   ; compute the next number
    sub r3, 0d1  ; check if we've reached 20...
    bne loop     ; ... loop until we're done
end:
    b   end      ; loop forever
