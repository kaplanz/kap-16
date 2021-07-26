; File:        fib.s
; Author:      Zakhary Kaplan <https://zakharykaplan.ca>
; Created:     22 Jul 2021
; Version:     0.1.0
; SPDX-License-Identifier: MIT

_main:
    MOV R0, 0d1  ; use R0, R1 to store the current...
    MOV R1, 0d1  ; ... and next numbers in the sequence
    MOV R2, 0d0  ; use R2 as a counter
LOOP:
    ADD R0, R1   ; compute the R2th number
    ADD R1, R0   ; computer the (R2 + 1)st number
    ADD R2, 0b2
    CMP R2, 0b7  ; check if we've reached 7...
    BLT LOOP     ; ... loop until we're done
                 ; R0, R1 now store the 6th, 7th numbers
