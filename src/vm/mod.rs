mod tests;

use std::fmt;

// Instruction opcodes
pub const LOAD_I64: u8 = 0x01;
pub const LOAD_F64: u8 = 0x02;
pub const ADD_I64: u8 = 0x03;
pub const SUB_I64: u8 = 0x04;
pub const MUL_I64: u8 = 0x05;
pub const GT_I64: u8 = 0x06;
pub const ADD_F64: u8 = 0x07;
pub const SUB_F64: u8 = 0x08;
pub const MUL_F64: u8 = 0x09;
pub const GT_F64: u8 = 0x0A;
pub const JUMP_FORWARD_IF_FALSE: u8 = 0x0B;
pub const JMP: u8 = 0x0C;
pub const I64_TO_F64: u8 = 0x0D;
pub const F64_TO_I64: u8 = 0x0E;

#[derive(Debug)]
pub enum VmError {
    InvalidOpcode(u8),
    InvalidJumpTarget(u16),
    UnexpectedEndOfProgram,
    // InvalidRegister(u8),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: 0x{:02X}", opcode),
            VmError::InvalidJumpTarget(target) => write!(f, "Invalid jump target: {}", target),
            VmError::UnexpectedEndOfProgram => write!(f, "Unexpected end of program"),
            // VmError::InvalidRegister(reg) => write!(f, "Invalid register: {}", reg),
        }
    }
}

impl std::error::Error for VmError {}

