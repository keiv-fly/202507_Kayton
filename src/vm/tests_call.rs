use super::*;

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

    let mut builder = BytecodeBuilder::new();
    builder.load_i64(fn_index as i64, 10);
    builder.load_i64(41, 11);
    builder.call_host(10);
    let bytecode = builder.build();

    vm.eval_program(&bytecode).unwrap();
    assert_eq!(vm.get_register_i64(10), 42);
}

#[test]
fn test_call_host_invalid_index() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.load_i64(999, 0);
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

    let mut builder = BytecodeBuilder::new();
    builder.load_i64(77, 5);
    builder.load_i64(fn_index as i64, 10);
    builder.load_i64(1, 11);
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

    let mut builder = BytecodeBuilder::new();
    builder.load_i64(fn_index as i64, 10);
    builder.load_i64(5, 11);
    builder.call_host(10);
    builder.load_i64(fn_index as i64, 20);
    builder.load_i64(100, 21);
    builder.call_host(20);
    let bytecode = builder.build();

    vm.eval_program(&bytecode).unwrap();
    assert_eq!(vm.get_register_i64(10), 6);
    assert_eq!(vm.get_register_i64(20), 101);
}
