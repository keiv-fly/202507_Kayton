use super::*;

#[derive(Debug)]
pub(crate) struct PendingJump {
    label_id: u32,
    patch_position: u16,
    jump_type: JumpType,
}

#[derive(Debug)]
pub(crate) enum JumpType {
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
