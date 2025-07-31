use std::fmt;

#[derive(Debug)]
pub enum Instr {
    LoadI64(i64, u64),    // LoadI64(value, dst)
    LoadF64(f64, u64),    // LoadF64(value, dst)
    Add(u64, u64, u64),   // Add(r1, r2, dst)
    Sub(u64, u64, u64),   // Sub(r1, r2, dst)
    Mul(u64, u64, u64),   // Mul(r1, r2, dst)
    Gt(u64, u64, u64),    // Gt(r1, r2, dst) → dst = 1 if r1 > r2 else 0
    JmpIfFalse(u64, u64), // If regs[cond] == 0 → pc = target
    Jmp(u64),             // Unconditional jump
    I64ToF64(u64, u64),   // Convert i64 to f64 (src, dst)
    F64ToI64(u64, u64),   // Convert f64 to i64 (src, dst)
}

#[derive(Debug)]
pub enum VmError {
    InvalidJumpTarget(u64),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::InvalidJumpTarget(target) => write!(f, "Invalid jump target: {}", target),
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
    fn get_i64(&self, reg: u64) -> i64 {
        self.registers[reg as usize] as i64
    }

    /// Interpret register value as f64
    fn get_f64(&self, reg: u64) -> f64 {
        f64::from_bits(self.registers[reg as usize])
    }

    /// Store i64 value in register
    fn set_i64(&mut self, reg: u64, value: i64) {
        self.registers[reg as usize] = value as u64;
    }

    /// Store f64 value in register
    fn set_f64(&mut self, reg: u64, value: f64) {
        self.registers[reg as usize] = value.to_bits();
    }

    /// Execute a single instruction
    fn execute_instruction(&mut self, instr: &Instr, pc: &mut u64) -> Result<(), VmError> {
        match instr {
            Instr::LoadI64(val, dst) => {
                self.set_i64(*dst, *val);
            }
            Instr::LoadF64(val, dst) => {
                self.set_f64(*dst, *val);
            }
            Instr::Add(r1, r2, dst) => {
                let val1_i64 = self.get_i64(*r1);
                let val2_i64 = self.get_i64(*r2);
                let val1_f64 = self.get_f64(*r1);
                let val2_f64 = self.get_f64(*r2);

                // Use f64 arithmetic if either value looks like a valid f64
                if (!val1_f64.is_nan() && val1_f64.fract() != 0.0)
                    || (!val2_f64.is_nan() && val2_f64.fract() != 0.0)
                {
                    self.set_f64(*dst, val1_f64 + val2_f64);
                } else {
                    self.set_i64(*dst, val1_i64.wrapping_add(val2_i64));
                }
            }
            Instr::Sub(r1, r2, dst) => {
                let val1_i64 = self.get_i64(*r1);
                let val2_i64 = self.get_i64(*r2);
                let val1_f64 = self.get_f64(*r1);
                let val2_f64 = self.get_f64(*r2);

                if (!val1_f64.is_nan() && val1_f64.fract() != 0.0)
                    || (!val2_f64.is_nan() && val2_f64.fract() != 0.0)
                {
                    self.set_f64(*dst, val1_f64 - val2_f64);
                } else {
                    self.set_i64(*dst, val1_i64.wrapping_sub(val2_i64));
                }
            }
            Instr::Mul(r1, r2, dst) => {
                let val1_i64 = self.get_i64(*r1);
                let val2_i64 = self.get_i64(*r2);
                let val1_f64 = self.get_f64(*r1);
                let val2_f64 = self.get_f64(*r2);

                if (!val1_f64.is_nan() && val1_f64.fract() != 0.0)
                    || (!val2_f64.is_nan() && val2_f64.fract() != 0.0)
                {
                    self.set_f64(*dst, val1_f64 * val2_f64);
                } else {
                    self.set_i64(*dst, val1_i64.wrapping_mul(val2_i64));
                }
            }
            Instr::Gt(r1, r2, dst) => {
                let val1_i64 = self.get_i64(*r1);
                let val2_i64 = self.get_i64(*r2);
                let val1_f64 = self.get_f64(*r1);
                let val2_f64 = self.get_f64(*r2);

                let result = if (!val1_f64.is_nan() && val1_f64.fract() != 0.0)
                    || (!val2_f64.is_nan() && val2_f64.fract() != 0.0)
                {
                    val1_f64 > val2_f64
                } else {
                    val1_i64 > val2_i64
                };
                self.set_i64(*dst, if result { 1 } else { 0 });
            }
            Instr::JmpIfFalse(cond, target) => {
                if self.registers[*cond as usize] == 0 {
                    *pc = *target;
                    return Ok(());
                }
            }
            Instr::Jmp(target) => {
                *pc = *target;
                return Ok(());
            }
            Instr::I64ToF64(src, dst) => {
                let i64_val = self.get_i64(*src);
                self.set_f64(*dst, i64_val as f64);
            }
            Instr::F64ToI64(src, dst) => {
                let f64_val = self.get_f64(*src);
                self.set_i64(*dst, f64_val as i64);
            }
        }
        *pc += 1;
        Ok(())
    }

