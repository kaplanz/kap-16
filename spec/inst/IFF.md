## Conditional If

Uses:
`IF{[AL],NV,EQ,NE,LT,LE,GE,GT,CC,CS,VC,VS,PL,MI}`

Mnemonics:
- **AL**ways
- **N**e**V**er
- **EQ**ual
- **N**ot **E**qual
- **L**ess **T**han
- **L**ess than or **E**qual
- **G**reater than or **E**qual
- **G**reater **T**han
- **C**arry **C**lear
- **C**arry **S**et
- o**V**erflow **C**lear
- o**V**erflow **S**et
- **PL**us (positive)
- **MI**nus (negative)

Description:
> Perform the next instruction conditionally.
> Functionally equivalent to incrementing the PC if the condition is false.

Condition Codes:
| Flag     | Modified |
| -------- | -------- |
| Carry    | &cross;  |
| Negative | &cross;  |
| Overflow | &cross;  |
| Zero     | &cross;  |


Examples:
```assembly
IFEQ        ; perform next instruction if zero
IFGT        ;    "     "        "      if greater than
IFCC        ;    "     "        "      if carry clear
```

Format:
```
│15     10│9     4│3    0│
┌─────────┬───────┬──────┐
│ 0000111 │ ----- │ CCCC │
└─────────┴───────┴──────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `C`      | Condition code   |
| `-`      | Unused           |

Condition Code (C):
| Flag   | Meaning |
| -----  | ------- |
| `0000` | AL      |
| `0001` | NV      |
| `0010` | EQ      |
| `0011` | NE      |
| `0100` | LT      |
| `0101` | LE      |
| `0110` | GE      |
| `0111` | GT      |
| `1000` | CC      |
| `1001` | CS      |
| `1010` | VC      |
| `1011` | VS      |
| `1100` | PL      |
| `1101` | MI      |
| `1110` | &mdash; |
| `1111` | &mdash; |
