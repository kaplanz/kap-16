## Branch

Uses:
`B[L]{[RA],EQ,NQ,LT,LE,GE,GT}`

Mnemonics:
- **BRA**nch (unconditional)
- **EQ**ual
- **N**ot **E**qual
- **L**ess **T**han
- **L**ess than or **E**qual
- **G**reater than or **E**qual
- **G**reater **T**han

Description:
> Branch to another instruction if specific conditions have been met.
> Functionally equivalent to moving into the PC.

Examples:
```assembly
BL   &Ry    ; branch to address in Ry after performing a link
BLRA &Ry    ; synonym of above (RA := unconditional branch)
            ; a.k.a: LR <- PC, PC <- Ry
BEQ  0x40   ; branch to address (PC + 0x40), if previous result is zero
            ; a.k.a: if EQ then PC <- PC + 0x40
```

Format (Op2):
```
│15  12│11  9│ 8 │ 7 │6   4│3    0│
┌──────┬─────┬───┬───┬─────┬──────┐
│ 1111 │ CCC │ L │ 0 │ --- │ YYYY │
└──────┴─────┴───┴───┴─────┴──────┘
```

Format (Imm):
```
│15  12│11  9│ 8 │ 7 │6       0│
┌──────┬─────┬───┬───┬─────────┐
│ 1111 │ CCC │ L │ 1 │ DDDDDDD │
└──────┴─────┴───┴───┴─────────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `C`      | Condition code   |
| `D`      | Immediate data   |
| `I`      | Immediate flag   |
| `L`      | Link flag        |
| `X`      | Destination `Rx` |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |

Link (L):
> When set, "link" on the branch.
> (Copies `PC <- LR` before performing the branch.)

Condition Code (C):
| Flag  | Meaning |
| ----- | ------- |
| `000` | RA      |
| `001` | EQ      |
| `010` | NE      |
| `011` | LT      |
| `100` | LE      |
| `101` | GE      |
| `110` | GT      |
| `111` | &mdash; |
