use super::const_pool::ValueType;
use super::*;
use std::time::Duration;

fn add_i64(vm: &mut VirtualMachine, value: i64) -> u16 {
    vm.const_pool.add_value("", value as u64, ValueType::I64) as u16
}

fn add_f64(vm: &mut VirtualMachine, value: f64) -> u16 {
    vm.const_pool.add_value("", value.to_bits(), ValueType::F64) as u16
}

#[test]
fn test_basic_i64_arithmetic() {
    let mut vm = VirtualMachine::new();
    let idx10 = add_i64(&mut vm, 10);
    let idx5 = add_i64(&mut vm, 5);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(idx10, 1);
    builder.load_const_value(idx5, 2);
    builder.add_i64(1, 2, 0);
    let bytecode = builder.build();

    println!("=== test_basic_i64_arithmetic bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 15);
}

#[test]
fn test_basic_f64_arithmetic() {
    let mut vm = VirtualMachine::new();
    let idx1 = add_f64(&mut vm, 3.14);
    let idx2 = add_f64(&mut vm, 2.0);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(idx1, 1);
    builder.load_const_value(idx2, 2);
    builder.mul_f64(1, 2, 0);
    let bytecode = builder.build();

    println!("=== test_basic_f64_arithmetic bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    let result = vm.get_register_f64(0);
    assert!((result - 6.28).abs() < 0.001);
}

#[test]
fn test_type_conversions() {
    let mut vm = VirtualMachine::new();
    let idx42 = add_i64(&mut vm, 42);
    let idx314 = add_f64(&mut vm, 3.14);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(idx42, 1);
    builder.i64_to_f64(1, 2); // r2 = 42.0
    builder.load_const_value(idx314, 3);
    builder.f64_to_i64(3, 4); // r4 = 3
    builder.add_f64(2, 3, 5); // r5 = 42.0 + 3.14 = 45.14
    builder.f64_to_i64(5, 0); // r0 = 45
    let bytecode = builder.build();

    println!("=== test_type_conversions bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 45);
    assert_eq!(vm.get_register_i64(4), 3);
    assert!((vm.get_register_f64(2) - 42.0).abs() < f64::EPSILON);
}

#[test]
fn test_negative_numbers() {
    let mut vm = VirtualMachine::new();
    let idx_neg10 = add_i64(&mut vm, -10);
    let idx5 = add_i64(&mut vm, 5);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(idx_neg10, 1);
    builder.load_const_value(idx5, 2);
    builder.add_i64(1, 2, 0); // -10 + 5 = -5
    let bytecode = builder.build();

    println!("=== test_negative_numbers bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), -5);
}

#[test]
fn test_register_base_offset() {
    let mut vm = VirtualMachine::new();
    let idx2 = add_i64(&mut vm, 2);
    let idx3 = add_i64(&mut vm, 3);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(idx2, 0);
    builder.load_const_value(idx3, 1);
    builder.add_i64(0, 1, 2);
    let bytecode = builder.build();

    vm.base = 5;
    vm.registers.ensure_len(5 + 3);

    vm.eval_program(&bytecode).unwrap();

    assert_eq!(vm.get_register_i64(7), 5);
    assert_eq!(vm.get_register_i64(2), 0);
}

#[test]
fn test_mixed_arithmetic() {
    let mut vm = VirtualMachine::new();
    let idx10 = add_i64(&mut vm, 10);
    let idx2_5 = add_f64(&mut vm, 2.5);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(idx10, 1); // r1 = 10
    builder.i64_to_f64(1, 2); // r2 = 10.0
    builder.load_const_value(idx2_5, 3); // r3 = 2.5
    builder.mul_f64(2, 3, 0); // r0 = 10.0 * 2.5 = 25.0
    let bytecode = builder.build();

    println!("=== test_mixed_arithmetic bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    let result = vm.get_register_f64(0);
    assert!((result - 25.0).abs() < f64::EPSILON);
}

#[test]
fn test_invalid_opcode() {
    let mut vm = VirtualMachine::new();
    let bytecode = vec![0xFF]; // Invalid opcode

    println!("=== test_invalid_opcode bytecode ===");
    print_bytecode(&bytecode);
    println!();

    let result = vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)));
    assert!(matches!(result, Err(VmError::InvalidOpcode(0xFF))));
}

#[test]
fn test_unexpected_end_of_program() {
    let mut vm = VirtualMachine::new();
    let bytecode = vec![LOAD_CONST_VALUE, 1]; // Missing index bytes

    println!("=== test_unexpected_end_of_program bytecode ===");
    print_bytecode(&bytecode);
    println!();

    let result = vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)));
    assert!(matches!(result, Err(VmError::UnexpectedEndOfProgram)));
}

