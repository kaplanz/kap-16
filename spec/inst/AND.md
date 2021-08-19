## Logical AND

Uses:
`AND`

Mnemonics:
- Logical **AND**

Description:
> Perform a logical AND operation.

Condition Codes:
- Cleared: carry, overflow
- Set: negative, zero

Examples:
```assembly
AND Rx, Ry   ; set Rx <- Rx & Ry
AND Rx, 0x0F ; set Rx <- Rx & 0x0F
```

Format (Op2):
```
│15  12│11   8│ 7 │6   4│3    0│
┌──────┬──────┬───┬─────┬──────┐
│ 0110 │ XXXX │ 0 │ --- │ YYYY │
└──────┴──────┴───┴─────┴──────┘
```

Format (Imm):
```
│15  12│11   8│ 7 │6       0│
┌──────┬──────┬───┬─────────┐
│ 0110 │ XXXX │ 1 │ DDDDDDD │
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
