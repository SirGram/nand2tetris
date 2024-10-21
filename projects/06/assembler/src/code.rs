
pub fn dest(dest_str: &str) -> Option<&'static str> {
    match dest_str {
        "null" => Some("000"),
        "M" => Some("001"),
        "D" => Some("010"),
        "DM" => Some("011"),
        "A" => Some("100"),
        "AM" => Some("101"),
        "AD" => Some("110"),
        "ADM" => Some("111"),
        _=> None,
    }
}

pub fn comp(comp_str: &str) -> Option<&'static str> {
    match comp_str {
        "0" => Some("101010"),
        "1" => Some("111111"),
        "-1" => Some("111010"),
        "D" => Some("001100"),
        "A" | "M" => Some("110000"),
        "!D" => Some("001101"),
        "!A" | "!M" => Some("110001"),
        "-D" => Some("001111"),
        "-A" | "-M" => Some("110011"),
        "D+1" => Some("011111"),
        "A+1" | "M+1" => Some("110111"),
        "D-1" => Some("001110"),
        "A-1" | "M-1" => Some("110010"),
        "D+A" | "D+M" => Some("000010"),
        "D-A" | "D-M" => Some("010011"),
        "A-D" | "M-D" => Some("000111"),
        "D&A" | "D&M" => Some("000000"),
        "D|A" | "D|M" => Some("010101"),
        _ => None,
    }
}

pub fn jump(jump_str: &str) -> Option<&'static str> {
    match jump_str {
        "null" => Some("000"),
        "JGT" => Some("001"),
        "JEQ" => Some("010"),
        "JGE" => Some("011"),
        "JLT" => Some("100"),
        "JNE" => Some("101"),
        "JLE" => Some("110"),
        "JMP" => Some("111"),
        _=> None,
    }
}