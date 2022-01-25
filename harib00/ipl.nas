;hello-os

    CYLS EQU 10         ;读取10个柱面
    ORG 0x7c00

;标准FAT12格式软盘专用的代码
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

entry:                  ;初始化部分
    MOV AX, 0
    MOV SS, AX
    MOV SP, 0x7c00
    MOV DS, AX
    
    MOV AX, 0x0820
    MOV ES, AX
    MOV CH, 0           ;从0号柱面开始
    MOV DH, 0           ;使用磁头0
    MOV CL, 2           ;从扇区2开始读

readloop:
    MOV SI, 0           ;SI记录错误次数

retry:
    MOV AH, 0x02
    MOV AL, 1           ;读模式，读取一个扇区
    MOV BX, 0
    MOV DL, 0x00
    INT 0x13
    JNC next
    ADD SI, 1           ;如果当前读取错误
    CMP SI, 5
    JAE error
    MOV AH, 0x00        ;重置驱动器
    MOV DL, 0x00
    INT 0x13
    JMP retry

next:
    MOV AX, ES
    ADD AX, 0x0020
    MOV ES, AX          ;将ES的值加16，相当于内存地址加16*16=512
                        ;取址时会将ES的值乘以16
    ADD CL, 1           ;扇区号加一
    CMP CL, 18
    JBE readloop
    MOV CL, 1
    ADD DH, 1           ;磁头号加一
    CMP DH, 2
    JB readloop
    MOV DH, 0
    ADD CH, 1           ;柱面号加一
    CMP CH, CYLS
    JB readloop

    ;读取完毕做的事
    MOV [0x0ff0], CH
    JMP 0xc200          ;跳转至操作系统的位置

error:
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
    DB "read error"
    DB 0x0a
    DB 0
    RESB 0x7dfe-$		; 填写0x00直到0x001fe
    DB 0x55, 0xaa
