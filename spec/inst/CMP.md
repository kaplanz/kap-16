## Compare

Uses:
`CMP`, `CMN`, `TST`, `TEQ`

Mnemonics:
- **C**o**MP**are
- **C**o**M**pare **N**egative (ADD)
- **T**e**ST** (AND)
- **T**e**S**t e**q**uql (XOR)

Description:
> Perform a comparison between the two operands, setting condition code flags.
> No registers are modified by this instruction.

Condition Codes: (see [`ADD`](./ADD.md), [`SUB`](./SUB.md), [`AND`](./AND.md), [`XOR`](./XOR.md))
| Flag     | Modified |
| -------- | -------- |
| Carry    | &check;  |
| Negative | &check;  |
| Overflow | &check;  |
| Zero     | &check;  |

Operation:
| Instruction | Operation       |
| ----------- | --------------- |
| `CMP`       | `Rx - Ry` (SUB) |
| `CMN`       | `Rx + Ry` (ADD) |
| `TST`       | `Rx & Ry` (AND) |
| `TEQ`       | `Rx ^ Ry` (XOR) |

Examples:
```assembly
CMP Rx, Ry    ; compare Rx - Ry
CMN Rx, 0x10  ; compare Rx + 0x10
TST Rx, Ry    ; compare Rx & Ry
TEQ Rx, 0x77  ; compare Rx ^ 0x77
```

Format (Op2):
```
│15  │13  │11   8│ 7 │6   4│3    0│
┌────┬────┬──────┬───┬─────┬──────┐
│ 10 │ MM │ XXXX │ 0 │ --- │ YYYY │
└────┴────┴──────┴───┴─────┴──────┘
```

Format (Imm):
```
│15  │13  │11   8│ 7 │6       0│
┌────┬────┬──────┬───┬─────────┐
│ 10 │ MM │ XXXX │ 1 │ DDDDDDD │
└────┴────┴──────┴───┴─────────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `D`      | Immediate data   |
| `M`      | Mode flags       |
| `X`      | Destination `Rx` |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |

Mode (M):
| Flag  | Meaning |
| ----- | ------- |
| `00`  | `CMP`   |
| `01`  | `CMN`   |
| `10`  | `TST`   |
| `11`  | `TEQ`   |
