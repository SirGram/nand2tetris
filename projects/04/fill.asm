

@8192 
D=A // 256 x (512 / 16) Screen Dimensions
@words
M=D
@SCREEN
D=A
@address
M=D

// --------------
// Keyboard press
// --------------
@KBD
D=M
@FILL_SCREEN
D;JNE
@EMPTY_SCREEN
0;JMP
// --------------

(FILL_SCREEN)
@words
D=M
@END
D;JEQ // finished processing all pixels
@address
A=M 
M=-1 // black 16 pixels
@address
M=M+1
@words
M=M-1
@FILL_SCREEN
0;JMP

(EMPTY_SCREEN)
@words
D=M
@END
D;JEQ // finished processing all pixels
@address
A=M 
M=0 // white 16 pixels
@address
M=M+1
@words
M=M-1
@EMPTY_SCREEN
0;JMP

// --------------
(END)
@END
0;JMP












