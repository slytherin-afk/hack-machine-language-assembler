# HACK Assembler

An **assembler** that converts the **HACK instruction set** defined in the `nand2tetris` course into machine language, according to the Hack computer hardware specifications.

---

## Overview

This project provides an assembler for the **Hack Assembly Language**. It takes an `.asm` file as input, containing Hack assembly instructions, and produces an equivalent `.hack` file containing Hack machine code (binary instructions).

The Hack instruction set and hardware are defined in the **Nand2Tetris** course. This project implements the translation from human-readable assembly instructions to machine-readable binary code.

---

## Features

- Supports both **A-instructions** and **C-instructions**:
  - **A-instruction**: `@value` (loads a value into the A-register).
  - **C-instruction**: `dest=comp;jump` (computational instructions with optional destination and jump fields).
- Handles labels (e.g., `(LOOP)`) by translating them into memory addresses.
- Ignores comments and whitespace for clean assembly processing.
- Outputs clean, binary machine code for the Hack computer.

---

## Input and Output

1. **Input**: A `.asm` file containing Hack assembly instructions.  
   Example:

   ```asm
   // Sample Assembly Code
   @2
   D=A
   @3
   D=D+A
   @0
   M=D
   ```

2. **Output**: A `.hack` file containing binary machine instructions.  
   Example:

   ```binary
   0000000000000010
   1110110000010000
   0000000000000011
   1110000010010000
   0000000000000000
   1110001100001000
   ```

---

## Usage

### 1. Build the Assembler

To build the assembler, compile the source code.

```bash
cargo build
```

### 2. Run the Assembler

Run the executable and provide the input `.asm` file:

```bash
./hack-machine-language-assembler input.asm -o output.bin
```

The assembler will generate a file with the binary instructions.

---

## References

- [Nand2Tetris](https://www.nand2tetris.org/)
