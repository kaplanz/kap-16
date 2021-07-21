# Specification

## LANv1

LANv1, short for **L**ite **A**ssembly **N**otation (**v**ersion **1**), is KAP-16's instruction set.
As a RISC "load–store" architecture, the [`LDR`](./instr/LDR.md) and [`STR`](./instr/STR.md) instructions (along with their pseudo-instructions) are the sole interface to memory; all other instructions perform operations between registers.
The ISA is inspired by ARM, but the notation does differ in several places.

### General

Each LANv1 instruction has 0-2 operands, with the final operand usually allowing for 7-bit immediate data instead of a register.
For detailed information on an instruction, refer to its manual page in [`instr/`](./instr).

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
   - A [load](./instr/LDR.md) must dereference an address with `*`.
   - A [store](./instr/STR.md) must resolve an address with `&`.

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

### Immediates

Most instructions allow the final operand to, instead of a register, be supplied as immediate data.
The following table outlines the immediate data various instructions accept.

| Instruction               | Immediate Mode | Width   |
| ------------------------- | -------------- | ------- |
| [`ADD`](./instr/ADD.md)   | &check;        | 7-bit   |
| [`AND`](./instr/AND.md)   | &check;        | 7-bit   |
| [`BRA`](./instr/BRA.md)   | &check;        | 7-bit   |
| [`CMN`](./instr/CMN.md)   | &check;        | 7-bit   |
| [`CMP`](./instr/CMP.md)   | &check;        | 7-bit   |
| [`LDR`](./instr/LDR.md)   | &check;        | 7-bit   |
| [`MOV`](./instr/MOV.md)   | &check;        | 7-bit   |
| [`MUL`](./instr/MUL.md)   | &check;        | 7-bit   |
| [`NEG`](./instr/NEG.md)   | &cross;        | &mdash; |
| [`NOP`](./instr/NOP.md)   | &cross;        | &mdash; |
| [`NOT`](./instr/NOT.md)   | &cross;        | &mdash; |
| [`ORR`](./instr/ORR.md)   | &check;        | 7-bit   |
| [`POP`](./instr/POP.md)   | &cross;        | &mdash; |
| [`PUSH`](./instr/PUSH.md) | &cross;        | &mdash; |
| [`RSB`](./instr/RSB.md)   | &check;        | 7-bit   |
| [`SHF`](./instr/SHF.md)   | &check;        | 4-bit   |
| [`STR`](./instr/STR.md)   | &check;        | 7-bit   |
| [`SUB`](./instr/SUB.md)   | &check;        | 7-bit   |
| [`TEQ`](./instr/TEQ.md)   | &check;        | 7-bit   |
| [`TST`](./instr/TST.md)   | &check;        | 7-bit   |
| [`XOR`](./instr/XOR.md)   | &check;        | 7-bit   |
