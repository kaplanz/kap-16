; File:        std.s
; Author:      Zakhary Kaplan <https://zakharykaplan.ca>
; Created:     15 Sep 2021
; Version:     0.1.0
; SPDX-License-Identifier: MIT

;
; @func: divide(num, den)
; @desc: Performs the division of two numbers.
;
; @arg0 num: numerator to divide
; @arg1 den: denominator to divide by
;
; @ret0 qot: quotient,  equal to `num / den`
; @ret1 rem: remainder, equal to `num % den`
;
.func
_divide:
    mov r2, 0d0         ; let rem: r2 = 0
.repeat 16
.begin ; long division
    shr a0, 0d1
    ifcs
    orr r2, 0b1
    cmp r2, a1
    iflt
    goto next
    sub r2, a1
    orr a0, 0b1
next:
.end
    mov a1, a2          ; let qot: ret0 = cur
    mov pc, lr          ; return
.end
