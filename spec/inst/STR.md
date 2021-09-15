## Store

Uses:
`STR`, `PUSH`

Mnemonics:
- **ST**ore from **R**egister
- **PUSH** into stack

Description:
> Store data from a register into a memory address.

Condition Codes: None

Notes:
- Before using `PUSH`, the stack pointer is first decremented by 2.
- May use a symbol optionally instead of an address offset

Examples:
```assembly
STR Rx, &Ry    ; load data from Rx into address in Ry
               ; a.k.a: *Ry <- Rx
STR Rx, &0x40  ; load data from Rx into address offset 0x40
               ; a.k.a: *(PC + 0x40) <- Rx
STR Rx, _foo   ; load data from Rx into address at symbol `_foo`
               ; a.k.a: *_foo <- Rx
PUSH Rx        ; push Rx onto the stack
               ; a.k.a: SP <- SP - 2, *SP <- Rx
```

Format (Op2):
```
│15  12│11   8│ 7 │ 6 │5  4│3    0│
┌──────┬──────┬───┬───┬────┬──────┐
│ 0010 │ XXXX │ 0 │ 0 │ -- │ YYYY │
└──────┴──────┴───┴───┴────┴──────┘
```

Format (Push):
```
│15  12│11   8│ 7 │ 6 │5      0│
┌──────┬──────┬───┬───┬────────┐
│ 0010 │ XXXX │ 0 │ 1 │ ------ │
└──────┴──────┴───┴───┴────────┘
```

Format (Imm):
```
│15  12│11   8│ 7 │6       0│
┌──────┬──────┬───┬─────────┐
│ 0010 │ XXXX │ 1 │ DDDDDDD │
└──────┴──────┴───┴─────────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `D`      | Immediate data   |
| `X`      | Destination `Rx` |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |

Push (P):
| Flag | Meaning |
| ---- | ------- |
| `0`  | STR     |
| `1`  | PUSH    |
