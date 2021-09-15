## Halt

Uses:
`HLT`

Mnemonics:
- **H**a**LT**

Description:
> Cause the processor to halt;
> Ends execution.

Condition Codes:
| Flag     | Modified |
| -------- | -------- |
| Carry    | &cross;  |
| Negative | &cross;  |
| Overflow | &cross;  |
| Zero     | &cross;  |

Examples:
```assembly
HLT  ; halt processor
```

Format:
```
│15      9│8         0│
┌─────────┬───────────┐
│ 0000110 │ --------- │
└─────────┴───────────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `-`      | Unused           |
