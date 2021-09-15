# Specification

## LANv1

LANv1, short for **L**ite **A**ssembly **N**otation (**v**ersion **1**), is KAP-16's instruction set.
As a RISC "load–store" architecture, the [`LDR`](./inst/LDR.md) and [`STR`](./inst/STR.md) instructions (along with their pseudo-instructions) are the sole interface to memory; all other instructions perform operations between registers.
The ISA is inspired by ARM, but the notation does differ in several places.

### Instructions

Due to the constraints of 16-bit instruction registers, KAP-16 uses an interesting instruction opcode format:
Instead of a traditional fixed-width opcode, [Huffman codings][huffman-codings] were used to design a variable-width opcode format.
Read about it [here](../doc/huffman/README.md) for further details.

| Core Instruction       | Opcode    |
| ---------------------- | :-------- |
| [`ADD`](./inst/ADD.md) | `1100`    |
| [`AND`](./inst/AND.md) | `1110`    |
| [`BRA`](./inst/BRA.md) | `00000`   |
| [`CMP`](./inst/CMP.md) | `10`      |
| [`HLT`](./inst/HLT.md) | `0000110` |
| [`IFF`](./inst/IFF.md) | `0000111` |
| [`LDR`](./inst/LDR.md) | `0011`    |
| [`MOV`](./inst/MOV.md) | `0111`    |
| [`MUL`](./inst/MUL.md) | `0110`    |
| [`ORR`](./inst/ORR.md) | `1101`    |
| [`SHF`](./inst/SHF.md) | `1111`    |
| [`STR`](./inst/STR.md) | `0010`    |
| [`SUB`](./inst/SUB.md) | `010`     |
| [`SYS`](./inst/SYS.md) | `000010`  |
| [`XOR`](./inst/XOR.md) | `0001`    |

### Condition Codes

Several instructions modify the status register's condition code flags as a result of their operation.
The following table provides a quick reference to how instructions interact with condition codes.

| Instruction              | Modifies |
| ------------------------ | -------- |
| [`ADD`](./inst/ADD.md)   | &check;  |
| [`AND`](./inst/AND.md)   | &check;  |
| [`ASL`](./inst/ASL.md)   | &check;  |
| [`ASR`](./inst/ASR.md)   | &check;  |
| [`CALL`](./inst/CALL.md) | &cross;  |
| [`CMN`](./inst/CMN.md)   | &check;  |
| [`CMP`](./inst/CMP.md)   | &check;  |
| [`GOTO`](./inst/GOTO.md) | &cross;  |
| [`HLT`](./inst/HLT.md)   | &cross;  |
| [`IFF`](./inst/IFF.md)   | &cross;  |
| [`LDR`](./inst/LDR.md)   | &cross;  |
| [`LSL`](./inst/LSL.md)   | &check;  |
| [`LSR`](./inst/LSR.md)   | &check;  |
| [`MOV`](./inst/MOV.md)   | &cross;  |
| [`MUL`](./inst/MUL.md)   | &check;  |
| [`NEG`](./inst/NEG.md)   | &cross;  |
| [`NOP`](./inst/NOP.md)   | &cross;  |
| [`NOT`](./inst/NOT.md)   | &cross;  |
| [`ORR`](./inst/ORR.md)   | &check;  |
| [`POP`](./inst/POP.md)   | &cross;  |
| [`PUSH`](./inst/PUSH.md) | &cross;  |
| [`RSB`](./inst/RSB.md)   | &check;  |
| [`ROL`](./inst/ASL.md)   | &check;  |
| [`ROR`](./inst/ASR.md)   | &check;  |
| [`STR`](./inst/STR.md)   | &cross;  |
| [`SUB`](./inst/SUB.md)   | &check;  |
| [`SYS`](./inst/SYS.md)   | &cross;  |
| [`TEQ`](./inst/TEQ.md)   | &check;  |
| [`TST`](./inst/TST.md)   | &check;  |
| [`XOR`](./inst/XOR.md)   | &check;  |

### Immediates

Most instructions allow the final operand to, instead of a register, be supplied as immediate data.
Additionally, immediate data may be either sign extended if specified by the instruction.
The following table outlines the immediate data various instructions accept.

| Instruction              | Immediate | Width   | Signed/Extended |
| ------------------------ | --------- | ------- | --------------- |
| [`ADD`](./inst/ADD.md)   | &check;   | 7-bit   | &cross;         |
| [`AND`](./inst/AND.md)   | &check;   | 7-bit   | &check;         |
| [`ASL`](./inst/ASL.md)   | &check;   | 7-bit   | &cross;         |
| [`ASR`](./inst/ASR.md)   | &check;   | 7-bit   | &cross;         |
| [`CALL`](./inst/CALL.md) | &check;   | 7-bit   |                 |
| [`CMN`](./inst/CMN.md)   | &check;   | 7-bit   | &check;         |
| [`CMP`](./inst/CMP.md)   | &check;   | 7-bit   | &check;         |
| [`GOTO`](./inst/GOTO.md) | &check;   | 7-bit   |                 |
| [`HLT`](./inst/HLT.md)   | &cross;   | &mdash; | &mdash;         |
| [`IFF`](./inst/IFF.md)   | &cross;   | &mdash; | &mdash;         |
| [`LDR`](./inst/LDR.md)   | &check;   | 7-bit   | &check;         |
| [`LSL`](./inst/LSL.md)   | &check;   | 7-bit   | &cross;         |
| [`LSR`](./inst/LSR.md)   | &check;   | 7-bit   | &cross;         |
| [`MOV`](./inst/MOV.md)   | &check;   | 7-bit   | &check;         |
| [`MUL`](./inst/MUL.md)   | &check;   | 7-bit   | &check;         |
| [`NEG`](./inst/NEG.md)   | &cross;   | &mdash; | &mdash;         |
| [`NOP`](./inst/NOP.md)   | &cross;   | &mdash; | &mdash;         |
| [`NOT`](./inst/NOT.md)   | &cross;   | &mdash; | &mdash;         |
| [`ORR`](./inst/ORR.md)   | &check;   | 7-bit   | &check;         |
| [`POP`](./inst/POP.md)   | &cross;   | &mdash; | &mdash;         |
| [`PUSH`](./inst/PUSH.md) | &cross;   | &mdash; | &mdash;         |
| [`RSB`](./inst/RSB.md)   | &check;   | 7-bit   | &cross;         |
| [`ROL`](./inst/ASL.md)   | &check;   | 7-bit   | &cross;         |
| [`ROR`](./inst/ASR.md)   | &check;   | 7-bit   | &cross;         |
| [`STR`](./inst/STR.md)   | &check;   | 7-bit   | &check;         |
| [`SUB`](./inst/SUB.md)   | &check;   | 7-bit   | &cross;         |
| [`SYS`](./inst/SYS.md)   | &cross;   | &mdash; | &mdash;         |
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

[huffman-codings]: https://en.wikipedia.org/wiki/Huffman_coding
