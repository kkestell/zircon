# Zircon

A stack-based virtual machine written in Rust, loosely based on the design presented in [Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## Bytecode

### Overview

Zircon Bytecode is a binary format used for the execution of programs in the Zircon virtual machine. All multi-byte values in the Zircon Bytecode file are stored in little-endian byte order.

### File Structure

A Zircon Bytecode file consists of a header, a constants table, and a functions section.

#### Header

* Magic Number: ZRCN (4 bytes)
* Version: 1 byte

#### Constants Table

* Number of Constants: 4 bytes (unsigned int)
* Constants: A sequence of entries, each with a type specifier followed by the constant value.
    * Number: 1 byte type specifier (`0x01`) + 8 bytes for the double-precision floating-point value.
    * Boolean: 1 byte type specifier (`0x02`) + 1 byte for the boolean value (0 for false, 1 for true).
    * String: 1 byte type specifier (`0x03`) + 2 bytes (unsigned short) for the string length in bytes + N bytes for the UTF-8 encoded string.

#### Functions Section

* Number of Functions: 4 bytes (unsigned int)
* Functions: A sequence of function definitions, each consisting of:
    * Number of Instructions: 4 bytes (unsigned int)
    * Number of Arguments: 4 bytes (unsigned int)
    * Instructions: A sequence of instruction bytes.

### Instructions

| Opcode             | Hex Value | Operand(s)            | Description                                                                                      |
| ------------------ | --------- | --------------------- | ------------------------------------------------------------------------------------------------ |
| `OP_PUSH_CONST`    | `0x01`    | 2-byte constant index | Pushes a specified constant onto the stack.                                                      |
| `OP_ADD`           | `0x10`    | None                  | Adds the top two values on the stack, pushing the result.                                        |
| `OP_SUBTRACT`      | `0x11`    | None                  | Subtracts the top stack value from the second top value, pushing the result.                     |
| `OP_MULTIPLY`      | `0x12`    | None                  | Multiplies the top two stack values, pushing the result.                                         |
| `OP_DIVIDE`        | `0x13`    | None                  | Divides the second top stack value by the top, pushing the result.                               |
| `OP_MODULO`        | `0x14`    | None                  | Calculates the modulus of the second top value by the top, pushing the result.                   |
| `OP_NEGATE`        | `0x15`    | None                  | Negates the top value on the stack, pushing the result.                                          |
| `OP_AND`           | `0x20`    | None                  | Performs a logical AND on the top two stack values, pushing the result.                          |
| `OP_OR`            | `0x21`    | None                  | Performs a logical OR on the top two stack values, pushing the result.                           |
| `OP_NOT`           | `0x22`    | None                  | Performs a logical NOT on the top stack value, pushing the result.                               |
| `OP_EQUAL`         | `0x30`    | None                  | Checks if the top two stack values are equal, pushing the boolean result.                        |
| `OP_JUMP`          | `0x40`    | 2-byte target address | Unconditionally jumps to the specified instruction address.                                      |
| `OP_JUMP_IF_TRUE`  | `0x41`    | 2-byte target address | Jumps to the specified address if the top stack value is true, popping the value.                |
| `OP_JUMP_IF_FALSE` | `0x42`    | 2-byte target address | Jumps to the specified address if the top stack value is false, popping the value.               |
| `OP_PRINT`         | `0x60`    | None                  | Prints the top value of the stack and pops it.                                                   |
| `OP_GET_LOCAL`     | `0x70`    | 2-byte variable index | Pushes the value of a local variable onto the stack.                                             |
| `OP_SET_LOCAL`     | `0x71`    | 2-byte variable index | Sets a local variable to the top value on the stack, popping the value.                          |
| `OP_CALL`          | `0x80`    | 2-byte function index | Initiates a function call with the specified index, setting up a new call frame.                 |
| `OP_RETURN`        | `0x81`    | None                  | Returns from the current function, possibly pushing a return value onto the stack of the caller. |
| `OP_HALT`          | `0xFF`    | None                  | Halts the VM execution.                                                                          |
