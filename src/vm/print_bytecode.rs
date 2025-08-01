use super::*;
/// Disassemble bytecode and print in human-readable format
pub fn print_bytecode(bytecode: &[u8]) {
    let mut pc = 0;

    while pc < bytecode.len() {
        let opcode = bytecode[pc];
        let start_pc = pc;
        pc += 1;

        match opcode {
            LOAD_I64 => {
                if pc >= bytecode.len() {
                    break;
                }
                let reg = bytecode[pc];
                pc += 1;
                if pc + 7 >= bytecode.len() {
                    break;
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
                println!("{} LOAD_I64 r{}, {}", start_pc, reg, value);
            }
            LOAD_F64 => {
                if pc >= bytecode.len() {
                    break;
                }
                let reg = bytecode[pc];
                pc += 1;
                if pc + 7 >= bytecode.len() {
                    break;
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
                println!("{} LOAD_F64 r{}, {}", start_pc, reg, value);
            }
            ADD_I64 => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                println!("{} ADD_I64 r{}, r{}, r{}", start_pc, r1, r2, dst);
            }
            SUB_I64 => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                println!("{} SUB_I64 r{}, r{}, r{}", start_pc, r1, r2, dst);
            }
            MUL_I64 => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                println!("{} MUL_I64 r{}, r{}, r{}", start_pc, r1, r2, dst);
            }
            GT_I64 => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                println!("{} GT_I64 r{}, r{}, r{}", start_pc, r1, r2, dst);
            }
            ADD_F64 => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                println!("{} ADD_F64 r{}, r{}, r{}", start_pc, r1, r2, dst);
            }
            SUB_F64 => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                println!("{} SUB_F64 r{}, r{}, r{}", start_pc, r1, r2, dst);
            }
            MUL_F64 => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                println!("{} MUL_F64 r{}, r{}, r{}", start_pc, r1, r2, dst);
            }
            GT_F64 => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let r1 = bytecode[pc];
                let r2 = bytecode[pc + 1];
                let dst = bytecode[pc + 2];
                pc += 3;
                println!("{} GT_F64 r{}, r{}, r{}", start_pc, r1, r2, dst);
            }
            JUMP_FORWARD_IF_FALSE => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let cond_reg = bytecode[pc];
                pc += 1;
                let offset = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                let target = pc + offset as usize;
                pc += 2;
                println!(
                    "{} JUMP_FORWARD_IF_FALSE r{}, {} (offset: {})",
                    start_pc, cond_reg, target, offset
                );
            }
            JUMP_FORWARD_IF_TRUE => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let cond_reg = bytecode[pc];
                pc += 1;
                let offset = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                let target = pc + offset as usize;
                pc += 2;
                println!(
                    "{} JUMP_FORWARD_IF_TRUE r{}, {} (offset: {})",
                    start_pc, cond_reg, target, offset
                );
            }
            JUMP_BACKWARD_IF_FALSE => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let cond_reg = bytecode[pc];
                pc += 1;
                let offset = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                pc += 2;
                let target = pc as i64 - offset as i64;
                println!(
                    "{} JUMP_BACKWARD_IF_FALSE r{}, {} (offset: {})",
                    start_pc, cond_reg, target, offset
                );
            }
            JUMP_BACKWARD_IF_TRUE => {
                if pc + 2 >= bytecode.len() {
                    break;
                }
                let cond_reg = bytecode[pc];
                pc += 1;
                let offset = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                pc += 2;
                let target = pc as i64 - offset as i64;
                println!(
                    "{} JUMP_BACKWARD_IF_TRUE r{}, {} (offset: {})",
                    start_pc, cond_reg, target, offset
                );
            }
            JMP => {
                if pc + 1 >= bytecode.len() {
                    break;
                }
                let target = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                pc += 2;
                println!("{} JMP {}", start_pc, target);
            }
            I64_TO_F64 => {
                if pc + 1 >= bytecode.len() {
                    break;
                }
                let src = bytecode[pc];
                let dst = bytecode[pc + 1];
                pc += 2;
                println!("{} I64_TO_F64 r{}, r{}", start_pc, src, dst);
            }
            F64_TO_I64 => {
                if pc + 1 >= bytecode.len() {
                    break;
                }
                let src = bytecode[pc];
                let dst = bytecode[pc + 1];
                pc += 2;
                println!("{} F64_TO_I64 r{}, r{}", start_pc, src, dst);
            }
            _ => {
                println!("{} UNKNOWN_OPCODE 0x{:02X}", start_pc, opcode);
                pc += 1;
            }
        }
    }
    println!("pc={}", pc);
    println!("bytecode.len()={}", bytecode.len());
}
