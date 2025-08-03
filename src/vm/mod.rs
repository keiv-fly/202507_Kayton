mod bytecode_builder;
mod const_pool;
mod print_bytecode;
mod registers;
mod call;
mod tests;
mod tests_bytecode_builder;
mod tests_const_opcodes;
mod tests_const_pool;
mod tests_print_bytecode;
mod tests_registers;
mod tests_call;

pub use bytecode_builder::BytecodeBuilder;
pub use print_bytecode::print_bytecode;
pub use registers::Registers;
pub use call::{HostFn, HostFunctionMetadata, HostFunctionRegistry, CallInfo};

use const_pool::ConstPool;
use std::fmt;
use std::time::{Duration, Instant};

// Instruction opcodes
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
pub const CALL_HOST: u8 = 0x1A;

#[derive(Debug)]
pub enum VmError {
    InvalidOpcode(u8),
    InvalidJumpTarget(usize),
    InvalidConstIndex(usize),
    UnexpectedEndOfProgram,
    Timeout(Duration),
    HostError(String),
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
            VmError::HostError(err) => write!(f, "Host error: {}", err),
            // VmError::InvalidRegister(reg) => write!(f, "Invalid register: {}", reg),
        }
    }
}

impl std::error::Error for VmError {}

pub struct VirtualMachine {
    pub registers: Registers,
    pub const_pool: ConstPool,
    pub host_functions: HostFunctionRegistry,
    pub call_stack: Vec<CallInfo>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            const_pool: ConstPool::new(),
            host_functions: HostFunctionRegistry::new(),
            call_stack: vec![CallInfo::Global { base: 0, top: 0 }],
        }
    }

    /// Interpret register value as i64
    fn get_i64(&self, reg: usize) -> i64 {
        self.registers.get(reg) as i64
    }

    /// Interpret register value as f64
    fn get_f64(&self, reg: usize) -> f64 {
        f64::from_bits(self.registers.get(reg))
    }

    /// Store i64 value in register
    fn set_i64(&mut self, reg: usize, value: i64) {
        self.registers.set(reg, value as u64);
    }

    /// Store f64 value in register
    fn set_f64(&mut self, reg: usize, value: f64) {
        self.registers.set(reg, value.to_bits());
    }

    /// Read a u16 from bytecode at given position (little-endian)
    fn read_u16(&self, bytecode: &[u8], pos: usize) -> Result<u16, VmError> {
        if pos + 1 >= bytecode.len() {
            return Err(VmError::UnexpectedEndOfProgram);
        }
        Ok(u16::from_le_bytes([bytecode[pos], bytecode[pos + 1]]))
    }

    /// Execute a single instruction
    fn execute_instruction(&mut self, bytecode: &[u8], pc: &mut usize) -> Result<(), VmError> {
        if *pc >= bytecode.len() {
            return Err(VmError::UnexpectedEndOfProgram);
        }

        let opcode = bytecode[*pc];
        *pc += 1;

        match opcode {
            ADD_I64 => {
                // Format: [opcode, r1, r2, dst]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
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
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
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
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
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
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
                *pc += 3;
                let val1 = self.get_i64(r1);
                let val2 = self.get_i64(r2);
                self.set_i64(dst, if val1 > val2 { 1 } else { 0 });
            }
            GTE_I64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
                *pc += 3;
                let val1 = self.get_i64(r1);
                let val2 = self.get_i64(r2);
                self.set_i64(dst, if val1 >= val2 { 1 } else { 0 });
            }
            LT_I64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
                *pc += 3;
                let val1 = self.get_i64(r1);
                let val2 = self.get_i64(r2);
                self.set_i64(dst, if val1 < val2 { 1 } else { 0 });
            }
            LTE_I64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
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
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
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
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
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
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
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
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
                *pc += 3;
                let val1 = self.get_f64(r1);
                let val2 = self.get_f64(r2);
                self.set_i64(dst, if val1 > val2 { 1 } else { 0 });
            }
            GTE_F64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
                *pc += 3;
                let val1 = self.get_f64(r1);
                let val2 = self.get_f64(r2);
                self.set_i64(dst, if val1 >= val2 { 1 } else { 0 });
            }
            LT_F64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
                *pc += 3;
                let val1 = self.get_f64(r1);
                let val2 = self.get_f64(r2);
                self.set_i64(dst, if val1 < val2 { 1 } else { 0 });
            }
            LTE_F64 => {
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let r1 = bytecode[*pc] as usize;
                let r2 = bytecode[*pc + 1] as usize;
                let dst = bytecode[*pc + 2] as usize;
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
                let cond_reg = bytecode[*pc] as usize;
                *pc += 1;
                let target = *pc + self.read_u16(bytecode, *pc)? as usize;
                *pc += 2;

                if target > bytecode.len() {
                    return Err(VmError::InvalidJumpTarget(target));
                }

                if self.registers.get(cond_reg) == 0 {
                    *pc = target;
                }
            }
            JUMP_FORWARD_IF_TRUE => {
                // Format: [opcode, cond_reg, offset[2]]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let cond_reg = bytecode[*pc] as usize;
                *pc += 1;
                let target = *pc + self.read_u16(bytecode, *pc)? as usize;
                *pc += 2;

                if target > bytecode.len() {
                    return Err(VmError::InvalidJumpTarget(target));
                }

                if self.registers.get(cond_reg) != 0 {
                    *pc = target;
                }
            }
            JUMP_BACKWARD_IF_FALSE => {
                // Format: [opcode, cond_reg, offset[2]]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let cond_reg = bytecode[*pc] as usize;
                *pc += 1;
                let offset = self.read_u16(bytecode, *pc)? as usize;
                *pc += 2;

                if offset > *pc {
                    let invalid_target = (*pc as isize - offset as isize) as usize;
                    return Err(VmError::InvalidJumpTarget(invalid_target));
                }

                if self.registers.get(cond_reg) == 0 {
                    *pc -= offset;
                }
            }
            JUMP_BACKWARD_IF_TRUE => {
                // Format: [opcode, cond_reg, offset[2]]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let cond_reg = bytecode[*pc] as usize;
                *pc += 1;
                let offset = self.read_u16(bytecode, *pc)? as usize;
                *pc += 2;

                if offset > *pc {
                    let invalid_target = (*pc as isize - offset as isize) as usize;
                    return Err(VmError::InvalidJumpTarget(invalid_target));
                }

                if self.registers.get(cond_reg) != 0 {
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
                    return Err(VmError::InvalidJumpTarget(target));
                }

                *pc = target;
            }
            I64_TO_F64 => {
                // Format: [opcode, src, dst]
                if *pc + 1 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let src = bytecode[*pc] as usize;
                let dst = bytecode[*pc + 1] as usize;
                *pc += 2;
                let i64_val = self.get_i64(src);
                self.set_f64(dst, i64_val as f64);
            }
            F64_TO_I64 => {
                // Format: [opcode, src, dst]
                if *pc + 1 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let src = bytecode[*pc] as usize;
                let dst = bytecode[*pc + 1] as usize;
                *pc += 2;
                let f64_val = self.get_f64(src);
                self.set_i64(dst, f64_val as i64);
            }
            LOAD_CONST_VALUE => {
                // Format: [opcode, dst, index[2]]
                if *pc + 2 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let dst = bytecode[*pc] as usize;
                let index = self.read_u16(bytecode, *pc + 1)? as usize;
                *pc += 3;
                let value = self
                    .const_pool
                    .values
                    .get(index)
                    .ok_or(VmError::InvalidConstIndex(index))?;
                self.registers.set(dst, *value);
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
                    .const_pool
                    .slices
                    .get(index)
                    .ok_or(VmError::InvalidConstIndex(index))?;
                let ptr = slice.as_ptr() as u64;
                let len = slice.len() as u64;
                self.registers.set(dst, ptr);
                self.registers.set(dst + 1, len);
            }
            CALL_HOST => {
                if *pc + 1 >= bytecode.len() {
                    return Err(VmError::UnexpectedEndOfProgram);
                }
                let reg_index = self.read_u16(bytecode, *pc)? as usize;
                *pc += 2;
                let abs_index = self.current_base() + reg_index;
                let fn_index = self.registers.get(abs_index) as usize;
                let func = self
                    .host_functions
                    .funcs
                    .get(fn_index)
                    .ok_or(VmError::InvalidConstIndex(fn_index))?;
                let meta = self
                    .host_functions
                    .metadata
                    .get(fn_index)
                    .ok_or(VmError::InvalidConstIndex(fn_index))?;
                let base = abs_index;
                let top = base + meta.num_registers.saturating_sub(1);
                self.call_stack
                    .push(CallInfo::CallHost { base, top, host_fn_index: fn_index });
                self.registers.ensure_len(top + 1);
                let result = func(base, &mut self.registers);
                self.call_stack.pop();
                if let Err(err) = result {
                    return Err(VmError::HostError(err));
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
    pub fn get_register_i64(&self, reg: usize) -> i64 {
        self.get_i64(reg)
    }

    /// Get register value as f64
    pub fn get_register_f64(&self, reg: usize) -> f64 {
        self.get_f64(reg)
    }

    /// Get raw register value
    pub fn get_register_raw(&self, reg: usize) -> u64 {
        self.registers.get(reg)
    }

    /// Set register value as i64
    pub fn set_register_i64(&mut self, reg: usize, value: i64) {
        self.set_i64(reg, value);
    }

    /// Set register value as f64
    pub fn set_register_f64(&mut self, reg: usize, value: f64) {
        self.set_f64(reg, value);
    }

    /// Set raw register value
    pub fn set_register_raw(&mut self, reg: usize, value: u64) {
        self.registers.set(reg, value);
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}
