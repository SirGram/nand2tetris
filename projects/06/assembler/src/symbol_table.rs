use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            table: HashMap::new(),
        }
    }

    pub fn add_predefined_symbols(&mut self) {
        let predefined_symbols = [
            ("SP", 0),
            ("LCL", 1),
            ("ARG", 2),
            ("THIS", 3),
            ("THAT", 4),
            ("R0", 0),
            ("R1", 1),
            ("R2", 2),
            ("R3", 3),
            ("R4", 4),
            ("R5", 5),
            ("R6", 6),
            ("R7", 7),
            ("R8", 8),
            ("R9", 9),
            ("R10", 10),
            ("R11", 11),
            ("R12", 12),
            ("R13", 13),
            ("R14", 14),
            ("R15", 15),
            ("SCREEN", 16384),
            ("KBD", 24576),
        ];
        for &(symbol, address) in &predefined_symbols {
            self.table.insert(symbol.to_string(), address);
        }
    }

    pub fn add_entry(&mut self, symbol: &str, address: usize) {
        self.table.insert(symbol.to_string(), address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn getAddress(&self, symbol: &str) -> Option<usize> {
        self.table.get(symbol).cloned()
    }
}