pub struct VirtualMachine {
    pub registers: [u64; 256],
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            registers: [0u64; 256],
        }
    }

    /// Interpret register value as i64
    fn get_i64(&self, reg: u8) -> i64 {
        self.registers[reg as usize] as i64
    }

    /// Interpret register value as f64
    fn get_f64(&self, reg: u8) -> f64 {
        f64::from_bits(self.registers[reg as usize])
    }

    /// Store i64 value in register
    fn set_i64(&mut self, reg: u8, value: i64) {
        self.registers[reg as usize] = value as u64;
    }

    /// Store f64 value in register
    fn set_f64(&mut self, reg: u8, value: f64) {
        self.registers[reg as usize] = value.to_bits();
    }

    /// Read a u16 from bytecode at given position (little-endian)
    fn read_u16(&self, bytecode: &[u8], pos: usize) -> Result<u16, VmError> {
        if pos + 1 >= bytecode.len() {
            return Err(VmError::UnexpectedEndOfProgram);
        }
        Ok(u16::from_le_bytes([bytecode[pos], bytecode[pos + 1]]))
    }

    /// Read an i64 from bytecode at given position (little-endian)
    fn read_i64(&self, bytecode: &[u8], pos: usize) -> Result<i64, VmError> {
        if pos + 7 >= bytecode.len() {
            return Err(VmError::UnexpectedEndOfProgram);
        }
        let bytes = [
            bytecode[pos],
            bytecode[pos + 1],
            bytecode[pos + 2],
            bytecode[pos + 3],
            bytecode[pos + 4],
            bytecode[pos + 5],
            bytecode[pos + 6],
            bytecode[pos + 7],
        ];
        Ok(i64::from_le_bytes(bytes))
    }

    /// Read an f64 from bytecode at given position (little-endian)
    fn read_f64(&self, bytecode: &[u8], pos: usize) -> Result<f64, VmError> {
        if pos + 7 >= bytecode.len() {
            return Err(VmError::UnexpectedEndOfProgram);
        }
        let bytes = [
            bytecode[pos],
            bytecode[pos + 1],
            bytecode[pos + 2],
            bytecode[pos + 3],
            bytecode[pos + 4],
            bytecode[pos + 5],
            bytecode[pos + 6],
            bytecode[pos + 7],
        ];
        Ok(f64::from_le_bytes(bytes))
    }

    /// Execute a single instruction
    fn execute_instruction(&mut self, bytecode: &[u8], pc: &mut usize) -> Result<(), VmError> {
        if *pc >= bytecode.len() {
            return Err(VmError::UnexpectedEndOfProgram);
        }

        let opcode = bytecode[*pc];
        *pc += 1;

        match opcode {
            LOAD_I64 => {
                // Format: [opcode, reg, i64_value[8]]
                if *pc >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let reg = bytecode[*pc];
                *pc += 1;
                let value = self.read_i64(bytecode, *pc)?;
                *pc += 8;
                self.set_i64(reg, value);
            }
            LOAD_F64 => {
                // Format: [opcode, reg, f64_value[8]]
                if *pc >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let reg = bytecode[*pc];
                *pc += 1;
                let value = self.read_f64(bytecode, *pc)?;
                *pc += 8;
                self.set_f64(reg, value);
            }
            ADD_I64 => {
                // Format: [opcode, r1, r2, dst]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_i64(r1);
                let val2 = self.get_i64(r2);
                self.set_i64(dst, val1.wrapping_add(val2));
            }
            SUB_I64 => {
                // Format: [opcode, r1, r2, dst]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_i64(r1);
                let val2 = self.get_i64(r2);
                self.set_i64(dst, val1.wrapping_sub(val2));
            }
            MUL_I64 => {
                // Format: [opcode, r1, r2, dst]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_i64(r1);
                let val2 = self.get_i64(r2);
                self.set_i64(dst, val1.wrapping_mul(val2));
            }
            GT_I64 => {
                // Format: [opcode, r1, r2, dst]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_i64(r1);
                let val2 = self.get_i64(r2);
                self.set_i64(dst, if val1 > val2 { 1 } else { 0 });
            }
            ADD_F64 => {
                // Format: [opcode, r1, r2, dst]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_f64(r1);
                let val2 = self.get_f64(r2);
                self.set_f64(dst, val1 + val2);
            }
            SUB_F64 => {
                // Format: [opcode, r1, r2, dst]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_f64(r1);
                let val2 = self.get_f64(r2);
                self.set_f64(dst, val1 - val2);
            }
            MUL_F64 => {
                // Format: [opcode, r1, r2, dst]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_f64(r1);
                let val2 = self.get_f64(r2);
                self.set_f64(dst, val1 * val2);
            }
            GT_F64 => {
                // Format: [opcode, r1, r2, dst]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_f64(r1);
                let val2 = self.get_f64(r2);
                self.set_i64(dst, if val1 > val2 { 1 } else { 0 });
            }
            JUMP_FORWARD_IF_FALSE => {
                // Format: [opcode, cond_reg, target[2]]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let cond_reg = bytecode[*pc];
                *pc += 1;
                let target = *pc + self.read_u16(bytecode, *pc)? as usize;
                *pc += 2;

                if target > bytecode.len() {
                    return Err(VmError::InvalidJumpTarget(target as u16));
                }

                if self.registers[cond_reg as usize] == 0 {
                    *pc = target;
                }
            }
            JMP => {
                // Format: [opcode, target[2]]
                if *pc + 1 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let target = self.read_u16(bytecode, *pc)? as usize;
                *pc += 2;

                if target > bytecode.len() {
                    return Err(VmError::InvalidJumpTarget(target as u16));
                }

                *pc = target;
            }
            I64_TO_F64 => {
                // Format: [opcode, src, dst]
                if *pc + 1 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let src = bytecode[*pc];
                let dst = bytecode[*pc + 1];
                *pc += 2;
                let i64_val = self.get_i64(src);
                self.set_f64(dst, i64_val as f64);
            }
            F64_TO_I64 => {
                // Format: [opcode, src, dst]
                if *pc + 1 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let src = bytecode[*pc];
                let dst = bytecode[*pc + 1];
                *pc += 2;
                let f64_val = self.get_f64(src);
                self.set_i64(dst, f64_val as i64);
            }
            _ => {
                return Err(VmError::InvalidOpcode(opcode));
            }
        }

        Ok(())
    }

    /// Execute a program from bytecode
    pub fn eval_program(&mut self, bytecode: &[u8]) -> Result<(), VmError> {
        let mut pc = 0usize;

        while pc < bytecode.len() {
            self.execute_instruction(bytecode, &mut pc)?;
        }

        Ok(())
    }

    /// Get register value as i64
    pub fn get_register_i64(&self, reg: u8) -> i64 {
        self.get_i64(reg)
    }

    /// Get register value as f64
    pub fn get_register_f64(&self, reg: u8) -> f64 {
        self.get_f64(reg)
    }

    /// Get raw register value
    pub fn get_register_raw(&self, reg: u8) -> u64 {
        self.registers[reg as usize]
    }

    /// Set register value as i64
    pub fn set_register_i64(&mut self, reg: u8, value: i64) {
        self.set_i64(reg, value);
    }

    /// Set register value as f64
    pub fn set_register_f64(&mut self, reg: u8, value: f64) {
        self.set_f64(reg, value);
    }

    /// Set raw register value
    pub fn set_register_raw(&mut self, reg: u8, value: u64) {
        self.registers[reg as usize] = value;
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for building bytecode
pub struct BytecodeBuilder {
    bytecode: Vec<u8>,
}

impl BytecodeBuilder {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
        }
    }

    pub fn load_i64(&mut self, value: i64, reg: u8) {
        self.bytecode.push(LOAD_I64);
        self.bytecode.push(reg);
        self.bytecode.extend_from_slice(&value.to_le_bytes());
    }

    pub fn load_f64(&mut self, value: f64, reg: u8) {
        self.bytecode.push(LOAD_F64);
        self.bytecode.push(reg);
        self.bytecode.extend_from_slice(&value.to_le_bytes());
    }

    pub fn add_i64(&mut self, r1: u8, r2: u8, dst: u8) {
        self.bytecode.push(ADD_I64);
        self.bytecode.push(r1);
        self.bytecode.push(r2);
        self.bytecode.push(dst);
    }

    pub fn sub_i64(&mut self, r1: u8, r2: u8, dst: u8) {
        self.bytecode.push(SUB_I64);
        self.bytecode.push(r1);
        self.bytecode.push(r2);
        self.bytecode.push(dst);
    }

    pub fn mul_i64(&mut self, r1: u8, r2: u8, dst: u8) {
        self.bytecode.push(MUL_I64);
        self.bytecode.push(r1);
        self.bytecode.push(r2);
        self.bytecode.push(dst);
    }

    pub fn gt_i64(&mut self, r1: u8, r2: u8, dst: u8) {
        self.bytecode.push(GT_I64);
        self.bytecode.push(r1);
        self.bytecode.push(r2);
        self.bytecode.push(dst);
    }

    pub fn add_f64(&mut self, r1: u8, r2: u8, dst: u8) {
        self.bytecode.push(ADD_F64);
        self.bytecode.push(r1);
        self.bytecode.push(r2);
        self.bytecode.push(dst);
    }

    pub fn sub_f64(&mut self, r1: u8, r2: u8, dst: u8) {
        self.bytecode.push(SUB_F64);
        self.bytecode.push(r1);
        self.bytecode.push(r2);
        self.bytecode.push(dst);
    }

    pub fn mul_f64(&mut self, r1: u8, r2: u8, dst: u8) {
        self.bytecode.push(MUL_F64);
        self.bytecode.push(r1);
        self.bytecode.push(r2);
        self.bytecode.push(dst);
    }

    pub fn gt_f64(&mut self, r1: u8, r2: u8, dst: u8) {
        self.bytecode.push(GT_F64);
        self.bytecode.push(r1);
        self.bytecode.push(r2);
        self.bytecode.push(dst);
    }

    pub fn jump_forward_if_false(&mut self, cond_reg: u8) -> u16 {
        self.bytecode.push(JUMP_FORWARD_IF_FALSE);
        self.bytecode.push(cond_reg);
        let target_bytes_pos = self.bytecode.len() as u16;
        self.bytecode.extend_from_slice(&0u16.to_le_bytes()); // Put zeros for target
        target_bytes_pos
    }

    pub fn jmp(&mut self, target: u16) -> u16 {
        self.bytecode.push(JMP);
        let target_bytes_pos = self.bytecode.len() as u16;
        self.bytecode.extend_from_slice(&target.to_le_bytes());
        target_bytes_pos
    }

    pub fn i64_to_f64(&mut self, src: u8, dst: u8) {
        self.bytecode.push(I64_TO_F64);
        self.bytecode.push(src);
        self.bytecode.push(dst);
    }

    pub fn f64_to_i64(&mut self, src: u8, dst: u8) {
        self.bytecode.push(F64_TO_I64);
        self.bytecode.push(src);
        self.bytecode.push(dst);
    }

    pub fn current_pos(&self) -> u16 {
        self.bytecode.len() as u16
    }

    pub fn build(self) -> Vec<u8> {
        self.bytecode
    }

    /// Patch a target address at the given position
    pub fn patch_target(&mut self, target_pos: u16, target_value: u16) {
        let pos = target_pos as usize;
        let bytes = target_value.to_le_bytes();
        self.bytecode[pos] = bytes[0];
        self.bytecode[pos + 1] = bytes[1];
    }
}

impl Default for BytecodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

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
                    "{} JUMP_FORWARD_IF_FALSE r{}, {}",
                    start_pc, cond_reg, target
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
