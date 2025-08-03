use super::*;
use super::const_pool::ValueType;

fn add_i64(vm: &mut VirtualMachine, value: i64) -> u16 {
    vm.const_pool.add_value("", value as u64, ValueType::I64) as u16
}

fn add_fn(vm: &mut VirtualMachine, index: usize) -> u16 {
    vm.const_pool
        .add_value("", index as u64, ValueType::FuncHost) as u16
}

fn inc(base: usize, registers: &mut Registers) -> Result<(), String> {
    let val = registers.get(base + 1);
    registers.set(base, val + 1);
    Ok(())
}

#[test]
fn test_call_host_basic() {
    let mut vm = VirtualMachine::new();
    let fn_index = vm
        .host_functions
        .register("inc", 1, 1, 2, inc);

    let fn_idx_const = add_fn(&mut vm, fn_index);
    let idx41 = add_i64(&mut vm, 41);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(fn_idx_const, 10);
    builder.load_const_value(idx41, 11);
    builder.call_host(10);
    let bytecode = builder.build();

    vm.eval_program(&bytecode).unwrap();
    assert_eq!(vm.get_register_i64(10), 42);
}

#[test]
fn test_call_host_invalid_index() {
    let mut vm = VirtualMachine::new();
    let idx = add_i64(&mut vm, 999);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(idx, 0);
    builder.call_host(0);
    let bytecode = builder.build();

    let result = vm.eval_program(&bytecode);
    assert!(matches!(result, Err(VmError::InvalidConstIndex(999))));
}

#[test]
fn test_call_host_register_isolation() {
    let mut vm = VirtualMachine::new();
    let fn_index = vm
        .host_functions
        .register("inc", 1, 1, 2, inc);

    let idx77 = add_i64(&mut vm, 77);
    let fn_idx_const = add_fn(&mut vm, fn_index);
    let idx1 = add_i64(&mut vm, 1);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(idx77, 5);
    builder.load_const_value(fn_idx_const, 10);
    builder.load_const_value(idx1, 11);
    builder.call_host(10);
    let bytecode = builder.build();

    vm.eval_program(&bytecode).unwrap();
    assert_eq!(vm.get_register_i64(5), 77);
    assert_eq!(vm.get_register_i64(10), 2);
}

#[test]
fn test_call_host_multiple_calls() {
    let mut vm = VirtualMachine::new();
    let fn_index = vm
        .host_functions
        .register("inc", 1, 1, 2, inc);
    let fn_idx_const = add_fn(&mut vm, fn_index);
    let idx5 = add_i64(&mut vm, 5);
    let idx100 = add_i64(&mut vm, 100);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(fn_idx_const, 10);
    builder.load_const_value(idx5, 11);
    builder.call_host(10);
    builder.load_const_value(fn_idx_const, 20);
    builder.load_const_value(idx100, 21);
    builder.call_host(20);
    let bytecode = builder.build();

    vm.eval_program(&bytecode).unwrap();
    assert_eq!(vm.get_register_i64(10), 6);
    assert_eq!(vm.get_register_i64(20), 101);
}