    /// Execute a program
    pub fn eval_program(&mut self, bytecode: &[Instr]) -> Result<(), VmError> {
        let mut pc = 0u64;

        while (pc as usize) < bytecode.len() {
            // Validate jump targets
            if let Instr::Jmp(target) | Instr::JmpIfFalse(_, target) = &bytecode[pc as usize] {
                if *target != u64::MAX && (*target as usize) >= bytecode.len() {
                    return Err(VmError::InvalidJumpTarget(*target));
                }
            }

            self.execute_instruction(&bytecode[pc as usize], &mut pc)?;
        }

        Ok(())
    }

    /// Get register value as i64
    pub fn get_register_i64(&self, reg: u64) -> i64 {
        self.get_i64(reg)
    }

    /// Get register value as f64
    pub fn get_register_f64(&self, reg: u64) -> f64 {
        self.get_f64(reg)
    }

    /// Get raw register value
    pub fn get_register_raw(&self, reg: u64) -> u64 {
        self.registers[reg as usize]
    }

    /// Set register value as i64
    pub fn set_register_i64(&mut self, reg: u64, value: i64) {
        self.set_i64(reg, value);
    }

    // /// Set register value as f64
    // pub fn set_register_f64(&mut self, reg: u64, value: f64) {
    //     self.set_f64(reg, value);
    // }

    // /// Set raw register value
    // pub fn set_register_raw(&mut self, reg: u64, value: u64) {
    //     self.registers[reg as usize] = value;
    // }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_i64_arithmetic() {
        let mut vm = VirtualMachine::new();
        let program = vec![
            Instr::LoadI64(10, 1),
            Instr::LoadI64(5, 2),
            Instr::Add(1, 2, 0),
        ];

