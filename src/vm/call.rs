use super::registers::Registers;
use super::VirtualMachine;

pub type HostFn = fn(base: usize, registers: &mut Registers) -> Result<(), String>;

#[derive(Clone)]
pub struct HostFunctionMetadata {
    pub name: &'static str,
    pub num_return_registers: usize,
    pub num_params: usize,
    pub num_registers: usize,
}

pub struct HostFunctionRegistry {
    pub funcs: Vec<HostFn>,
    pub metadata: Vec<HostFunctionMetadata>,
}

impl HostFunctionRegistry {
    pub fn new() -> Self {
        Self { funcs: Vec::new(), metadata: Vec::new() }
    }

    pub fn register(
        &mut self,
        name: &'static str,
        num_return_registers: usize,
        num_params: usize,
        num_registers: usize,
        func: HostFn,
    ) -> usize {
        let index = self.funcs.len();
        self.funcs.push(func);
        self.metadata.push(HostFunctionMetadata {
            name,
            num_return_registers,
            num_params,
            num_registers,
        });
        index
    }
}

#[derive(Debug, Clone)]
pub enum CallInfo {
    Global { base: usize, top: usize },
    Call { base: usize, top: usize, function_index: usize },
    CallHost { base: usize, top: usize, host_fn_index: usize },
}

impl VirtualMachine {
    pub fn current_base(&self) -> usize {
        self.call_stack
            .last()
            .map(|info| match info {
                CallInfo::Global { base, .. } => *base,
                CallInfo::Call { base, .. } => *base,
                CallInfo::CallHost { base, .. } => *base,
            })
            .unwrap_or(0)
    }
}
