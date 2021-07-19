#### Store

Uses:
`STR`

Mnemonics:
- **ST**ore from **R**egister

Description:
> Store data from a register into a memory address.

Examples:
```asm
STR Rx, &Ry   ; load data from Rx into address in Ry
              ; a.k.a: *Ry <- Rx
STR Rx, &0x40 ; load data from Rx into address offset +0x40
              ; a.k.a: *(PC + 0x40) <- Rx
```

Format (Op2):
```
│15  12│11   8│ 7 │6   4│3    0│
┌──────┬──────┬───┬─────┬──────┐
│ 1101 │ XXXX │ 0 │ --- │ YYYY │
└──────┴──────┴───┴─────┴──────┘
```

Format (Imm):
```
│15  12│11   8│ 7 │6       0│
┌──────┬──────┬───┬─────────┐
│ 1101 │ XXXX │ 1 │ DDDDDDD │
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
