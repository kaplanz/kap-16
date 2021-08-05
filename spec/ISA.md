# Specification

## LANv1

LANv1, short for **L**ite **A**ssembly **N**otation (**v**ersion **1**), is KAP-16's instruction set.
As a RISC "load–store" architecture, the [`LDR`](./inst/LDR.md) and [`STR`](./inst/STR.md) instructions (along with their pseudo-instructions) are the sole interface to memory; all other instructions perform operations between registers.
The ISA is inspired by ARM, but the notation does differ in several places.

### Instructions

Due to the constraints of 16-bit instruction registers, KAP-16 uses an interesting instruction opcode format:
Instead of a traditional fixed-width opcode, [Huffman codings][huffman-codings] were used to design a variable-width opcode format.
Read about it [here](../doc/huffman/README.md) for further details.

| Core Instruction             | Opcode |
| ---------------------------- | -----: |
| [Move](./inst/MOV.md)        |   1010 |
| [Branch](./inst/BRA.md)      |   1111 |
| [Load](./inst/LDR.md)        |   1011 |
| [Store](./inst/STR.md)       |   1101 |
| [Compare](./inst/CMP.md)     |     00 |
| [Shift](./inst/SHF.md)       |   1110 |
| [Logical AND](./inst/AND.md) |   0110 |
| [Logical OR](./inst/ORR.md)  |   0100 |
| [Logical XOR](./inst/XOR.md) |   0101 |
| [Add](./inst/ADD.md)         |   1100 |
| [Subtract](./inst/SUB.md)    |    100 |
| [Multiply](./inst/MUL.md)    |   0111 |

### Condition Codes

After

| Instruction              | Modifies |
| ------------------------ | -------- |
| [`ADD`](./inst/ADD.md)   | &check;  |
| [`AND`](./inst/AND.md)   | &check;  |
| [`BRA`](./inst/BRA.md)   |          |
| [`CMN`](./inst/CMN.md)   | &check;  |
| [`CMP`](./inst/CMP.md)   | &check;  |
| [`LDR`](./inst/LDR.md)   |          |
| [`MOV`](./inst/MOV.md)   |          |
| [`MUL`](./inst/MUL.md)   | &check;  |
| [`NEG`](./inst/NEG.md)   |          |
| [`NOP`](./inst/NOP.md)   | &cross;  |
| [`NOT`](./inst/NOT.md)   |          |
| [`ORR`](./inst/ORR.md)   | &check;  |
| [`POP`](./inst/POP.md)   |          |
| [`PUSH`](./inst/PUSH.md) |          |
| [`RSB`](./inst/RSB.md)   | &check;  |
| [`SHF`](./inst/SHF.md)   | &check;  |
| [`STR`](./inst/STR.md)   |          |
| [`SUB`](./inst/SUB.md)   | &check;  |
| [`TEQ`](./inst/TEQ.md)   | &check;  |
| [`TST`](./inst/TST.md)   | &check;  |
| [`XOR`](./inst/XOR.md)   | &check;  |

### Immediates

Most instructions allow the final operand to, instead of a register, be supplied as immediate data.
The following table outlines the immediate data various instructions accept.

| Instruction              | Immediate | Width   | Signed/Extended |
| ------------------------ | --------- | ------- | --------------- |
| [`ADD`](./inst/ADD.md)   | &check;   | 7-bit   | &cross;         |
| [`AND`](./inst/AND.md)   | &check;   | 7-bit   | &check;         |
| [`BRA`](./inst/BRA.md)   | &check;   | 7-bit   | &check;         |
| [`CMN`](./inst/CMN.md)   | &check;   | 7-bit   | &check;         |
| [`CMP`](./inst/CMP.md)   | &check;   | 7-bit   | &check;         |
| [`LDR`](./inst/LDR.md)   | &check;   | 7-bit   | &check;         |
| [`MOV`](./inst/MOV.md)   | &check;   | 7-bit   | &check;         |
| [`MUL`](./inst/MUL.md)   | &check;   | 7-bit   | &check;         |
| [`NEG`](./inst/NEG.md)   | &cross;   | &mdash; | &mdash;         |
| [`NOP`](./inst/NOP.md)   | &cross;   | &mdash; | &mdash;         |
| [`NOT`](./inst/NOT.md)   | &cross;   | &mdash; | &mdash;         |
| [`ORR`](./inst/ORR.md)   | &check;   | 7-bit   | &check;         |
| [`POP`](./inst/POP.md)   | &cross;   | &mdash; | &mdash;         |
| [`PUSH`](./inst/PUSH.md) | &cross;   | &mdash; | &mdash;         |
| [`RSB`](./inst/RSB.md)   | &check;   | 7-bit   | &cross;         |
| [`SHF`](./inst/SHF.md)   | &check;   | 4-bit   | &cross;         |
| [`STR`](./inst/STR.md)   | &check;   | 7-bit   | &check;         |
| [`SUB`](./inst/SUB.md)   | &check;   | 7-bit   | &cross;         |
| [`TEQ`](./inst/TEQ.md)   | &check;   | 7-bit   | &check;         |
| [`TST`](./inst/TST.md)   | &check;   | 7-bit   | &check;         |
| [`XOR`](./inst/XOR.md)   | &check;   | 7-bit   | &check;         |

### Notation

Each LANv1 instruction has 0-2 operands, with the final operand usually allowing for 7-bit immediate data instead of a register.
For detailed information on an instruction, refer to its manual page in [`inst/`](./inst).

General guidelines are as follows:
- All registers may be referred to by either their index (`Rx`), or their alias (e.g. `PC`).
- Operands are to be separated by a single comma (`,`).
- Extra whitespace is ignored by the assembler, so feel free to align instructions as you see fit.
- Literal integer constants **must** specify their radix using one of the following prefixes:
  - Binary: `0b`
  - Octal: `0o`
  - Decimal: `0d`
  - Hexadecimal: `0x`
- Whenever interfacing with memory, use C-style pointer notation:
   - A [load](./inst/LDR.md) must dereference an address with `*`.
   - A [store](./inst/STR.md) must resolve an address with `&`.

#### Example

Here is a sample program in LANv1 calculating the 7th number in the Fibonacci sequence.

```assembly
_main:
    MOV R0, 0d1  ; use R0, R1 to store the current...
    MOV R1, 0d1  ; ... and next numbers in the sequence
    MOV R2, 0d0  ; use R2 as a counter
LOOP:
    ADD R0, R1   ; compute the R2th number
    ADD R1, R0   ; computer the (R2 + 1)st number
    ADD R2, 0b2
    CMP R2, 0b7  ; check if we've reached 7...
    BLT LOOP     ; ... loop until we're done
                 ; R0, R1 now store the 6th, 7th numbers
```

[huffman-codings]: https://en.wikipedia.org/wiki/Huffman_coding