        vm.eval_program(&program).unwrap();
        assert_eq!(vm.get_register_i64(0), 15);
    }

    #[test]
    fn test_basic_f64_arithmetic() {
        let mut vm = VirtualMachine::new();
        let program = vec![
            Instr::LoadF64(3.14, 1),
            Instr::LoadF64(2.0, 2),
            Instr::Mul(1, 2, 0),
        ];

        vm.eval_program(&program).unwrap();
        let result = vm.get_register_f64(0);
        assert!((result - 6.28).abs() < 0.001);
    }

    #[test]
    fn test_type_conversions() {
        let mut vm = VirtualMachine::new();
        let program = vec![
            Instr::LoadI64(42, 1),
            Instr::I64ToF64(1, 2), // r2 = 42.0
            Instr::LoadF64(3.14, 3),
            Instr::F64ToI64(3, 4), // r4 = 3
            Instr::Add(2, 3, 5),   // r5 = 42.0 + 3.14 = 45.14
            Instr::F64ToI64(5, 0), // r0 = 45
        ];

        vm.eval_program(&program).unwrap();
        assert_eq!(vm.get_register_i64(0), 45);
        assert_eq!(vm.get_register_i64(4), 3);
        assert!((vm.get_register_f64(2) - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_comparison_and_jumps() {
        let mut vm = VirtualMachine::new();
        let program = vec![
            Instr::LoadI64(10, 1),   // r1 = 10
            Instr::LoadI64(5, 2),    // r2 = 5
            Instr::Gt(1, 2, 3),      // r3 = 1 (10 > 5)
            Instr::JmpIfFalse(3, 6), // Don't jump (r3 != 0)
            Instr::LoadI64(100, 0),  // r0 = 100
            Instr::Jmp(u64::MAX),    // Jump to end
            Instr::LoadI64(200, 0),  // r0 = 200 (skipped)
        ];

        vm.eval_program(&program).unwrap();
        assert_eq!(vm.get_register_i64(0), 100);
    }

    #[test]
    fn test_conditional_jump_with_zero() {
        let mut vm = VirtualMachine::new();
        let program = vec![
            Instr::LoadI64(5, 1),    // r1 = 5
            Instr::LoadI64(5, 2),    // r2 = 5
            Instr::Gt(1, 2, 3),      // r3 = 0 (5 > 5 is false)
            Instr::JmpIfFalse(3, 6), // Jump because r3 == 0
            Instr::LoadI64(100, 0),  // r0 = 100 (skipped)
            Instr::Jmp(u64::MAX),    // (skipped)
            Instr::LoadI64(200, 0),  // r0 = 200 (executed)
        ];

        vm.eval_program(&program).unwrap();
        assert_eq!(vm.get_register_i64(0), 200);
    }

    #[test]
    fn test_negative_numbers() {
        let mut vm = VirtualMachine::new();
        let program = vec![
            Instr::LoadI64(-10, 1),
            Instr::LoadI64(5, 2),
            Instr::Add(1, 2, 0), // -10 + 5 = -5
        ];

        vm.eval_program(&program).unwrap();
        assert_eq!(vm.get_register_i64(0), -5);
    }

    #[test]
    fn test_raw_register_operations() {
        let mut vm = VirtualMachine::new();

        // Store -1 as i64 (all bits set)
        vm.set_register_i64(0, -1);
        assert_eq!(vm.get_register_raw(0), u64::MAX);
        assert_eq!(vm.get_register_i64(0), -1);

        // The same bits interpreted as f64 should be NaN
        let as_f64 = vm.get_register_f64(0);
        assert!(as_f64.is_nan());
    }

    #[test]
    fn test_factorial_loop() {
        let mut vm = VirtualMachine::new();
        // Calculate factorial of 5: 5! = 120
        let program = vec![
            Instr::LoadI64(5, 1), // r1 = 5 (counter)
            Instr::LoadI64(1, 0), // r0 = 1 (result)
            Instr::LoadI64(1, 2), // r2 = 1 (decrement)
            Instr::LoadI64(0, 3), // r3 = 0 (comparison)
            // Loop start (instruction 4)
            Instr::Gt(1, 3, 4),       // r4 = (r1 > 0)
            Instr::JmpIfFalse(4, 10), // if r1 <= 0, exit loop
            Instr::Mul(0, 1, 0),      // r0 = r0 * r1
            Instr::Sub(1, 2, 1),      // r1 = r1 - 1
            Instr::Jmp(4),            // jump back to loop start
                                      // Loop end
        ];

        vm.eval_program(&program).unwrap();
        assert_eq!(vm.get_register_i64(0), 120);
    }

    #[test]
    fn test_mixed_arithmetic() {
        let mut vm = VirtualMachine::new();
        let program = vec![
            Instr::LoadI64(10, 1),  // r1 = 10
            Instr::I64ToF64(1, 2),  // r2 = 10.0
            Instr::LoadF64(2.5, 3), // r3 = 2.5
            Instr::Mul(2, 3, 0),    // r0 = 10.0 * 2.5 = 25.0
        ];

        vm.eval_program(&program).unwrap();
        let result = vm.get_register_f64(0);
        assert!((result - 25.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_subtraction_overflow() {
        let mut vm = VirtualMachine::new();
        let program = vec![
            Instr::LoadI64(5, 1),
            Instr::LoadI64(10, 2),
            Instr::Sub(1, 2, 0), // 5 - 10 = -5
        ];

        vm.eval_program(&program).unwrap();
        assert_eq!(vm.get_register_i64(0), -5);
    }

    #[test]
    fn test_f64_to_i64_truncation() {
        let mut vm = VirtualMachine::new();
        let program = vec![
            Instr::LoadF64(3.99, 1),
            Instr::F64ToI64(1, 0), // 3.99 -> 3 (truncated)
        ];

        vm.eval_program(&program).unwrap();
        assert_eq!(vm.get_register_i64(0), 3);
    }
}
