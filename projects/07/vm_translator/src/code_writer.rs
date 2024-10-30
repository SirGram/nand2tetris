use crate::CommandType;

pub struct CodeWriter {
    code: Vec<String>,
    label_count: i32,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            label_count: 0,
        }
    }

    pub fn close(&mut self) -> String {
        self.code.push("// End of code".to_string());
        self.code.join("\n")
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: i16) {
        match command {
            CommandType::CPush => self.write_push(segment, index),
            CommandType::CPop => self.write_pop(segment, index),
            _ => panic!("Invalid command type"),
        }
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        let asm_code = match command {
            // Binary arithmetic operations
            "add" => format!(
                "// add\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 D=M\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 M=M+D\n\
                 @SP\n\
                 M=M+1\n"
            ),
            "sub" => format!(
                "// sub\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 D=M\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 M=M-D\n\
                 @SP\n\
                 M=M+1\n"
            ),
            "neg" => format!(
                "// neg\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 M=-M\n\
                 @SP\n\
                 M=M+1\n"
            ),
            "and" => format!(
                "// and\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 D=M\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 M=M&D\n\
                 @SP\n\
                 M=M+1\n"
            ),
            "or" => format!(
                "// or\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 D=M\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 M=M|D\n\
                 @SP\n\
                 M=M+1\n"
            ),
            "not" => format!(
                "// not\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 M=!M\n\
                 @SP\n\
                 M=M+1\n"
            ),
            // Comparison operations
            "eq" => self.write_comparison("JEQ"),
            "gt" => self.write_comparison("JGT"),
            "lt" => self.write_comparison("JLT"),

            _ => panic!("Unknown arithmetic command: {}", command),
        };

        self.code.push(asm_code);
    }

    fn write_push(&mut self, segment: &str, index: i16) {
        let asm_code = match segment {
            // Constant: load the constant `index` into `D` and push onto the stack.
            "constant" => format!(
                "// push constant {}\n\
                 @{}\n\
                 D=A\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n",
                index, index
            ),
            // Temp: access RAM[5 + index] and push value to stack.
            "temp" => format!(
                "// push temp {}\n\
                 @{}\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n",
                index,
                5 + index
            ),
            // Local, Argument, This, That: get base + index and push the value at that address.
            "local" | "argument" | "this" | "that" => {
                let base = match segment {
                    "local" => "LCL",
                    "argument" => "ARG",
                    "this" => "THIS",
                    "that" => "THAT",
                    _ => unreachable!(),
                };
                format!(
                    "// push {} {}\n\
                     @{}\n\
                     D=A\n\
                     @{}\n\
                     A=D+M\n\
                     D=M\n\
                     @SP\n\
                     A=M\n\
                     M=D\n\
                     @SP\n\
                     M=M+1\n",
                    segment, index, index, base
                )
            }
            // Pointer: access THIS or THAT directly.
            "pointer" => format!(
                "// push pointer {}\n\
                 @{}\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n",
                index,
                if index == 0 { 3 } else { 4 }
            ),
            // Static: access the static segment.
            "static" => format!(
                "// push static {}\n\
                 @{}\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n",
                index,
                16 + index
            ),
            _ => panic!("Unknown segment: {}", segment),
        };
        self.code.push(asm_code);
    }

    fn write_pop(&mut self, segment: &str, index: i16) {
        let asm_code = match segment {
            // Temp: store top of stack in RAM[5 + index].
            "temp" => format!(
                "// pop temp {}\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 D=M\n\
                 @{}\n\
                 M=D\n",
                index,
                5 + index
            ),
            // Local, Argument, This, That: store top of stack at base + index.
            "local" | "argument" | "this" | "that" => {
                let base = match segment {
                    "local" => "LCL",
                    "argument" => "ARG",
                    "this" => "THIS",
                    "that" => "THAT",
                    _ => unreachable!(),
                };
                format!(
                    "// pop {} {}\n\
                     @{}\n\
                     D=A\n\
                     @{}\n\
                     D=D+M\n\
                     @R13\n\
                     M=D\n\
                     @SP\n\
                     M=M-1\n\
                     A=M\n\
                     D=M\n\
                     @R13\n\
                     A=M\n\
                     M=D\n",
                    segment, index, index, base
                )
            }
            // Pointer: store top of stack in THIS (RAM[3]) or THAT (RAM[4]).
            "pointer" => format!(
                "// pop pointer {}\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 D=M\n\
                 @{}\n\
                 M=D\n",
                index,
                if index == 0 { 3 } else { 4 }
            ),
            // Static: store top of stack in static segment at RAM[16 + index].
            "static" => format!(
                "// pop static {}\n\
                 @SP\n\
                 M=M-1\n\
                 A=M\n\
                 D=M\n\
                 @{}\n\
                 M=D\n",
                index,
                16 + index
            ),
            _ => panic!("Unknown segment: {}", segment),
        };
        self.code.push(asm_code);
    }

    fn write_comparison(&mut self, jump: &str) -> String {
        let label_true = self.new_label();
        let label_end = self.new_label();

        format!(
            "// comparison {}\n\
             @SP\n\
             M=M-1\n\
             A=M\n\
             D=M\n\
             @SP\n\
             M=M-1\n\
             A=M\n\
             D=M-D\n\
             @{}\n\
             D;{}\n\
             @SP\n\
             A=M\n\
             M=0\n\
             @{}\n\
             0;JMP\n\
             ({})\n\
             @SP\n\
             A=M\n\
             M=-1\n\
             ({})\n\
             @SP\n\
             M=M+1\n",
            jump, label_true, jump, label_end, label_true, label_end
        )
    }

    fn new_label(&mut self) -> String {
        self.label_count += 1;
        format!("LABEL_{}", self.label_count)
    }
}
