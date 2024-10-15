// i var is R0 value
@R0
D=M
@i
M=D
// reset R2
@R2
M=0

(LOOP)
@i
D=M
@END
D;JEQ // i==0
@R1
D=M
@R2
M=D+M
@i
M=M-1
@LOOP
0;JMP

(END)
@END
0;JMP



