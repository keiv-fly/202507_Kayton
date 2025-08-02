use super::*;

/// Format bytecode as a human-readable string
pub fn format_bytecode(bytecode: &[u8]) -> Result<String, String> {
    let mut output = String::new();
    let mut pc = 0;

    while pc < bytecode.len() {
        let opcode = bytecode[pc];
        let start_pc = pc;
        pc += 1;

        match opcode {
            LOAD_I64 => {
                if pc >= bytecode.len() {
                    return Err(format!(
                        "Incomplete LOAD_I64 instruction at pc {}: missing register",
                        start_pc
                    ));
                }
                let reg = bytecode[pc];
                pc += 1;
                if pc + 7 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete LOAD_I64 instruction at pc {}: missing value bytes",
                        start_pc
                    ));
                }
                let value = i64::from_le_bytes([
                    bytecode[pc],
                    bytecode[pc + 1],
                    bytecode[pc + 2],
                    bytecode[pc + 3],
                    bytecode[pc + 4],
                    bytecode[pc + 5],
                    bytecode[pc + 6],
                    bytecode[pc + 7],
                ]);
                pc += 8;
                output.push_str(&format!("{} LOAD_I64 r{}, {}\n", start_pc, reg, value));
            }
            LOAD_F64 => {
                if pc >= bytecode.len() {
                    return Err(format!(
                        "Incomplete LOAD_F64 instruction at pc {}: missing register",
                        start_pc
                    ));
                }
                let reg = bytecode[pc];
                pc += 1;
                if pc + 7 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete LOAD_F64 instruction at pc {}: missing value bytes",
                        start_pc
                    ));
                }
                let value = f64::from_le_bytes([
                    bytecode[pc],
                    bytecode[pc + 1],
                    bytecode[pc + 2],
                    bytecode[pc + 3],
                    bytecode[pc + 4],
                    bytecode[pc + 5],
                    bytecode[pc + 6],
                    bytecode[pc + 7],
                ]);
                pc += 8;
                output.push_str(&format!("{} LOAD_F64 r{}, {}\n", start_pc, reg, value));
            }
            LOAD_CONST_VALUE => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete LOAD_CONST_VALUE instruction at pc {}: missing operands",
                        start_pc
                    ));
                }
                let reg = bytecode[pc];
                pc += 1;
                let index = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                pc += 2;
                output.push_str(&format!("{} LOAD_CONST_VALUE r{}, {}\n", start_pc, reg, index));
            }
            LOAD_CONST_SLICE => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete LOAD_CONST_SLICE instruction at pc {}: missing operands",
                        start_pc
                    ));
                }
                let reg = bytecode[pc];
                pc += 1;
                let index = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                pc += 2;
                output.push_str(&format!("{} LOAD_CONST_SLICE r{}, {}\n", start_pc, reg, index));
            }
            ADD_I64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete ADD_I64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!(
                    "{} ADD_I64 r{}, r{}, r{}\n",
                    start_pc, r1, r2, dst
                ));
            }
            SUB_I64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete SUB_I64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!(
                    "{} SUB_I64 r{}, r{}, r{}\n",
                    start_pc, r1, r2, dst
                ));
            }
            MUL_I64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete MUL_I64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!(
                    "{} MUL_I64 r{}, r{}, r{}\n",
                    start_pc, r1, r2, dst
                ));
            }
            GT_I64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete GT_I64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!("{} GT_I64 r{}, r{}, r{}\n", start_pc, r1, r2, dst));
            }
            GTE_I64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete GTE_I64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!("{} GTE_I64 r{}, r{}, r{}\n", start_pc, r1, r2, dst));
            }
            LT_I64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete LT_I64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!("{} LT_I64 r{}, r{}, r{}\n", start_pc, r1, r2, dst));
            }
            LTE_I64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete LTE_I64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!("{} LTE_I64 r{}, r{}, r{}\n", start_pc, r1, r2, dst));
            }
            ADD_F64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete ADD_F64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!(
                    "{} ADD_F64 r{}, r{}, r{}\n",
                    start_pc, r1, r2, dst
                ));
            }
            SUB_F64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete SUB_F64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!(
                    "{} SUB_F64 r{}, r{}, r{}\n",
                    start_pc, r1, r2, dst
                ));
            }
            MUL_F64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete MUL_F64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!(
                    "{} MUL_F64 r{}, r{}, r{}\n",
                    start_pc, r1, r2, dst
                ));
            }
            GT_F64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete GT_F64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!("{} GT_F64 r{}, r{}, r{}\n", start_pc, r1, r2, dst));
            }
            GTE_F64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete GTE_F64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!("{} GTE_F64 r{}, r{}, r{}\n", start_pc, r1, r2, dst));
            }
            LT_F64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete LT_F64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!("{} LT_F64 r{}, r{}, r{}\n", start_pc, r1, r2, dst));
            }
            LTE_F64 => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete LTE_F64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                output.push_str(&format!("{} LTE_F64 r{}, r{}, r{}\n", start_pc, r1, r2, dst));
            }
            JUMP_FORWARD_IF_FALSE => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete JUMP_FORWARD_IF_FALSE instruction at pc {}: missing condition register or offset",
                        start_pc
                    ));
                }
                let cond_reg = bytecode[pc];
                pc += 1;
                let offset = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                let target = pc + offset as usize;
                pc += 2;
                output.push_str(&format!(
                    "{} JUMP_FORWARD_IF_FALSE r{}, {} (offset: {})\n",
                    start_pc, cond_reg, target, offset
                ));
            }
            JUMP_FORWARD_IF_TRUE => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete JUMP_FORWARD_IF_TRUE instruction at pc {}: missing condition register or offset",
                        start_pc
                    ));
                }
                let cond_reg = bytecode[pc];
                pc += 1;
                let offset = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                let target = pc + offset as usize;
                pc += 2;
                output.push_str(&format!(
                    "{} JUMP_FORWARD_IF_TRUE r{}, {} (offset: {})\n",
                    start_pc, cond_reg, target, offset
                ));
            }
            JUMP_BACKWARD_IF_FALSE => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete JUMP_BACKWARD_IF_FALSE instruction at pc {}: missing condition register or offset",
                        start_pc
                    ));
                }
                let cond_reg = bytecode[pc];
                pc += 1;
                let offset = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                pc += 2;
                let target = pc as i64 - offset as i64;
                output.push_str(&format!(
                    "{} JUMP_BACKWARD_IF_FALSE r{}, {} (offset: {})\n",
                    start_pc, cond_reg, target, offset
                ));
            }
            JUMP_BACKWARD_IF_TRUE => {
                if pc + 2 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete JUMP_BACKWARD_IF_TRUE instruction at pc {}: missing condition register or offset",
                        start_pc
                    ));
                }
                let cond_reg = bytecode[pc];
                pc += 1;
                let offset = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                pc += 2;
                let target = pc as i64 - offset as i64;
                output.push_str(&format!(
                    "{} JUMP_BACKWARD_IF_TRUE r{}, {} (offset: {})\n",
                    start_pc, cond_reg, target, offset
                ));
            }
            JMP => {
                if pc + 1 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete JMP instruction at pc {}: missing target address",
                        start_pc
                    ));
                }
                let target = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                pc += 2;
                output.push_str(&format!("{} JMP {}\n", start_pc, target));
            }
            I64_TO_F64 => {
                if pc + 1 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete I64_TO_F64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let src = bytecode[pc];
                let dst = bytecode[pc + 1];
                pc += 2;
                output.push_str(&format!("{} I64_TO_F64 r{}, r{}\n", start_pc, src, dst));
            }
            F64_TO_I64 => {
                if pc + 1 >= bytecode.len() {
                    return Err(format!(
                        "Incomplete F64_TO_I64 instruction at pc {}: missing register operands",
                        start_pc
                    ));
                }
                let src = bytecode[pc];
                let dst = bytecode[pc + 1];
                pc += 2;
                output.push_str(&format!("{} F64_TO_I64 r{}, r{}\n", start_pc, src, dst));
            }
            _ => {
                return Err(format!("{} UNKNOWN_OPCODE 0x{:02X}\n", start_pc, opcode));
            }
        }
    }

    output.push_str(&format!("pc={}\n", pc));
    output.push_str(&format!("bytecode.len()={}\n", bytecode.len()));

    Ok(output)
}

/// Disassemble bytecode and print in human-readable format
pub fn print_bytecode(bytecode: &[u8]) {
    match format_bytecode(bytecode) {
        Ok(formatted) => print!("{}", formatted),
        Err(error) => eprintln!("Error formatting bytecode: {}", error),
    }
}
