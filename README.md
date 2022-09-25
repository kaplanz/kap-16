# kap-16

<p align="center">
  <img width="100" height="100" src="./docs/assets/img/cpu.svg"/>
</p>

<p align="center">
  <q>
    If you wish to make an apple pie from scratch, you must first invent the universe.
  </q>
  &mdash;
  <i>
    Carl Sagan
  </i>
</p>

---

KAP-16 &mdash; short for **K**inda **A**dequate **P**rocessor, **16**-bit &mdash; is a toy microprocessor architecture.
It runs its own instruction set architecture (ISA) called LANv1: **L**ite **A**ssembly **N**otation, **v**ersion **1**.

## Contents

### Specification

KAP-16's specifications are outlined in [`spec/`](./spec).
Read the [`README.md`](./spec/README.md) for information on the architecture.

### Assembler

The assembler is responsible for converting programs written in LANv1 assembly language into bit patterns that can be interpreted by KAP-16.
It is also capable of linking multiple object files into an executable, which can be run directly on the processor.
Since KAP-16 does not have an operating system, the executables it runs are actually a memory image.

Source code for the assembler can be found in the [`asm/`](./asm) directory.
Read the [`README.md`](./asm/README.md) for information on building and running the assembler.

### Emulator

A fully functional emulator for the KAP-16 can be found inside the [`emu/`](./emu) directory.
Read the [`README.md`](./emu/README.md) for information on building and running the emulator.
