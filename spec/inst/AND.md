## Logical AND

Uses:
`AND`

Mnemonics:
- Logical **AND**

Description:
> Perform a logical AND operation.

Condition Codes:
| Flag     | Modified |
| -------- | -------- |
| Carry    | &empty;  |
| Negative | &check;  |
| Overflow | &empty;  |
| Zero     | &check;  |

Examples:
```assembly
AND Rx, Ry    ; set Rx <- Rx & Ry
AND Rx, 0x0f  ; set Rx <- Rx & 0x0f
```

Format (Op2):
```
│15  12│11   8│ 7 │6   4│3    0│
┌──────┬──────┬───┬─────┬──────┐
│ 1110 │ XXXX │ 0 │ --- │ YYYY │
└──────┴──────┴───┴─────┴──────┘
```

Format (Imm):
```
│15  12│11   8│ 7 │6       0│
┌──────┬──────┬───┬─────────┐
│ 1110 │ XXXX │ 1 │ DDDDDDD │
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
