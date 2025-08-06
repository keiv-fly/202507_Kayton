use std::collections::HashMap;

use super::const_pool::{SliceType, ValueType};

#[derive(Debug, Clone, Copy)]
pub enum PtrType {
    Slice(SliceType),
}

#[derive(Debug, Clone, Copy)]
pub enum GlobalVarType {
    Value(ValueType),
    Ptr(PtrType),
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalVarMeta {
    pub typ: GlobalVarType,
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalVar {
    pub register_id: usize,
    pub meta: GlobalVarMeta,
}

#[derive(Debug, Default)]
pub struct GlobalVars {
    vars: HashMap<String, GlobalVar>,
}

impl GlobalVars {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn insert(&mut self, name: &str, register_id: usize, typ: GlobalVarType) {
        self.vars.insert(
            name.to_string(),
            GlobalVar {
                register_id,
                meta: GlobalVarMeta { typ },
            },
        );
    }

    pub fn get(&self, name: &str) -> Option<&GlobalVar> {
        self.vars.get(name)
    }

    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }
}

