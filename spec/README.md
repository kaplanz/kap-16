# spec

- For the full microarchitecture specification, read [`ARCH.md`](./ARCH.md).
- For the full instruction set specification, read [`ISA.md`](./ISA.md).
- For detailed instruction information, refer to the manual pages in [`instr/`](./instr).

## Instructions

Below is a directory is all currently supported instructions.

| Instruction               | Description    |
| ------------------------- | -------------- |
| [`ADD`](./instr/ADD.md)   | Add            |
| [`AND`](./instr/AND.md)   | Logical AND    |
| [`BRA`](./instr/BRA.md)   | Branch         |
| [`CMN`](./instr/CMN.md)   | Compare (ADD)  |
| [`CMP`](./instr/CMP.md)   | Compare (SUB)  |
| [`LDR`](./instr/LDR.md)   | Load           |
| [`MOV`](./instr/MOV.md)   | Move           |
| [`MUL`](./instr/MUL.md)   | Multiply       |
| [`NEG`](./instr/NEG.md)   | Logical Negate |
| [`NOP`](./instr/NOP.md)   | No Operation   |
| [`NOT`](./instr/NOT.md)   | Arithmetic NOT |
| [`ORR`](./instr/ORR.md)   | Logical OR     |
| [`POP`](./instr/POP.md)   | Pop Register   |
| [`PUSH`](./instr/PUSH.md) | Push Register  |
| [`RSB`](./instr/RSB.md)   | Reverse SUB    |
| [`SHF`](./instr/SHF.md)   | Shift          |
| [`STR`](./instr/STR.md)   | Store          |
| [`SUB`](./instr/SUB.md)   | Subtract       |
| [`TEQ`](./instr/TEQ.md)   | Compare (XOR)  |
| [`TST`](./instr/TST.md)   | Compare (AND)  |
| [`XOR`](./instr/XOR.md)   | Logical XOR    |

## Registers

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
