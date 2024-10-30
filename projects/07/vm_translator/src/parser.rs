use crate::CommandType;

pub struct Parser {
    input: Vec<String>,
    current_index: usize,
    current_instruction: Option<String>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let input = input
            .lines()
            .filter_map(|line| {
                // filter out empty lines and comments
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with("//") {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            })
            .collect();
        Parser {
            input,
            current_index: 0,
            current_instruction: None,
        }
    }
    pub fn has_more_lines(&self) -> bool {
        self.current_index < self.input.len()
    }
    pub fn advance(&mut self) {
        if self.has_more_lines() {
            self.current_instruction = self.input.get(self.current_index).cloned();
            self.current_index += 1;
        }
    }

    pub fn command_type(&self) -> Option<CommandType> {
        let first_word = self
            .current_instruction
            .as_ref()?
            .split_whitespace()
            .next()?;
        match first_word {
            "push" => Some(CommandType::CPush),
            "pop" => Some(CommandType::CPop),
            "label" => Some(CommandType::CLabel),
            "goto" => Some(CommandType::CGoto),
            "if" => Some(CommandType::CIf),
            "function" => Some(CommandType::CFunction),
            "return" => Some(CommandType::CReturn),
            "call" => Some(CommandType::CCall),
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                Some(CommandType::CArithmetic)
            }
            _ => None,
        }
    }

    pub fn arg1(&self) -> Option<String> {
        match self.command_type()? {
            CommandType::CArithmetic => {
                // For arithmetic commands, `arg1` is the command itself (e.g., "add", "sub")
                self.current_instruction
                    .as_ref()?
                    .split_whitespace()
                    .next()
                    .map(|s| s.to_string())
            }
            _ => {
                // For other commands, `arg1` is the first argument after the command name
                self.current_instruction
                    .as_ref()?
                    .split_whitespace()
                    .nth(1)
                    .map(|s| s.to_string())
            }
        }
    }

    pub fn arg2(&self) -> Option<String> {
        self.current_instruction
            .as_ref()?
            .split_whitespace()
            .nth(2)
            .map(|s| s.to_string())
    }
}
