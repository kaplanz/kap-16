; File:        prime.s
; Author:      Zakhary Kaplan <https://zakharykaplan.ca>
; Created:     02 Sep 2021
; Version:     0.1.0
; SPDX-License-Identifier: NONE

_setup:
    mov sp, 0b1         ; let sp = 0x4000
    lsl sp, 0d14

_main:
    mov r4, 0d32        ; let cur: r4 = 32
_main__loop:
    mov r0, r4
    bl  _is_prime       ; call is_prime(cur) -> res
    cmp r0, 0b0         ; if res == false:
    beq 0x0002          ; else: // res == true
    push r4             ;     push cur
    sub r4, 0d1         ; cur--
    cmp r4, 0d1         ; if cur > 1
    bgt _main__loop     ;     goto loop
_main__end:
    b   _main__end      ; goto end // loop forever

;
; @func: is_prime(n)
; @desc: Check if a number is prime.
;
; @arg0 n: number to check primeness
;
; @ret0 res: boolean result
;
_is_prime:
    push lr
    push r4
    push r5
    mov r4, r0          ; let n: r4 = arg0
    mov r5, r0          ; let count: r5 = n
_is_prime__loop:
    sub r5, 0d1         ; count--
    cmp r5, 0d1         ; if count <= 1:
    ble _is_prime__end  ;     goto done
    mov r0, r4
    mov r1, r5
    bl  _divide         ; call divide(n, count) -> (qut, rem)
    cmp r1, 0b0         ; if rem != 0:
    bne _is_prime__loop ;     goto loop
_is_prime__end:
    mov r0, r1          ; let res: ret0 = rem
    cmp r0, 0b0         ; res = (qot == 0) ? false : true
    beq 0x0002
    mov r0, 0b1
    pop r5
    pop r4
    pop pc              ; return

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
_divide:
    mov r2, r0          ; let num: r2 = arg0
    mov r0, 0d0         ; let count: r0 = 0
_divide__loop:
    cmp r2, r1          ; if num < den:
    blt _divide__end    ;     goto end
    sub r2, r1          ; num -= den
    add r0, 0d1         ; count++
    b   _divide__loop   ; goto loop
_divide__end:           ;
    mov r1, r2          ; let qot: ret0 = count
    mov pc, lr          ; return
