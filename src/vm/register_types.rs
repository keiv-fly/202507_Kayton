use super::global_vars::GlobalVarType;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterType {
    /// Default value register
    ValueRegister = 0,
    /// Main register for an allocated pointer variable, carrying its type
    AllocatedPtrVarMain(GlobalVarType),
    /// Additional register for an allocated pointer variable (e.g. length)
    AllocatedPtrVarOther = 2,
    /// Register holding pointer to constant slice data
    ConstSliceVarMain = 5,
    /// Register holding length for constant slice
    ConstSliceVarLen = 6,
}

impl Default for RegisterType {
    fn default() -> Self {
        RegisterType::ValueRegister
    }
}

pub struct RegisterTypes {
    fixed: [RegisterType; super::registers::Registers::FIXED_COUNT],
    spill: Vec<RegisterType>,
}

impl RegisterTypes {
    pub const FIXED_COUNT: usize = super::registers::Registers::FIXED_COUNT;
    pub const SPILL_INIT: usize = super::registers::Registers::SPILL_INIT;

    pub fn new() -> Self {
        Self {
            fixed: [RegisterType::ValueRegister; Self::FIXED_COUNT],
            spill: Vec::with_capacity(Self::SPILL_INIT),
        }
    }

    pub fn get(&self, index: usize) -> RegisterType {
        if index < Self::FIXED_COUNT {
            self.fixed[index]
        } else {
            let spill_index = index - Self::FIXED_COUNT;
            self.spill.get(spill_index).copied().unwrap_or_default()
        }
    }

    pub fn set(&mut self, index: usize, value: RegisterType) {
        if index < Self::FIXED_COUNT {
            self.fixed[index] = value;
        } else {
            let spill_index = index - Self::FIXED_COUNT;
            if spill_index >= self.spill.len() {
                self.spill
                    .resize(spill_index + 1, RegisterType::ValueRegister);
            }
            self.spill[spill_index] = value;
        }
    }

    pub fn ensure_len(&mut self, len: usize) {
        if len <= Self::FIXED_COUNT {
            return;
        }
        let spill_needed = len - Self::FIXED_COUNT;
        if spill_needed > self.spill.len() {
            self.spill
                .resize(spill_needed, RegisterType::ValueRegister);
        }
    }
}

impl Default for RegisterTypes {
    fn default() -> Self {
        Self::new()
    }
}
