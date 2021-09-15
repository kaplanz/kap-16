## Branch

Uses:
`GOTO`, `CALL`

Mnemonics:
- **GO** **TO**
- **CALL**

Description:
> Branch (goto) another point in the code and continue execution.
> Functionally equivalent to moving into the PC.

Condition Codes:
| Flag     | Modified |
| -------- | -------- |
| Carry    | &cross;  |
| Negative | &cross;  |
| Overflow | &cross;  |
| Zero     | &cross;  |

Notes:
- May use a symbol optionally instead of an address offset

Examples:
```assembly
CALL &Ry    ; branch to address in Ry after performing a link
            ; a.k.a: LR <- PC, PC <- Ry
GOTO 0x40   ; branch to address (PC + 0x40)
            ; a.k.a: PC <- PC + 0x40
CALL _foo   ; branch and link to the symbol `_foo`
            ; (performs a procedure call)
```

Format (Op2):
```
│15   11│10 │9  8│ 7 │6   4│3    0│
┌───────┬───┬────┬───┬─────┬──────┐
│ 00000 │ L │ -- │ 0 │ --- │ YYYY │
└───────┴───┴────┴───┴─────┴──────┘
```

Format (Imm):
```
│15   11│10 │9  8│ 7 │6       0│
┌───────┬───┬────┬───┬─────────┐
│ 00000 │ L │ -- │ 1 │ DDDDDDD │
└───────┴───┴────┴───┴─────────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `D`      | Immediate data   |
| `L`      | Link flag        |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |

Link (L):
> When set, "link" on the branch.
> (Copies `PC <- LR` before performing the branch.)
