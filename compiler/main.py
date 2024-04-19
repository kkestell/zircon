import struct
from dataclasses import dataclass

VAL_NUMBER = 0x01
VAL_BOOLEAN = 0x02
VAL_STRING = 0x03

OP_PUSH_CONST = 0x01
OP_ADD = 0x10
OP_SUBTRACT = 0x11
OP_MULTIPLY = 0x12
OP_DIVIDE = 0x13
OP_MODULO = 0x14
OP_NEGATE = 0x15
OP_AND = 0x20
OP_OR = 0x21
OP_NOT = 0x22
OP_EQUAL = 0x30
OP_JUMP = 0x40
OP_JUMP_IF_TRUE = 0x41
OP_JUMP_IF_FALSE = 0x42
OP_PRINT = 0x60
OP_GET_LOCAL = 0x70
OP_SET_LOCAL = 0x71
OP_CALL = 0x80
OP_RETURN = 0x81
OP_HALT = 0xFF


@dataclass
class Instruction:
    opcode: int
    operand: int = None


class BinaryData:
    def __init__(self):
        self.data = bytearray()

    def u8(self, value):
        self.data += struct.pack('<B', value)

    def u16(self, value):
        self.data += struct.pack('<H', value)

    def u32(self, value):
        self.data += struct.pack('<I', value)

    def f64(self, value):
        self.data += struct.pack('<d', value)

    def string(self, value):
        encoded_str = value.encode('utf-8')
        self.u16(len(encoded_str))
        self.data += encoded_str

    def append(self, data):
        self.data += data

    def bytes(self):
        return bytes(self.data)


class BytecodeBuilder:
    def __init__(self):
        self.constants = []
        self.functions = []
        self.current_function_instructions = []

    def add_constant(self, value):
        self.constants.append(value)
        return len(self.constants) - 1

    def add_instruction(self, opcode, operand=None):
        self.current_function_instructions.append(Instruction(opcode, operand))

    def start_function(self):
        self.current_function_instructions = []

    def end_function(self, num_args):
        func_data = BinaryData()
        for instr in self.current_function_instructions:
            func_data.u8(instr.opcode)
            if instr.operand is not None:
                func_data.u16(instr.operand)

        self.functions.append((len(self.current_function_instructions), num_args, func_data.bytes()))

    def write(self, filename):
        bytecode = BinaryData()
        bytecode.append(b'ZRCN')
        bytecode.u8(1)

        bytecode.u32(len(self.constants))
        for const in self.constants:
            if isinstance(const, float):
                bytecode.u8(VAL_NUMBER)
                bytecode.f64(const)
            elif isinstance(const, bool):
                bytecode.u8(VAL_BOOLEAN)
                bytecode.u8(int(const))
            elif isinstance(const, str):
                bytecode.u8(VAL_STRING)
                bytecode.string(const)

        bytecode.u32(len(self.functions))
        for num_instructions, num_args, func_data in self.functions:
            bytecode.u32(num_instructions)
            bytecode.u32(num_args)
            bytecode.append(func_data)

        with open(filename, 'wb') as file:
            file.write(bytecode.bytes())


if __name__ == '__main__':
    bc = BytecodeBuilder()

    c1 = bc.add_constant(37.0)
    c2 = bc.add_constant(25.0)
    bc.start_function()
    bc.add_instruction(OP_PUSH_CONST, c1)
    bc.add_instruction(OP_SET_LOCAL, 0)
    bc.add_instruction(OP_PUSH_CONST, c2)
    bc.add_instruction(OP_SET_LOCAL, 1)
    bc.add_instruction(OP_GET_LOCAL, 0)
    bc.add_instruction(OP_GET_LOCAL, 1)
    bc.add_instruction(OP_ADD)
    bc.add_instruction(OP_NEGATE)
    bc.add_instruction(OP_PRINT)
    bc.add_instruction(OP_HALT)
    bc.end_function(num_args=0)

    bc.write('example.bcv')

# if __name__ == '__main__':
#     builder = BytecodeBuilder()
#
#     c1 = builder.add_constant(1.0)
#     c2 = builder.add_constant(True)
#     c3 = builder.add_constant("hello world")
#     builder.start_function()
#     builder.add_instruction(OP_PUSH_CONST, c1)
#     builder.add_instruction(OP_PUSH_CONST, c2)
#     builder.add_instruction(OP_PUSH_CONST, c3)
#     builder.add_instruction(OP_PRINT)
#     builder.add_instruction(OP_PRINT)
#     builder.add_instruction(OP_PRINT)
#     builder.add_instruction(OP_HALT)
#     builder.end_function(num_args=0)
#
#     builder.build('example.bcv')

# if __name__ == '__main__':
#     builder = BytecodeBuilder()
#
#     const_index = builder.add_constant(42.0)
#     builder.start_function()
#     builder.add_instruction(OP_PUSH_CONST, const_index)
#     builder.add_instruction(OP_PRINT)
#     builder.add_instruction(OP_HALT)
#     builder.end_function(num_args=0)
#
#     builder.build('example.bcv')