; File:        fib.s
; Author:      Zakhary Kaplan <https://zakharykaplan.ca>
; Created:     22 Jul 2021
; Version:     0.1.0
; SPDX-License-Identifier: MIT

_main:
    mov r0, 0d1  ; use r0, r1 to store the current...
    mov r1, 0d1  ; ... and next numbers in the sequence
    mov r2, 0d0  ; use r2 as a counter
loop:
    add r0, r1   ; compute the r2th number
    add r1, r0   ; computer the (r2 + 1)st number
    add r2, 0b2
    cmp r2, 0b7  ; check if we've reached 7...
    blt loop     ; ... loop until we're done
                 ; r0, r1 now store the 6th, 7th numbers
