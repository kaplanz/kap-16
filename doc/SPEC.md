# Specification

[TOC]

## KAP-16

The `KAP-16` is a 16-bit, little endian, [von Neumann architecture](https://en.wikipedia.org/wiki/Von_Neumann_architecture) microprocessor.

### Registers

| Register | Alias | Use                     |
| -------- | ----- | ----------------------- |
| R0       | A0    | **A**rgument 0          |
| R1       | A1    | **A**rgument 1          |
| R2       | A2    | **A**rgument 2          |
| R3       | A3    | **A**rgument 3          |
| R4...R12 | —     | General Purpose         |
| R13      | SP    | **S**tack **P**ointer   |
| R14      | LR    | **L**ink **R**egister   |
| R15      | PC    | **P**rogram **C**ounter |
| —        | SR    | **S**tatus **R**egister |

### Instruction Set

Due to the constraints of 16-bit instruction registers, a key innovation of the `KAP-16` is its instruction opcode format;
Instead of using a traditional fixed-width opcode, [Huffman codings](https://en.wikipedia.org/wiki/Huffman_coding) are instead used to design a variable-width opcode format.

| Instruction                      | Opcode |
| -------------------------------- | -----: |
| [Move](./instr/MOV.md)           |   1010 |
| [Branch](./instr/BRA.md)         |   1111 |
| [Load](./instr/LDR.md)           |   1011 |
| [Store](./instr/STR.md)          |   1101 |
| [Compare](./instr/CMP.md)        |     00 |
| [Shift](./instr/SHF.md)          |   1110 |
| [Logical And](./instr/AND.md)    |   0110 |
| [Logical Or](./instr/ORR.md)     |   0100 |
| [Logical Xor](./instr/XOR.md)    |   0101 |
| [Addition](./instr/ADD.md)       |   1100 |
| [Subtraction](./instr/SUB.md)    |    100 |
| [Multiplication](./instr/MUL.md) |   0111 |
