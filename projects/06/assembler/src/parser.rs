pub struct Parser {
    input: Vec<String>,
    current_index: usize,
    current_instruction: Option<String>,
}

#[derive(PartialEq)]
pub enum InstructionType {
    A_INSTRUCTION,
    C_INSTRUCTION,
    L_INSTRUCTION,
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

    pub fn reset(&mut self) {
        self.current_index = 0;
        self.current_instruction = None;
    }

    pub fn has_more_lines(&self) -> bool {
        if self.current_index < self.input.len() {
            return true;
        }
        return false;
    }

    pub fn advance(&mut self) {
        if self.has_more_lines() {
            self.current_instruction = Some(self.input[self.current_index].clone());
            self.current_index += 1;
        } else {
            self.current_instruction = None;
        }
    }
    pub fn instruction_type(&self) -> Option<InstructionType> {
        if let Some(ref instruction) = self.current_instruction {
            if instruction.starts_with('@') {
                return Some(InstructionType::A_INSTRUCTION);
            } else if instruction.starts_with('(') && instruction.ends_with(')') {
                return Some(InstructionType::L_INSTRUCTION);
            } else {
                return Some(InstructionType::C_INSTRUCTION);
            }
        }
        None
    }
    pub fn symbol(&self) -> Option<String> {
        if let Some(ref instruction) = self.current_instruction {
            let instruction_type = self.instruction_type();

            if let Some(ref instruction_type) = instruction_type {
                if instruction_type == &InstructionType::A_INSTRUCTION {
                    // Return the symbol after '@'
                    return Some(instruction[1..].to_string());
                } else if instruction_type == &InstructionType::L_INSTRUCTION {
                    // Return the symbol inside parentheses
                    return Some(instruction[1..instruction.len() - 1].to_string());
                }
            }
        }

        None
    }
    pub fn dest(&self) -> Option<String> {
        if let Some(ref instruction) = self.current_instruction {
            if let Some(equal_pos) = instruction.find('=') {
                return Some(instruction[0..equal_pos].to_string());
            }
        }
        Some("null".to_string()) // Return "null" if no dest part is found
    }

    pub fn comp(&self) -> Option<String> {
        if let Some(ref instruction) = self.current_instruction {
            let equal_pos = instruction.find('=');
            let semicolon_pos = instruction.find(';');

            match (equal_pos, semicolon_pos) {
                (Some(eq), Some(semi)) => Some(instruction[eq + 1..semi].to_string()),
                (Some(eq), None) => Some(instruction[eq + 1..].to_string()),
                (None, Some(semi)) => Some(instruction[0..semi].to_string()),
                (None, None) => Some(instruction.to_string()),
            }
        } else {
            None
        }
    }

    pub fn jump(&self) -> Option<String> {
        if let Some(ref instruction) = self.current_instruction {
            if let Some(semicolon_pos) = instruction.find(';') {
                return Some(instruction[semicolon_pos + 1..].to_string());
            }
        }
        Some("null".to_string()) // Return "null" if no jump part is found
    }
}

