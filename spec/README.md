# spec

- For the full microarchitecture specification, read [`ARCH.md`](./ARCH.md).
- For the full instruction set specification, read [`ISA.md`](./ISA.md).
- For detailed instruction information, refer to the manual pages in [`inst/`](./inst).

## Instructions

Below is a directory is all currently supported instructions.

| Instruction              | Description    |
| ------------------------ | -------------- |
| [`ADD`](./inst/ADD.md)   | Add            |
| [`AND`](./inst/AND.md)   | Logical AND    |
| [`BRA`](./inst/BRA.md)   | Branch         |
| [`CMN`](./inst/CMN.md)   | Compare (ADD)  |
| [`CMP`](./inst/CMP.md)   | Compare (SUB)  |
| [`LDR`](./inst/LDR.md)   | Load           |
| [`MOV`](./inst/MOV.md)   | Move           |
| [`MUL`](./inst/MUL.md)   | Multiply       |
| [`NEG`](./inst/NEG.md)   | Logical Negate |
| [`NOP`](./inst/NOP.md)   | No Operation   |
| [`NOT`](./inst/NOT.md)   | Arithmetic NOT |
| [`ORR`](./inst/ORR.md)   | Logical OR     |
| [`POP`](./inst/POP.md)   | Pop Register   |
| [`PUSH`](./inst/PUSH.md) | Push Register  |
| [`RSB`](./inst/RSB.md)   | Reverse SUB    |
| [`SHF`](./inst/SHF.md)   | Shift          |
| [`STR`](./inst/STR.md)   | Store          |
| [`SUB`](./inst/SUB.md)   | Subtract       |
| [`TEQ`](./inst/TEQ.md)   | Compare (XOR)  |
| [`TST`](./inst/TST.md)   | Compare (AND)  |
| [`XOR`](./inst/XOR.md)   | Logical XOR    |

## Registers

| Register | Alias   | Use                     | Responsibility |
| -------- | ------- | ----------------------- | -------------- |
| R0       | A0      | **A**rgument 0          | &mdash;        |
| R1       | A1      | **A**rgument 1          | &mdash;        |
| R2       | A2      | **A**rgument 2          | &mdash;        |
| R3       | A3      | **A**rgument 3          | &mdash;        |
| R4...R12 | &mdash; | General Purpose         | Callee         |
| R13      | SP      | **S**tack **P**ointer   | Callee         |
| R14      | LR      | **L**ink **R**egister   | Caller         |
| R15      | PC      | **P**rogram **C**ounter | &mdash;        |
| &mdash;  | SR      | **S**tatus **R**egister | &mdash;        |

### Argument Registers

While the argument registers do not hold any specific meaning to the processor itself, it is the convention of LANv1 that arguments to procedures be placed in the argument registers, A0 to A3, also knows as R0 to R3.
Callers of a procedure cannot expect data to remain within these registers after the procedure completes.
Additionally, the argument registers may optionally be used to return data from a procedure.
This should be outlined specifically in by the procedure's author.

### Unnamed Registers

The unnamed registers, R4 to R12, do not have a designated use.
As general purpose registers, their usage within a program is up to the programmer.
However, it is important to note that if they are modified in a procedure, the previous values must be restored by the callee.
To easily save and restored a register's value, use the [`PUSH`](./inst/PUSH.md) and [`POP`](./inst/POP.md) instructions respectively.

### Stack Pointer

The [stack pointer][stack-pointer] (SP), also known as R13, tracks the bottom of the [hardware stack][hardware-stack].
When calling a procedure, the callee is responsible for restoring the stack pointer to its previous value.

### Link Register

The [link register][link-register] (LR), also known as R14, holds the return address to branch to after a procedure.
Before calling a procedure, the caller is responsible for setting the link register to the desired return address (usually the program counter).
This allows the procedure to know where to return to, which can be accomplished with an instruction such as `B LR`.
This can be handled by calling the [`BL`](./inst/BRA.md) instruction to perform a branch and link.

### Program Counter

The [program counter][program-counter] (PC), also known as R15, performs one of the most important functions of any register:
it keeps track of the current instruction within the running program, and increments ("counts") to the next instruction every cycle.
While it can be written to as with any other register, it is typically only modified by the programmer through the [`BRA`](./inst/BRA.md) family of instructions.

[stack-pointer]: https://en.wikipedia.org/wiki/Call_stack#STACK-POINTER
[hardware-stack]: https://en.wikipedia.org/wiki/Stack_(abstract_data_type)#Hardware_stack
[link-register]: https://en.wikipedia.org/wiki/Link_register
[program-counter]: https://en.wikipedia.org/wiki/Program_counter
