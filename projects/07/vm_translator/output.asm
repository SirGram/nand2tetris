// push constant 7
@7
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 8
@8
D=A
@SP
A=M
M=D
@SP
M=M+1

// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1

// push constant 5
@5
D=A
@SP
A=M
M=D
@SP
M=M+1

// sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1

// push constant 10
@10
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 15
@15
D=A
@SP
A=M
M=D
@SP
M=M+1

// comparison JGT
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@LABEL_1
D;JGT
@SP
A=M
M=0
@LABEL_2
0;JMP
(LABEL_1)
@SP
A=M
M=-1
(LABEL_2)
@SP
M=M+1

// push constant 20
@20
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 10
@10
D=A
@SP
A=M
M=D
@SP
M=M+1

// comparison JLT
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@LABEL_3
D;JLT
@SP
A=M
M=0
@LABEL_4
0;JMP
(LABEL_3)
@SP
A=M
M=-1
(LABEL_4)
@SP
M=M+1

// push constant 5
@5
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 5
@5
D=A
@SP
A=M
M=D
@SP
M=M+1

// comparison JEQ
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@LABEL_5
D;JEQ
@SP
A=M
M=0
@LABEL_6
0;JMP
(LABEL_5)
@SP
A=M
M=-1
(LABEL_6)
@SP
M=M+1

// push constant 3
@3
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 6
@6
D=A
@SP
A=M
M=D
@SP
M=M+1

// and
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M&D
@SP
M=M+1

// push constant 3
@3
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 6
@6
D=A
@SP
A=M
M=D
@SP
M=M+1

// or
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M|D
@SP
M=M+1

// not
@SP
M=M-1
A=M
M=!M
@SP
M=M+1

// push constant 2
@2
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop temp 0
@SP
M=M-1
A=M
D=M
@5
M=D

// push constant 3
@3
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop temp 1
@SP
M=M-1
A=M
D=M
@6
M=D

// push temp 0
@5
D=M
@SP
A=M
M=D
@SP
M=M+1

// push temp 1
@6
D=M
@SP
A=M
M=D
@SP
M=M+1

// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1

// push constant 3030
@3030
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop pointer 0
@SP
M=M-1
A=M
D=M
@3
M=D

// push constant 4040
@4040
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop pointer 1
@SP
M=M-1
A=M
D=M
@4
M=D

// push pointer 0
@3
D=M
@SP
A=M
M=D
@SP
M=M+1

// push pointer 1
@4
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 100
@100
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop static 5
@SP
M=M-1
A=M
D=M
@21
M=D

// push constant 200
@200
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop static 6
@SP
M=M-1
A=M
D=M
@22
M=D

// push static 5
@21
D=M
@SP
A=M
M=D
@SP
M=M+1

// push static 6
@22
D=M
@SP
A=M
M=D
@SP
M=M+1

// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1

// End of code