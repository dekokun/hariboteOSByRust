[BITS 32]

GLOBAL _io_hlt
GLOBAL _write_mem8

[SECTION .text]

_io_hlt:
    HLT
    RET

_write_mem8: ; void write_mem8(int addr, int data)
    MOV ECX, [ESp+4]
    MOV AL, [ESP+8]
    MOV [ECX], AL
    RET
