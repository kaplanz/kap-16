## Move

Uses:
`MOV`, `NEG`, `NOT`, `NOP`

Mnemonics:
- **MOV**e
- Arithmetic **NEG**ate (twos complement)
- Logical **NOT** (ones complement)
- **N**o **Op**eration

Condition Codes:
| Flag     | Modified |
| -------- | -------- |
| Carry    | &cross;  |
| Negative | &cross;  |
| Overflow | &cross;  |
| Zero     | &cross;  |

Description:
> Copy ("move") the data from one register to another.
> Provides mode flags for optional ones or twos complement to occur alongside move.

Notes:
- In immdiate data mode, this instruction does not support `NEG`, `NOT`.
  (The assembler is responsible for resolving these operations.)
- Only supports 7-bit immediate data, for more see [`LDR`](./LDR.md).

Examples:
```assembly
MOV Rx, Ry    ; set Rx <- Ry
MOV Rx, 0d16  ; set Rx <- 16
NEG Rx        ; set Rx <- ~Rx
NOT Rx, Ry    ; set Rx <- -Ry
NOP           ; does nothing
```

Format (Op2):
```
│15  12│11   8│ 7 │ 6 │5  4│3    0│
┌──────┬──────┬───┬───┬────┬──────┐
│ 0111 │ XXXX │ 0 │ - │ MM │ YYYY │
└──────┴──────┴───┴───┴────┴──────┘
```

Format (Imm):
```
│15  12│11   8│ 7 │6       0│
┌──────┬──────┬───┬─────────┐
│ 0111 │ XXXX │ 1 │ DDDDDDD │
└──────┴──────┴───┴─────────┘
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
| Flag | Meaning |
| ---- | ------- |
| `00` | MOV     |
| `01` | NEG     |
| `10` | NOT     |
| `11` | &mdash; |

Pseudo-instructions:
- `NOP := MOV R0, R0 ; no operation`
