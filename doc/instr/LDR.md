#### Load

Uses:
`LDR`

Mnemonics:
- **L**oa**D** into **R**egister

Description:
> Load data from a memory address into a register.

Notes:
- Using the `LDR` instruction, immediate data wider than 7-bits can be loaded
  into a register. However, in doing so we are still performing a memory access.
  (This is converted by the assembler.)

Examples:
```asm
LDR Rx, *Ry     ; load data from address in Ry into Rx
                ; a.k.a: Rx <- *Ry
LDR Rx, *0x40   ; load data from address offset +0x40 into Rx
                ; a.k.a: Rx <- *(PC + 0x40)
LDR Rx, =0d1234 ; load data 0d1234 into Rx from memory
                ; a.k.a: Rx <- 1234
```

Format (Op2):
```
│15  12│11   8│ 7 │6   4│3    0│
┌──────┬──────┬───┬─────┬──────┐
│ 1011 │ XXXX │ 0 │ --- │ YYYY │
└──────┴──────┴───┴─────┴──────┘
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
| `X`      | Destination `Rx` |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |
