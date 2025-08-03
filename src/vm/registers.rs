pub struct Registers {
    fixed: [u64; Self::FIXED_COUNT],
    spill: Vec<u64>,
}

impl Registers {
    pub const FIXED_COUNT: usize = 256;
    pub const SPILL_INIT: usize = 256;

    pub fn new() -> Self {
        Self {
            fixed: [0; Self::FIXED_COUNT],
            spill: Vec::with_capacity(Self::SPILL_INIT),
        }
    }

    pub fn get(&self, index: usize) -> u64 {
        if index < Self::FIXED_COUNT {
            self.fixed[index]
        } else {
            let spill_index = index - Self::FIXED_COUNT;
            self.spill.get(spill_index).copied().unwrap_or(0)
        }
    }

    pub fn set(&mut self, index: usize, value: u64) {
        if index < Self::FIXED_COUNT {
            self.fixed[index] = value;
        } else {
            let spill_index = index - Self::FIXED_COUNT;
            if spill_index >= self.spill.len() {
                self.spill.resize(spill_index + 1, 0);
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
            self.spill.resize(spill_needed, 0);
        }
    }

}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
