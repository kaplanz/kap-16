## Logical OR

Uses:
`ORR`

Mnemonics:
- Logical **OR**

Description:
> Perform a logical OR operation.

Condition Codes:
| Flag     | Modified |
| -------- | -------- |
| Carry    | &empty;  |
| Negative | &check;  |
| Overflow | &empty;  |
| Zero     | &check;  |

Examples:
```assembly
ORR Rx, Ry    ; set Rx <- Rx | Ry
ORR Rx, 0x04  ; set Rx <- Rx | 0x04
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
| `X`      | Destination `Rx` |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |
