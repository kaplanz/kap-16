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

## LANv1

As with any microprocessor, the `KAP-16` comes with its own assembly language: LAN, short for **L**ight **A**ssembly **N**otation.
LAN is loosely based on ARM, but the notation does differ in several places, notably in addressing modes for the `LDR` and `STR` instructions.

### General Format

A typical LAN instruction has 0-2 operands, with the second operand usually allowing for 7-bit immediate data instead of a register.
For detailed information on an instruction, refer to its manual page in [`doc/instr`](./doc/instr).

General guidelines are as follows:
- All registers may be referred to by either their index (`Rx`), or their alias (e.g. `PC`).
- Operands are to be separated by a single comma (`,`).
- Extra whitespace is ignored by the assembler, so feel free to align instructions as you see fit.
- Literal integer constants **must** specify their radix using one of the following prefixes:
  - Binary: `0b`
  - Octal: `0o`
  - Decimal: `0d`
  - Hexadecimal: `0x`
- Whenever an instruction interacts with memory (including immediate), use C-style pointer notation:
 - A [load](./instr/LDR.md) must dereference an address with `*`.
 - A [store](./instr/STR.md) must resolve an address with `&`.

#### Example

Here is a sample program in LAN, finding the 7th number in the Fibonacci sequence.

```asm
_main:
    MOV R0, 0d1  ; use R0, R1 to store the most recent...
    MOV R1, 0d1  ; ... two numbers in the sequence
    MOV R2, 0d0  ; use R2 as a counter
LOOP:
    ADD R0, R1   ; compute the R2th number
    ADD R1, R0   ; computer the (R2 + 1)st number
    ADD R2, 0b2
    CMP R2, 0b7  ; check if we've reached 7...
    BLT &LOOP    ; ... loop until we're done
                 ; R0, R1 now store the 6th, 7th numbers
```

### Immediate Data

Most instructions allow the final operand to, instead of a register, be supplied as immediate data.
The following table outlines the immediate data various instructions accept.

| Instruction | Accepts Immediate | Data Width |
| ----------- | ----------------- | ---------- |
| ADD         | &check;           | 7-bit      |
| AND         | &check;           | 7-bit      |
| BRA         | &check;           | 7-bit      |
| CMN         | &check;           | 7-bit      |
| CMP         | &check;           | 7-bit      |
| LDR         | &check;           | 7-bit      |
| MOV         | &check;           | 7-bit      |
| MUL         | &check;           | 7-bit      |
| NEG         | &cross;           | —          |
| NOP         | &cross;           | —          |
| NOT         | &cross;           | —          |
| ORR         | &check;           | 7-bit      |
| POP         | &cross;           | —          |
| PUSH        | &cross;           | —          |
| RSB         | &check;           | 7-bit      |
| SHF         | &check;           | 4-bit      |
| STR         | &check;           | 7-bit      |
| SUB         | &check;           | 7-bit      |
| TEQ         | &check;           | 7-bit      |
| TST         | &check;           | 7-bit      |
| XOR         | &check;           | 7-bit      |
