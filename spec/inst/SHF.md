## Shift

Uses:
`LS{R,L}`, `AS{R,L}`, `RO{R,L}`,

Mnemonics:
- **L**ogical **S**hift **R**ight/**L**eft
- **A**rithmetic **S**hift **R**ight/**L**eft
- **RO**tate **R**ight/**L**eft

Description:
> Perform a logical, arithmetic, or rotate shift.

Condition Codes:
| Flag     | Modified |
| -------- | -------- |
| Carry    | &check;  |
| Negative | &check;  |
| Overflow | &check;  |
| Zero     | &check;  |

Notes:
- Logical shifts always fill with zeros.
- Arithmetic shifts perform sign extension/preservation.
- Rotations will spill over shifted bits.

Examples:
```assembly
LSL Rx, Ry    ; set Rx <- Rx L<< Ry
ASR Rx, 0d10  ; set Rx <- Rx A>> 10
ROR Rx, Ry    ; set Rx <- Rx R>> Ry
```

Format (Op2):
```
│15  12│11   8│ 7 │ 6 │5  4│3    0│
┌──────┬──────┬───┬───┬────┬──────┐
│ 1111 │ XXXX │ 0 │ L │ MM │ YYYY │
└──────┴──────┴───┴───┴────┴──────┘
```

Format (Imm):
```
│15  12│11   8│ 7 │ 6 │5  4│3    0│
┌──────┬──────┬───┬───┬────┬──────┐
│ 1111 │ XXXX │ 1 │ L │ MM │ DDDD │
└──────┴──────┴───┴───┴────┴──────┘
```

Legend:
| Format   | Use              |
| -------- | ---------------- |
| `0`, `1` | Literal bit      |
| `D`      | Immediate data   |
| `L`      | Left flag        |
| `M`      | Mode flags       |
| `X`      | Destination `Rx` |
| `Y`      | Source `Ry`      |
| `-`      | Unused           |

Left (L):
| Flag | Meaning |
| ---- | ------- |
| `0`  | Right   |
| `1`  | Left    |

Mode (M):
| Flag | Meaning |
| ---- | ------- |
| `00` | LSR     |
| `01` | ASR     |
| `10` | ROR     |
| `11` | &mdash; |
