# spec

- For the full microarchitecture specification, read [`ARCH.md`](./ARCH.md).
- For the full instruction set specification, read [`ISA.md`](./ISA.md).
- For detailed instruction information, refer to the manual pages in [`inst/`](./inst).

## Instructions

Below is a directory is all currently supported instructions.
They are divided into four categories:
- Alias: alternate name and interface for another instruction.
- Base: a common implementation for a group of instruction variants; cannot use directly.
- Core: an instruction whose operation is directly implemented; may have variants.
- Pseudo: an instruction implemented via another; a shorthand.
- Variant: a variation on another instruction.

Only core instructions have their own designated opcode.
Variants are implemented within the namespace of their corresponding core instruction.

| Instruction              | Description            | Type    |
| ------------------------ | ---------------------- | ------- |
| [`ADD`](./inst/ADD.md)   | Arithmetic Add         | Core    |
| [`AND`](./inst/AND.md)   | Logical AND            | Core    |
| [`ASL`](./inst/ASL.md)   | Arithmetic Shift Left  | Variant |
| [`ASR`](./inst/ASR.md)   | Arithmetic Shift Right | Variant |
| [`BRA`](./inst/BRA.md)   | Branch                 | Base    |
| [`CALL`](./inst/CALL.md) | Call (branch + link)   | Variant |
| [`CMN`](./inst/CMN.md)   | Compare by ADD         | Variant |
| [`CMP`](./inst/CMP.md)   | Compare by SUB         | Core    |
| [`GOTO`](./inst/GOTO.md) | Goto (branch)          | Alias   |
| [`HLT`](./inst/HLT.md)   | Halt                   | Core    |
| [`IFF`](./inst/IFF.md)   | Conditional If         | Core    |
| [`LDR`](./inst/LDR.md)   | Load                   | Core    |
| [`LSL`](./inst/LSL.md)   | Logical Shift Left     | Variant |
| [`LSR`](./inst/LSR.md)   | Logical Shift Right    | Variant |
| [`MOV`](./inst/MOV.md)   | Move                   | Core    |
| [`MUL`](./inst/MUL.md)   | Arithmetic Multiply    | Core    |
| [`NEG`](./inst/NEG.md)   | Arithmetic Negate      | Variant |
| [`NOP`](./inst/NOP.md)   | No Operation           | Pseudo  |
| [`NOT`](./inst/NOT.md)   | Logical NOT            | Variant |
| [`ORR`](./inst/ORR.md)   | Logical OR             | Core    |
| [`POP`](./inst/POP.md)   | Pop Register           | Variant |
| [`PUSH`](./inst/PUSH.md) | Push Register          | Variant |
| [`RSB`](./inst/RSB.md)   | Reverse SUB            | Variant |
| [`ROL`](./inst/ASL.md)   | Rotate Left            | Variant |
| [`ROR`](./inst/ASR.md)   | Rotate Right           | Variant |
| [`SHF`](./inst/SHF.md)   | Shift                  | Base    |
| [`STR`](./inst/STR.md)   | Store                  | Core    |
| [`SUB`](./inst/SUB.md)   | Arithmetic Subtract    | Core    |
| [`SYS`](./inst/SYS.md)   | System (reserved)      | Core    |
| [`TEQ`](./inst/TEQ.md)   | Compare by XOR         | Variant |
| [`TST`](./inst/TST.md)   | Compare by AND         | Variant |
| [`XOR`](./inst/XOR.md)   | Logical XOR            | Core    |

## Registers

| Register | Alias   | Use                     | Responsibility |
| -------- | ------- | ----------------------- | -------------- |
| R0       | A0      | **A**rgument 0          | &mdash;        |
| R1       | A1      | **A**rgument 1          | &mdash;        |
| R2       | A2      | **A**rgument 2          | &mdash;        |
| R3       | A3      | **A**rgument 3          | &mdash;        |
| R4...R12 | G0...G8 | **G**eneral Purpose     | Callee         |
| R13      | SP      | **S**tack **P**ointer   | Callee         |
| R14      | LR      | **L**ink **R**egister   | Caller         |
| R15      | PC      | **P**rogram **C**ounter | &mdash;        |
| &mdash;  | SR      | **S**tatus **R**egister | &mdash;        |

### Argument Registers

While the argument registers do not hold any specific meaning to the processor itself, it is the convention of LANv1 that arguments to procedures be placed in the argument registers, A0 to A3, also knows as R0 to R3.
Callers of a procedure cannot expect data to remain within these registers after the procedure completes.
Additionally, the argument registers may optionally be used to return data from a procedure.
This should be outlined specifically in by the procedure's author.

### General Registers

The general registers, G0 to G8, also known as R4 to R12, do not have any special use.
Rather, as general purpose registers their usage within a program is up to the programmer.
It is important to note that if they are modified within in a procedure, the previous values must be restored by the callee before returning.
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

### Status Register

The [status register][status-register] (SR) is responsible for maintaining current status of the processor, and is different from other registers in several regards.
Unlike any of the numbered registers, the status register is not addressable by the processor; this means it cannot be read or modified directly.
Rather, it is updated automatically to reflect the current processor status.
Several instructions update the condition code flags, which are read by conditional [`BRA`](./inst/BRA.md) instructions.

Layout:
```
│15           4│ 3 │ 2 │ 1 │ 0 │
┌──────────────┬───┬───┬───┬───┐
│ ------------ │ C │ V │ N │ Z │
└──────────────┴───┴───┴───┴───┘
```

Legend:
| Format | Use                            |
| ------ | ------------------------------ |
| `C`    | [Carry flag][carry-flag]       |
| `N`    | [Negative flag][negative-flag] |
| `V`    | [Overflow flag][overflow-flag] |
| `Z`    | [Zero flag][zero-flag]         |
| `-`    | Unused                         |

[stack-pointer]: https://en.wikipedia.org/wiki/Call_stack#STACK-POINTER
[hardware-stack]: https://en.wikipedia.org/wiki/Stack_(abstract_data_type)#Hardware_stack
[link-register]: https://en.wikipedia.org/wiki/Link_register
[program-counter]: https://en.wikipedia.org/wiki/Program_counter
[status-register]: https://en.wikipedia.org/wiki/Status_register
[carry-flag]: https://en.wikipedia.org/wiki/Carry_flag
[negative-flag]: https://en.wikipedia.org/wiki/Negative_flag
[overflow-flag]: https://en.wikipedia.org/wiki/Overflow_flag
[zero-flag]: https://en.wikipedia.org/wiki/Zero_flag