#[test]
fn test_invalid_jump_target() {
    let mut vm = VirtualMachine::new();
    let idx1 = add_i64(&mut vm, 1);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(idx1, 0);
    builder.jmp_to(1000); // Invalid target - beyond program length
    let bytecode = builder.build();

    println!("=== test_invalid_jump_target bytecode ===");
    print_bytecode(&bytecode);
    println!();

    let result = vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)));
    assert!(matches!(result, Err(VmError::InvalidJumpTarget(1000))));
}

#[test]
fn test_register_raw_operations() {
    let mut vm = VirtualMachine::new();

    // Test setting and getting raw register values
    vm.set_register_raw(0, 0x123456789ABCDEF0);
    assert_eq!(vm.get_register_raw(0), 0x123456789ABCDEF0);

    // Test setting and getting i64 values using public methods
    vm.set_register_i64(1, -42);
    assert_eq!(vm.get_register_i64(1), -42);

    // Test setting and getting f64 values using public methods
    vm.set_register_f64(2, 3.14159);
    assert!((vm.get_register_f64(2) - 3.14159).abs() < f64::EPSILON);

    // Verify that raw access shows the bit representation
    let pi_bits = 3.14159f64.to_bits();
    vm.set_register_f64(3, 3.14159);
    assert_eq!(vm.get_register_raw(3), pi_bits);
}

#[test]
fn test_f64_subtraction() {
    let mut vm = VirtualMachine::new();
    let idx1 = add_f64(&mut vm, 10.5);
    let idx2 = add_f64(&mut vm, 3.2);
    let mut builder = BytecodeBuilder::new();

    builder.load_const_value(idx1, 1);
    builder.load_const_value(idx2, 2);
    builder.sub_f64(1, 2, 0); // r0 = 10.5 - 3.2 = 7.3
    let bytecode = builder.build();

    println!("=== test_f64_subtraction bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    let result = vm.get_register_f64(0);
    assert!((result - 7.3).abs() < 0.001);
}

#[test]
fn test_i64_comparison_ops() {
    let mut vm = VirtualMachine::new();
    let idx5a = add_i64(&mut vm, 5);
    let idx5b = add_i64(&mut vm, 5);
    let mut builder = BytecodeBuilder::new();

    builder.load_const_value(idx5a, 1);
    builder.load_const_value(idx5b, 2);
    builder.gte_i64(1, 2, 0); // r0 = 1
    builder.lt_i64(1, 2, 3); // r3 = 0
    builder.lte_i64(1, 2, 4); // r4 = 1

    let bytecode = builder.build();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 1);
    assert_eq!(vm.get_register_i64(3), 0);
    assert_eq!(vm.get_register_i64(4), 1);
}

#[test]
fn test_f64_comparison_ops() {
    let mut vm = VirtualMachine::new();
    let idx2_5 = add_f64(&mut vm, 2.5);
    let idx3 = add_f64(&mut vm, 3.0);
    let mut builder = BytecodeBuilder::new();

    builder.load_const_value(idx2_5, 1);
    builder.load_const_value(idx3, 2);
    builder.gte_f64(1, 2, 0); // r0 = 0
    builder.lt_f64(1, 2, 3); // r3 = 1
    builder.lte_f64(1, 2, 4); // r4 = 1

    let bytecode = builder.build();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 0);
    assert_eq!(vm.get_register_i64(3), 1);
    assert_eq!(vm.get_register_i64(4), 1);
}

