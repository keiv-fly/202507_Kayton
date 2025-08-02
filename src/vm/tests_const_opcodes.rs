use super::{BytecodeBuilder, VirtualMachine};
use super::const_pool::{ConstPool, SliceType, ValueType};

#[test]
fn test_load_const_value_and_slice() {
    let mut pool = ConstPool::new();
    pool.add_value("42", 42u64, ValueType::I64);
    pool.add_slice("hello", b"hello", SliceType::Utf8Str);

    let mut vm = VirtualMachine::new();
    vm.set_const_pool(&pool);

    let mut builder = BytecodeBuilder::new();
    let idx_v = *pool.value_name_to_index.get("42").unwrap() as u16;
    let idx_s = *pool.slice_name_to_index.get("hello").unwrap() as u16;
    builder.load_const_value(idx_v, 0);
    builder.load_const_slice(idx_s, 1);
    let bytecode = builder.build();

    vm.eval_program(&bytecode).unwrap();
    assert_eq!(vm.get_register_raw(0), pool.get_value("42").unwrap());
    let ptr = vm.get_register_raw(1) as *const u8;
    let len = vm.get_register_raw(2) as usize;
    let data = unsafe { std::slice::from_raw_parts(ptr, len) };
    assert_eq!(data, pool.get_slice("hello").unwrap());
}
