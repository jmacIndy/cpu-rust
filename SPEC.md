# Instructions
Mnemonic | Operand | OpCode | Description
-------- | ------- | ------ | -----------
HALT||0x00|Halt the CPU
SET0|*value*|0x01|Set register 0 = *value*
SET1|*value*|0x02|Set register 1 = *value*
ADD||0x03|Set register 0 = register 0 + register 1
STOR|*address*|0x04|Store register 0 on heap at *address*
PRT|*address*|0x05|Print contents of heap at *address*
MULT||0x07|Set register 0 = register 0 * register 1
DIV|| 0x08|Set register 0 = register 0 / register 1
SUB||0x09|Set register 0 = register 0 - register 1
JEQ|*address*|0x0A|Jump to *address* if greater than and less than flags are 0
JNE|*address*|0x0B|Jump to *address* if greater than or less than flag is set
JLT|*address*|0x0C|Jump to *address* if less than flag is set
JGT|*address*|0x0D|Jump to *address* if greater than flag is set
CALL|*address*|0x0E|Jump to subroutine at *address*
RET||0x0F|Return from subroutine to statement after CALL
INT||0x10|Handle interrupts
CMP||0x11|Compare values
JNZ|*address*|0x12|Jump to *address* if R0 is NOT zero
JMP|*address*|0x13|Unconditional jump to *address*
JZ|*address*|0x14|Jump to *address* if R0 is zero
INC0||0x15|Increment register 0 by 1
INC1||0x16|Increment register 1 by 1
DEC0||0x17|Decrement register 0 by 1
DEC1||0x18|Decrement register 1 by 1
LD0|*address*|0x19|Set register 0 = value at *address*
LD1|*address*|0x1A|Set register 1 = value at *address*
TST||0x1B|Set zero or non zero flag based on value in register 0

# Comments/Blanks
any line that starts with //
or // til end of line

any line can be blank

# Directives
Directive | Meaning
--------- | -------
.CODE|starts code section
.VARS|starts a section of variables;followed by *label* *value*; continues until .CODE directive is found

Any *value* in code
* *number* 
* =X*hex number* 
* *label*
