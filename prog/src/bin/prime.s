; File:        prime.s
; Author:      Zakhary Kaplan <https://zakharykaplan.ca>
; Created:     02 Sep 2021
; Version:     0.1.0
; SPDX-License-Identifier: MIT

.init sp 0x4000

.entry
.func
_main:
    mov g0, 0d32        ; let cur: g0 = 32
loop:
    mov a0, g0
    call _is_prime      ; call is_prime(cur) -> res
    cmp a0, 0b0
    ifne                ; if res != false:
    push g0             ;   push cur
    sub g0, 0d1         ; cur--
    cmp g0, 0d1
    ifgt                ; if cur > 1:
    goto loop           ;   goto loop
end:
    hlt                 ; halt the processor
.end

;
; @func: is_prime(n)
; @desc: Check if a number is prime.
;
; @arg0 nun: number to check primeness
;
; @ret0 res: boolean result
;
.func
_is_prime:
    push lr
    push g0
    push g1
    mov g0, a0          ; let num: g0 = arg0
    mov g1, g0          ; let cur: g1 = num
loop:
    sub g1, 0d1         ; cur--
    cmp g1, 0d1
    ifle                ; if cur <= 1:
    goto end            ;   goto end
    mov a0, g0
    mov a1, g1
    call _divide        ; call divide(num, cur) -> (qut, rem)
    cmp a1, 0b0
    ifne                ; if rem != 0:
    goto loop           ;   goto loop
end:
    mov a0, a1          ; let res: ret0 = (bool)rem
    cmp a0, 0b0
    ifne
    mov a0, 0b1
    pop g1
    pop g0
    pop pc              ; return
.end

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
    mov a2, a0          ; let num: a2 = arg0
    mov a0, 0d0         ; let cur: a0 = 0
loop:
    cmp a2, a1
    iflt                ; while num >= den {
    goto end
    sub a2, a1          ;   num -= den
    add a0, 0d1         ;   cur++
    goto loop           ; }
end:
    mov a1, a2          ; let qot: ret0 = cur
    mov pc, lr          ; return
.end
