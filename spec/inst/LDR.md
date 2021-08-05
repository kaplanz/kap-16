## Load

Uses:
`LDR`, `POP`

Mnemonics:
- **L**oa**D** into **R**egister
- **POP** from stack

Description:
> Load data from a memory address into a register.

Notes:
- Using the `LDR` instruction, immediate data wider than 7-bits can be loaded
  into a register. However, in doing so we are still performing a memory access.
  (This is converted by the assembler.)
- After using `POP`, the stack pointer is then incremented by 2.

Examples:
```assembly
LDR Rx, *Ry     ; load data from address in Ry into Rx
                ; a.k.a: Rx <- *Ry
LDR Rx, *0x40   ; load data from address offset +0x40 into Rx
                ; a.k.a: Rx <- *(PC + 0x40)
LDR Rx, =0d1234 ; load data 0d1234 into Rx from memory
                ; a.k.a: Rx <- 1234
POP Rx          ; pop Rx from the stack
                ; a.k.a: Rx <- *SP, SP <- SP + 2
```

Format (Op2):
```
│15  12│11   8│ 7 │ 6 │5  4│3    0│
┌──────┬──────┬───┬───┬────┬──────┐
│ 1011 │ XXXX │ 0 │ 0 │ -- │ YYYY │
└──────┴──────┴───┴───┴────┴──────┘
```

Format (Pop):
```
│15  12│11   8│ 7 │ 6 │5      0│
┌──────┬──────┬───┬───┬────────┐
│ 1011 │ XXXX │ 0 │ 1 │ ------ │
└──────┴──────┴───┴───┴────────┘
```

Format (Imm):
```
│15  12│11   8│ 7 │6       0│
┌──────┬──────┬───┬─────────┐
│ 1011 │ XXXX │ 1 │ DDDDDDD │
└──────┴──────┴───┴─────────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `D`      | Immediate data   |
| `I`      | Immediate flag   |
| `P`      | Pop flag         |
| `X`      | Destination `Rx` |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |

Pop (P):
| Flag | Meaning |
| ---- | ------- |
| `0`  | LDR     |
| `1`  | POP     |