#[test]
fn test_f64_comparison() {
    let mut vm = VirtualMachine::new();
    let idx5_5 = add_f64(&mut vm, 5.5);
    let idx3_2 = add_f64(&mut vm, 3.2);
    let idx2_1a = add_f64(&mut vm, 2.1);
    let idx2_1b = add_f64(&mut vm, 2.1);
    let mut builder = BytecodeBuilder::new();

    // Test case 1: 5.5 > 3.2 should be true (1)
    builder.load_const_value(idx5_5, 1);
    builder.load_const_value(idx3_2, 2);
    builder.gt_f64(1, 2, 0); // r0 = 1

    // Test case 2: 2.1 > 2.1 should be false (0)
    builder.load_const_value(idx2_1a, 3);
    builder.load_const_value(idx2_1b, 4);
    builder.gt_f64(3, 4, 5); // r5 = 0

    let bytecode = builder.build();

    println!("=== test_f64_comparison bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 1); // 5.5 > 3.2 is true
    assert_eq!(vm.get_register_i64(5), 0); // 2.1 > 2.1 is false
}

#[test]
fn test_f64_comparison_with_negatives() {
    let mut vm = VirtualMachine::new();
    let idx_neg1_5 = add_f64(&mut vm, -1.5);
    let idx_neg2_7 = add_f64(&mut vm, -2.7);
    let mut builder = BytecodeBuilder::new();

    // Test: -1.5 > -2.7 should be true (1)
    builder.load_const_value(idx_neg1_5, 1);
    builder.load_const_value(idx_neg2_7, 2);
    builder.gt_f64(1, 2, 0); // r0 = 1

    let bytecode = builder.build();

    println!("=== test_f64_comparison_with_negatives bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 1); // -1.5 > -2.7 is true
}

#[test]
fn test_complex_f64_operations() {
    let mut vm = VirtualMachine::new();
    let idx10 = add_f64(&mut vm, 10.0);
    let idx3 = add_f64(&mut vm, 3.0);
    let idx5 = add_f64(&mut vm, 5.0);
    let mut builder = BytecodeBuilder::new();

    // Calculate (10.0 - 3.0) and check if result > 5.0
    builder.load_const_value(idx10, 1);
    builder.load_const_value(idx3, 2);
    builder.sub_f64(1, 2, 3); // r3 = 7.0
    builder.load_const_value(idx5, 4);
    builder.gt_f64(3, 4, 0); // r0 = 1 (7.0 > 5.0)

    let bytecode = builder.build();

    println!("=== test_complex_f64_operations bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 1); // 7.0 > 5.0 is true
    assert!((vm.get_register_f64(3) - 7.0).abs() < f64::EPSILON);
}

#[test]
fn test_backward_jump_bounds_check() {
    let mut vm = VirtualMachine::new();
    let idx1 = add_i64(&mut vm, 1);
    let mut builder = BytecodeBuilder::new();

    builder.load_const_value(idx1, 1); // r1 = 1
    // Try to jump backward too far (offset larger than current position)
    builder.jump_backward_if_true(1, 15); // This should cause an error when executed

    let bytecode = builder.build();

    println!("=== test_backward_jump_bounds_check bytecode ===");
    print_bytecode(&bytecode);
    println!();

    let result = vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)));
    println!("result={:?}", result);
    assert!(matches!(result, Err(VmError::InvalidJumpTarget(_))));
}

// TIMEOUT TESTS

#[test]
fn test_execution_with_timeout_no_timeout() {
    let mut vm = VirtualMachine::new();
    let idx10 = add_i64(&mut vm, 10);
    let idx5 = add_i64(&mut vm, 5);
    let mut builder = BytecodeBuilder::new();

    // Simple program that should complete quickly
    builder.load_const_value(idx10, 1);
    builder.load_const_value(idx5, 2);
    builder.add_i64(1, 2, 0);
    let bytecode = builder.build();

    println!("=== test_execution_with_timeout_no_timeout bytecode ===");
    print_bytecode(&bytecode);
    println!();

    // Run with a generous timeout - should not timeout
    let result = vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)));
    assert!(result.is_ok());
    assert_eq!(vm.get_register_i64(0), 15);
}

#[test]
fn test_execution_with_short_timeout() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let loop_start = builder.create_label();
    let loop_end = builder.create_label();

    // Create a long-running program with a tight loop
    let idx_counter = add_i64(&mut vm, 100000);
    let idx_zero = add_i64(&mut vm, 0);
    let idx_one = add_i64(&mut vm, 1);
    builder.load_const_value(idx_counter, 1); // r1 = 100000 (large counter)
    builder.load_const_value(idx_zero, 0); // r0 = 0 (accumulator)
    builder.load_const_value(idx_one, 2); // r2 = 1 (decrement)
    builder.load_const_value(idx_zero, 3); // r3 = 0 (comparison)

    // Loop start - this will run many iterations
    builder.place_label(loop_start);
    builder.gt_i64(1, 3, 4); // r4 = (r1 > 0)
    builder.jump_if_false_to_label(4, loop_end); // if r1 <= 0, exit loop
    builder.add_i64(0, 2, 0); // r0 = r0 + 1
    builder.sub_i64(1, 2, 1); // r1 = r1 - 1
    builder.jmp_to_label(loop_start); // jump back to loop start

    builder.place_label(loop_end);

    let bytecode = builder.build();

    println!("=== test_execution_with_short_timeout bytecode ===");
    print_bytecode(&bytecode);
    println!();

    // Run with a very short timeout - should timeout
    let result = vm.eval_program_with_timeout(&bytecode, Some(Duration::from_millis(1)));

    match result {
        Err(VmError::Timeout(elapsed)) => {
            println!("Program timed out after {:?}", elapsed);
            assert!(elapsed > Duration::from_millis(1));
        }
        _ => panic!("Expected timeout error, got {:?}", result),
    }
}

#[test]
fn test_execution_with_no_timeout_specified() {
    let mut vm = VirtualMachine::new();
    let idx42 = add_i64(&mut vm, 42);
    let idx8 = add_i64(&mut vm, 8);
    let mut builder = BytecodeBuilder::new();

    builder.load_const_value(idx42, 1);
    builder.load_const_value(idx8, 2);
    builder.mul_i64(1, 2, 0);
    let bytecode = builder.build();

    println!("=== test_execution_with_no_timeout_specified bytecode ===");
    print_bytecode(&bytecode);
    println!();

    // Run with no timeout (None) - should work normally
    let result = vm.eval_program_with_timeout(&bytecode, None);
    assert!(result.is_ok());
    assert_eq!(vm.get_register_i64(0), 336);
}

#[test]
fn test_timeout_check_interval() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    // Create a program with exactly 999 instructions (less than TIMEOUT_CHECK_INTERVAL)
    // This tests that short programs complete without timeout checks
    for i in 0..333 {
        let idxi = add_i64(&mut vm, i);
        let idx1 = add_i64(&mut vm, 1);
        builder.load_const_value(idxi, 1);
        builder.load_const_value(idx1, 2);
        builder.add_i64(1, 2, 0); // 3 instructions per iteration = 999 total
    }

    let bytecode = builder.build();

    println!("=== test_timeout_check_interval bytecode ===");
    println!("Bytecode length: {} bytes", bytecode.len());
    println!();

    // Even with a very short timeout, this should complete because
    // it has fewer than TIMEOUT_CHECK_INTERVAL instructions
    let result = vm.eval_program_with_timeout(&bytecode, Some(Duration::from_nanos(1)));
    assert!(result.is_ok());
}

#[test]
fn test_backward_compatibility() {
    let mut vm = VirtualMachine::new();
    let idx100 = add_i64(&mut vm, 100);
    let idx50 = add_i64(&mut vm, 50);
    let mut builder = BytecodeBuilder::new();

    builder.load_const_value(idx100, 1);
    builder.load_const_value(idx50, 2);
    builder.sub_i64(1, 2, 0);
    let bytecode = builder.build();

    println!("=== test_backward_compatibility bytecode ===");
    print_bytecode(&bytecode);
    println!();

    // Test that the original eval_program method still works
    let result = vm.eval_program(&bytecode);
    assert!(result.is_ok());
    assert_eq!(vm.get_register_i64(0), 50);
}

#[test]
fn test_timeout_error_display() {
    let timeout_error = VmError::Timeout(Duration::from_millis(500));
    let error_string = format!("{}", timeout_error);
    assert!(error_string.contains("Execution timeout"));
    assert!(error_string.contains("500ms"));
    println!("Timeout error display: {}", error_string);
}
