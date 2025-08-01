mod tests;

use std::fmt;
use std::time::{Duration, Instant};

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

#[derive(Debug)]
pub enum VmError {
    InvalidOpcode(u8),
    InvalidJumpTarget(u16),
    UnexpectedEndOfProgram,
    Timeout(Duration),
    // InvalidRegister(u8),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: 0x{:02X}", opcode),
            VmError::InvalidJumpTarget(target) => write!(f, "Invalid jump target: {}", target),
            VmError::UnexpectedEndOfProgram => write!(f, "Unexpected end of program"),
            VmError::Timeout(duration) => write!(f, "Execution timeout after {:?}", duration),
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

#[derive(Debug)]
struct PendingJump {
    label_id: u32,
    patch_position: u16,
    jump_type: JumpType,
}

#[derive(Debug)]
enum JumpType {
    Absolute,
    ForwardIfFalse,
    ForwardIfTrue,
}

/// Enhanced bytecode builder with label support and target-based jumps
pub struct BytecodeBuilder {
    bytecode: Vec<u8>,
    labels: std::collections::HashMap<u32, u16>,
    next_label_id: u32,
    pending_jumps: Vec<PendingJump>,
}

impl BytecodeBuilder {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            labels: std::collections::HashMap::new(),
            next_label_id: 0,
            pending_jumps: Vec::new(),
        }
    }

    pub fn current_pos(&self) -> u16 {
        self.bytecode.len() as u16
    }

    // === LABEL MANAGEMENT ===

    /// Create a new label and return its ID
    pub fn create_label(&mut self) -> u32 {
        let label_id = self.next_label_id;
        self.next_label_id += 1;
        label_id
    }

    /// Place a label at the current position
    pub fn place_label(&mut self, label_id: u32) {
        let pos = self.current_pos();
        self.labels.insert(label_id, pos);
    }

    // === BASIC INSTRUCTIONS ===

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

    // === ORIGINAL JUMP METHODS (for backward compatibility) ===

    pub fn jump_forward_if_false(&mut self, cond_reg: u8) -> u16 {
        self.bytecode.push(JUMP_FORWARD_IF_FALSE);
        self.bytecode.push(cond_reg);
        let target_bytes_pos = self.bytecode.len() as u16;
        self.bytecode.extend_from_slice(&0u16.to_le_bytes()); // Put zeros for target
        target_bytes_pos
    }

    pub fn jump_forward_if_true(&mut self, cond_reg: u8) -> u16 {
        self.bytecode.push(JUMP_FORWARD_IF_TRUE);
        self.bytecode.push(cond_reg);
        let target_bytes_pos = self.bytecode.len() as u16;
        self.bytecode.extend_from_slice(&0u16.to_le_bytes()); // Put zeros for target
        target_bytes_pos
    }

    pub fn jump_backward_if_false(&mut self, cond_reg: u8, offset: u16) {
        self.bytecode.push(JUMP_BACKWARD_IF_FALSE);
        self.bytecode.push(cond_reg);
        self.bytecode.extend_from_slice(&offset.to_le_bytes());
    }

    pub fn jump_backward_if_true(&mut self, cond_reg: u8, offset: u16) {
        self.bytecode.push(JUMP_BACKWARD_IF_TRUE);
        self.bytecode.push(cond_reg);
        self.bytecode.extend_from_slice(&offset.to_le_bytes());
    }

    pub fn jmp(&mut self, target: u16) -> u16 {
        self.bytecode.push(JMP);
        let target_bytes_pos = self.bytecode.len() as u16;
        self.bytecode.extend_from_slice(&target.to_le_bytes());
        target_bytes_pos
    }

    /// Patch a target address at the given position
    pub fn patch_target(&mut self, target_pos: u16, target_value: u16) {
        let pos = target_pos as usize;
        let bytes = target_value.to_le_bytes();
        self.bytecode[pos] = bytes[0];
        self.bytecode[pos + 1] = bytes[1];
    }

    // === TARGET-BASED JUMP METHODS ===

    /// Jump to a specific target position if condition is false
    pub fn jump_forward_if_false_to(&mut self, cond_reg: u8, target: u16) {
        let current_pos = self.current_pos();
        if target <= current_pos {
            panic!("Forward jump target must be after current position");
        }
        let offset = target - current_pos - 4; // -4 for instruction size
        self.bytecode.push(JUMP_FORWARD_IF_FALSE);
        self.bytecode.push(cond_reg);
        self.bytecode.extend_from_slice(&offset.to_le_bytes());
    }

    /// Jump to a specific target position if condition is true
    pub fn jump_forward_if_true_to(&mut self, cond_reg: u8, target: u16) {
        let current_pos = self.current_pos();
        if target <= current_pos {
            panic!("Forward jump target must be after current position");
        }
        let offset = target - current_pos - 4; // -4 for instruction size
        self.bytecode.push(JUMP_FORWARD_IF_TRUE);
        self.bytecode.push(cond_reg);
        self.bytecode.extend_from_slice(&offset.to_le_bytes());
    }

    /// Jump to a specific target position (backwards) if condition is false
    pub fn jump_backward_if_false_to(&mut self, cond_reg: u8, target: u16) {
        let current_pos = self.current_pos();
        if target >= current_pos {
            panic!("Backward jump target must be before current position");
        }
        let offset = current_pos - target + 4; // +4 for instruction size
        self.jump_backward_if_false(cond_reg, offset);
    }

    /// Jump to a specific target position (backwards) if condition is true
    pub fn jump_backward_if_true_to(&mut self, cond_reg: u8, target: u16) {
        let current_pos = self.current_pos();
        if target >= current_pos {
            panic!("Backward jump target must be before current position");
        }
        let offset = current_pos - target + 4; // +4 for instruction size
        self.jump_backward_if_true(cond_reg, offset);
    }

    /// Unconditional jump to a specific target position
    pub fn jmp_to(&mut self, target: u16) {
        self.bytecode.push(JMP);
        self.bytecode.extend_from_slice(&target.to_le_bytes());
    }

    // === LABEL-BASED JUMP METHODS ===

    /// Jump to a label if condition is false
    pub fn jump_if_false_to_label(&mut self, cond_reg: u8, label_id: u32) {
        if let Some(&target) = self.labels.get(&label_id) {
            let current_pos = self.current_pos();
            if target > current_pos {
                self.jump_forward_if_false_to(cond_reg, target);
            } else {
                self.jump_backward_if_false_to(cond_reg, target);
            }
        } else {
            // Store for later resolution
            let patch_pos = self.jump_forward_if_false(cond_reg);
            self.pending_jumps.push(PendingJump {
                label_id,
                patch_position: patch_pos,
                jump_type: JumpType::ForwardIfFalse,
            });
        }
    }

    /// Jump to a label if condition is true
    pub fn jump_if_true_to_label(&mut self, cond_reg: u8, label_id: u32) {
        if let Some(&target) = self.labels.get(&label_id) {
            let current_pos = self.current_pos();
            if target > current_pos {
                self.jump_forward_if_true_to(cond_reg, target);
            } else {
                self.jump_backward_if_true_to(cond_reg, target);
            }
        } else {
            // Store for later resolution
            let patch_pos = self.jump_forward_if_true(cond_reg);
            self.pending_jumps.push(PendingJump {
                label_id,
                patch_position: patch_pos,
                jump_type: JumpType::ForwardIfTrue,
            });
        }
    }

    /// Unconditional jump to a label
    pub fn jmp_to_label(&mut self, label_id: u32) {
        if let Some(&target) = self.labels.get(&label_id) {
            self.jmp_to(target);
        } else {
            // Store for later resolution
            let patch_pos = self.jmp(0);
            self.pending_jumps.push(PendingJump {
                label_id,
                patch_position: patch_pos,
                jump_type: JumpType::Absolute,
            });
        }
    }

    // === BUILD METHOD ===

    /// Build the final bytecode, resolving all pending jumps
    pub fn build(&mut self) -> Vec<u8> {
        // Resolve all pending jumps
        for pending in std::mem::take(&mut self.pending_jumps) {
            if let Some(&target) = self.labels.get(&pending.label_id) {
                match pending.jump_type {
                    JumpType::Absolute => {
                        self.patch_target(pending.patch_position, target);
                    }
                    JumpType::ForwardIfFalse | JumpType::ForwardIfTrue => {
                        let offset = target - pending.patch_position;
                        self.patch_target(pending.patch_position, offset);
                    }
                }
            } else {
                panic!("Unresolved label: {}", pending.label_id);
            }
        }

        self.bytecode.clone()
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
