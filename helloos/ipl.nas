;hello-os

    ORG 0x7c00
    ;DB 0xeb, 0x4e, 0x90

    
    JMP entry
    DB 0x90
    DB "HELLOIPL"       ;name of ipl, 8 bytes
    DW 512              ;sector size
    DB 1                ;cluster size
    DW 1                ;FAT starter
    DB 2                ;FAT num
    DW 224              ;root dir size
    DW 2280             ;disk size
    DB 0xf0             ;disk type, must be 0xf0
    DW 9                ;FAT length
    DW 18               ;sector num of a track
    DW 2
    DD 0
    DD 2880
    DB 0, 0, 0x29
    DD 0xffffffff
    DB "HELLO-OS   "    ;disk name, 11 bytes
    DB "FAT12   "       ;disk fmt, 8 bytes
    RESB 18             ;blank 18 bytes

    ;DB 0xb8, 0x00, 0x00, 0x8e, 0xd0, 0xbc, 0x00, 0x7c
    ;DB 0x8e, 0xd8, 0x8e, 0xc0, 0xbe, 0x74, 0x7c, 0x8a
    ;DB 0x04, 0x83, 0xc6, 0x01, 0x3c, 0x00, 0x74, 0x09
    ;DB 0xb4, 0x0e, 0xbb, 0x0f, 0x00, 0xcd, 0x10, 0xeb
    ;DB 0xee, 0xf4, 0xeb, 0xfd

entry:
    MOV AX, 0
    MOV SS, AX
    MOV SP, 0x7c00
    MOV DS, AX
    MOV ES, DX
    MOV SI, msg

putloop:
    MOV AL, [SI]
    ADD SI, 1
    CMP AL, 0
    JE fin
    MOV AH, 0x0e
    MOV BX, 15
    INT 0x10
    JMP putloop

fin:
    HLT
    JMP fin

msg:
    DB 0x0a, 0x0a
    DB "hello, world"
    DB 0x0a
    DB 0

    RESB 0x7dfe-$
    DB 0x55, 0xaa

    ;DB 0xf0, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00
    ;RESB 4600
    ;DB 0xf0, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00
    ;RESB 1469432