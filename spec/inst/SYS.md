## System

Uses:
`SYS`

Mnemonics:
- **SYS**tem

Description:
> Reserved general system instruction.

Condition Codes:
| Flag     | Modified |
| -------- | -------- |
| Carry    | &cross;  |
| Negative | &cross;  |
| Overflow | &cross;  |
| Zero     | &cross;  |

Examples:
```assembly
SYS  ; perform system functions
```

Format:
```
│15    10│9          0│
┌────────┬────────────┐
│ 000010 │ ---------- │
└────────┴────────────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `-`      | Unused           |
