## Arithmetic Multiply

Uses:
`MUL`

Mnemonics:
- **MUL**tiplication

Description:
> Perform an arithmetic multiplication.

Condition Codes:
| Flag     | Modified |
| -------- | -------- |
| Carry    | &check;  |
| Negative | &check;  |
| Overflow | &check;  |
| Zero     | &check;  |

Examples:
```assembly
MUL Rx, Ry    ; set Rx <- Rx * Ry
MUL Rx, 0d10  ; set Rx <- Rx * 10
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
| `X`      | Destination `Rx` |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |
