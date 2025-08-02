mod bytecode_builder;
mod const_pool;
mod print_bytecode;
mod tests;
mod tests_bytecode_builder;
mod tests_const_pool;
mod tests_print_bytecode;
mod tests_const_opcodes;

pub use bytecode_builder::BytecodeBuilder;
pub use print_bytecode::print_bytecode;

use std::fmt;
use std::time::{Duration, Instant};
use const_pool::ConstPool;

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
pub const JUMP_BACKWARD_IF_FALSE: u8 = 0x0F;
pub const JUMP_BACKWARD_IF_TRUE: u8 = 0x10;
pub const JUMP_FORWARD_IF_TRUE: u8 = 0x11;
pub const GTE_I64: u8 = 0x12;
pub const LT_I64: u8 = 0x13;
pub const LTE_I64: u8 = 0x14;
pub const GTE_F64: u8 = 0x15;
pub const LT_F64: u8 = 0x16;
pub const LTE_F64: u8 = 0x17;
pub const LOAD_CONST_VALUE: u8 = 0x18;
pub const LOAD_CONST_SLICE: u8 = 0x19;

#[derive(Debug)]
pub enum VmError {
    InvalidOpcode(u8),
    InvalidJumpTarget(u16),
    InvalidConstIndex(u16),
    UnexpectedEndOfProgram,
    Timeout(Duration),
    // InvalidRegister(u8),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: 0x{:02X}", opcode),
            VmError::InvalidJumpTarget(target) => write!(f, "Invalid jump target: {}", target),
            VmError::InvalidConstIndex(index) => {
                write!(f, "Invalid constant index: {}", index)
            }
            VmError::UnexpectedEndOfProgram => write!(f, "Unexpected end of program"),
            VmError::Timeout(duration) => write!(f, "Execution timeout after {:?}", duration),
            // VmError::InvalidRegister(reg) => write!(f, "Invalid register: {}", reg),
        }
    }
}

impl std::error::Error for VmError {}

pub struct VirtualMachine {
    pub registers: [u64; 256],
    pub const_values: Vec<u64>,
    pub const_slices: Vec<&'static [u8]>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            registers: [0u64; 256],
            const_values: Vec::new(),
            const_slices: Vec::new(),
        }
    }

    pub fn set_const_pool(&mut self, pool: &ConstPool) {
        self.const_values = pool.values.clone();
        self.const_slices = pool.slices.clone();
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
            GTE_I64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_i64(r1);
                let val2 = self.get_i64(r2);
                self.set_i64(dst, if val1 >= val2 { 1 } else { 0 });
            }
            LT_I64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_i64(r1);
                let val2 = self.get_i64(r2);
                self.set_i64(dst, if val1 < val2 { 1 } else { 0 });
            }
            LTE_I64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_i64(r1);
                let val2 = self.get_i64(r2);
                self.set_i64(dst, if val1 <= val2 { 1 } else { 0 });
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
            GTE_F64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_f64(r1);
                let val2 = self.get_f64(r2);
                self.set_i64(dst, if val1 >= val2 { 1 } else { 0 });
            }
            LT_F64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_f64(r1);
                let val2 = self.get_f64(r2);
                self.set_i64(dst, if val1 < val2 { 1 } else { 0 });
            }
            LTE_F64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc];
                let r2 = bytecode[*pc + 1];
                let dst = bytecode[*pc + 2];
                *pc += 3;
                let val1 = self.get_f64(r1);
                let val2 = self.get_f64(r2);
                self.set_i64(dst, if val1 <= val2 { 1 } else { 0 });
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
            JUMP_FORWARD_IF_TRUE => {
                // Format: [opcode, cond_reg, offset[2]]
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

                if self.registers[cond_reg as usize] != 0 {
                    *pc = target;
                }
            }
            JUMP_BACKWARD_IF_FALSE => {
                // Format: [opcode, cond_reg, offset[2]]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let cond_reg = bytecode[*pc];
                *pc += 1;
                let offset = self.read_u16(bytecode, *pc)? as usize;
                *pc += 2;

                if offset > *pc {
                    return Err(VmError::InvalidJumpTarget((*pc - offset) as u16));
                }

                if self.registers[cond_reg as usize] == 0 {
                    *pc -= offset;
                }
            }
            JUMP_BACKWARD_IF_TRUE => {
                // Format: [opcode, cond_reg, offset[2]]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let cond_reg = bytecode[*pc];
                *pc += 1;
                let offset = self.read_u16(bytecode, *pc)? as usize;
                *pc += 2;

                if offset > *pc {
                    return Err(VmError::InvalidJumpTarget(
                        (*pc as i64 - offset as i64) as u16,
                    ));
                }

                if self.registers[cond_reg as usize] != 0 {
                    *pc -= offset;
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
            LOAD_CONST_VALUE => {
                // Format: [opcode, dst, index[2]]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let dst = bytecode[*pc];
                let index = self.read_u16(bytecode, *pc + 1)? as usize;
                *pc += 3;
                let value = self
                    .const_values
                    .get(index)
                    .ok_or(VmError::InvalidConstIndex(index as u16))?;
                self.registers[dst as usize] = *value;
            }
            LOAD_CONST_SLICE => {
                // Format: [opcode, dst, index[2]]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let dst = bytecode[*pc] as usize;
                let index = self.read_u16(bytecode, *pc + 1)? as usize;
                *pc += 3;
                let slice = self
                    .const_slices
                    .get(index)
                    .ok_or(VmError::InvalidConstIndex(index as u16))?;
                let ptr = slice.as_ptr() as u64;
                let len = slice.len() as u64;
                self.registers[dst] = ptr;
                if dst + 1 < self.registers.len() {
                    self.registers[dst + 1] = len;
                }
            }
            _ => {
                return Err(VmError::InvalidOpcode(opcode));
            }
        }

        Ok(())
    }

    /// Execute a program from bytecode without timeout
    pub fn eval_program(&mut self, bytecode: &[u8]) -> Result<(), VmError> {
        self.eval_program_with_timeout(bytecode, None)
    }

    /// Execute a program from bytecode with optional timeout
    pub fn eval_program_with_timeout(
        &mut self,
        bytecode: &[u8],
        timeout: Option<Duration>,
    ) -> Result<(), VmError> {
        let mut pc = 0usize;
        let start_time = Instant::now();
        let mut instruction_count = 0u64;

        // Check timeout every N instructions to balance performance and responsiveness
        const TIMEOUT_CHECK_INTERVAL: u64 = 1000;

        while pc < bytecode.len() {
            self.execute_instruction(bytecode, &mut pc)?;

            instruction_count += 1;

            // Periodically check for timeout to avoid overhead on every instruction
            if let Some(timeout_duration) = timeout {
                if instruction_count % TIMEOUT_CHECK_INTERVAL == 0 {
                    let elapsed = start_time.elapsed();
                    if elapsed > timeout_duration {
                        return Err(VmError::Timeout(elapsed));
                    }
                }
            }
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
