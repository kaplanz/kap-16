# Specification

## KAP-16

### Instructions

Due to the constraints of 16-bit instruction registers, KAP-16 uses an interesting instruction opcode format:
Instead of a traditional fixed-width opcode, [Huffman codings][huffman-codings] were used to design a variable-width opcode format.
Read about it [here](./doc/huffman/README.md) for further details.

| Core Instruction              | Opcode |
| ----------------------------- | -----: |
| [Move](./instr/MOV.md)        |   1010 |
| [Branch](./instr/BRA.md)      |   1111 |
| [Load](./instr/LDR.md)        |   1011 |
| [Store](./instr/STR.md)       |   1101 |
| [Compare](./instr/CMP.md)     |     00 |
| [Shift](./instr/SHF.md)       |   1110 |
| [Logical AND](./instr/AND.md) |   0110 |
| [Logical OR](./instr/ORR.md)  |   0100 |
| [Logical XOR](./instr/XOR.md) |   0101 |
| [Add](./instr/ADD.md)         |   1100 |
| [Subtract](./instr/SUB.md)    |    100 |
| [Multiply](./instr/MUL.md)    |   0111 |

### Registers

| Register | Alias   | Use                     |
| -------- | ------- | ----------------------- |
| R0       | A0      | **A**rgument 0          |
| R1       | A1      | **A**rgument 1          |
| R2       | A2      | **A**rgument 2          |
| R3       | A3      | **A**rgument 3          |
| R4...R12 | &mdash; | General Purpose         |
| R13      | SP      | **S**tack **P**ointer   |
| R14      | LR      | **L**ink **R**egister   |
| R15      | PC      | **P**rogram **C**ounter |
| &mdash;  | SR      | **S**tatus **R**egister |

[huffman-codings]: https://en.wikipedia.org/wiki/Huffman_coding
