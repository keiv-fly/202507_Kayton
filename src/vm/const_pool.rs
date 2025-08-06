use bumpalo::Bump;
use std::collections::HashMap;

//
// Value constants
//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    I64,
    F64,
    Bool,
    FuncHost,
    // Add more types if needed
}

#[derive(Debug)]
pub struct ValueConstMeta {
    pub name: &'static str,
    pub typ: ValueType,
    pub index: usize,
}

//
// Slice constants
//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliceType {
    Utf8Str,
    Binary,
}

#[derive(Debug)]
pub struct SliceConstMeta {
    pub name: &'static str,
    pub typ: SliceType,
    pub index: usize,
}

//
// Unified constant pool
//

pub struct ConstPool {
    arena: Bump,

    // value constants
    pub values: Vec<u64>,
    pub value_metadata: Vec<ValueConstMeta>,
    pub value_name_to_index: HashMap<&'static str, usize>,

    // slice constants
    pub slices: Vec<&'static [u8]>,
    pub slice_metadata: Vec<SliceConstMeta>,
    pub slice_name_to_index: HashMap<&'static str, usize>,
}

impl ConstPool {
    pub fn new() -> Self {
        ConstPool {
            arena: Bump::new(),

            values: Vec::new(),
            value_metadata: Vec::new(),
            value_name_to_index: HashMap::new(),

            slices: Vec::new(),
            slice_metadata: Vec::new(),
            slice_name_to_index: HashMap::new(),
        }
    }

    pub fn add_value(&mut self, name: &str, value: u64, typ: ValueType) -> usize {
        let name_static = self.alloc_static_str(name);
        let index = self.values.len();
        self.values.push(value);
        self.value_metadata.push(ValueConstMeta {
            name: name_static,
            typ,
            index,
        });
        self.value_name_to_index.insert(name_static, index);
        index
    }

    pub fn add_slice(&mut self, name: &str, data: &[u8], typ: SliceType) -> usize {
        let name_static = self.alloc_static_str(name);
        let data_static = self.alloc_static_slice(data);
        let index = self.slices.len();
        self.slices.push(data_static);
        self.slice_metadata.push(SliceConstMeta {
            name: name_static,
            typ,
            index,
        });
        self.slice_name_to_index.insert(name_static, index);
        index
    }

    pub fn get_value(&self, name: &str) -> Option<u64> {
        self.value_name_to_index.get(name).map(|&i| self.values[i])
    }

    pub fn get_slice(&self, name: &str) -> Option<&'static [u8]> {
        self.slice_name_to_index.get(name).map(|&i| self.slices[i])
    }

    fn alloc_static_str(&self, s: &str) -> &'static str {
        let s = self.arena.alloc_str(s);
        unsafe { std::mem::transmute::<&str, &'static str>(s) }
    }

    fn alloc_static_slice(&self, slice: &[u8]) -> &'static [u8] {
        let s = self.arena.alloc_slice_copy(slice);
        unsafe { std::mem::transmute::<&[u8], &'static [u8]>(s) }
    }
}
