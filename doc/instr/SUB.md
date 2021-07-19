#### Subtraction

Uses:
`SUB`, `RSB`

Mnemonics:
- **SUB**traction
- **R**everse **S**u**B**traction

Description:
> Perform an arithmetic subtraction.

Examples:
```asm
SUB Rx, Ry   ; set Rx <- Rx - Ry
SUB Rx, 0d10 ; set Rx <- Rx - 10
RSB Rx, Ry   ; set Rx <- Ry - Rx
RSB Rx, 0d10 ; set Rx <- 10 - Rx
```

Format (Op2):
```
│15 13│12 │11   8│ 7 │6   4│3    0│
┌─────┬───┬──────┬───┬─────┬──────┐
│ 100 │ M │ XXXX │ 0 │ --- │ YYYY │
└─────┴───┴──────┴───┴─────┴──────┘
```

Format (Imm):
```
│15 13│12 │11   8│ 7 │6       0│
┌─────┬───┬──────┬───┬─────────┐
│ 100 │ M │ XXXX │ 1 │ DDDDDDD │
└─────┴───┴──────┴───┴─────────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `D`      | Immediate data   |
| `I`      | Immediate flag   |
| `M`      | Mode flags       |
| `X`      | Destination `Rx` |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |

Mode (M):
| Flag | Meaning |
| ---- | ------- |
| `0`  | SUB     |
| `1`  | RSB     |
