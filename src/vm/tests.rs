use super::*;
use std::time::Duration;

#[test]
fn test_basic_i64_arithmetic() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.load_i64(10, 1);
    builder.load_i64(5, 2);
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
    let mut builder = BytecodeBuilder::new();
    builder.load_f64(3.14, 1);
    builder.load_f64(2.0, 2);
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
    let mut builder = BytecodeBuilder::new();
    builder.load_i64(42, 1);
    builder.i64_to_f64(1, 2); // r2 = 42.0
    builder.load_f64(3.14, 3);
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
fn test_comparison_and_jumps_with_labels() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let skip_label = builder.create_label();
    let end_label = builder.create_label();

    builder.load_i64(10, 1); // r1 = 10
    builder.load_i64(5, 2); // r2 = 5
    builder.gt_i64(1, 2, 3); // r3 = 1 (10 > 5)

    builder.jump_if_false_to_label(3, skip_label); // Don't jump (r3 != 0)
    builder.load_i64(100, 0); // r0 = 100
    builder.jmp_to_label(end_label);

    builder.place_label(skip_label);
    builder.load_i64(200, 0); // r0 = 200 (should be skipped)

    builder.place_label(end_label);

    let bytecode = builder.build();

    println!("=== test_comparison_and_jumps_with_labels bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 100);
}

#[test]
fn test_conditional_jump_with_zero() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let false_branch = builder.create_label();

    builder.load_i64(5, 1); // r1 = 5
    builder.load_i64(5, 2); // r2 = 5
    builder.gt_i64(1, 2, 3); // r3 = 0 (5 > 5 is false)

    builder.jump_if_false_to_label(3, false_branch); // Jump because r3 == 0
    builder.load_i64(100, 0); // r0 = 100 (should be skipped)

    builder.place_label(false_branch);
    builder.load_i64(200, 0); // r0 = 200 (should be executed)

    let bytecode = builder.build();

    println!("=== test_conditional_jump_with_zero bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 200);
}

#[test]
fn test_negative_numbers() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.load_i64(-10, 1);
    builder.load_i64(5, 2);
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
fn test_factorial_loop_with_labels() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let loop_start = builder.create_label();
    let loop_end = builder.create_label();

    builder.load_i64(5, 1); // r1 = 5 (counter)
    builder.load_i64(1, 0); // r0 = 1 (result)
    builder.load_i64(1, 2); // r2 = 1 (decrement)
    builder.load_i64(0, 3); // r3 = 0 (comparison)

    // Loop start
    builder.place_label(loop_start);
    builder.gt_i64(1, 3, 4); // r4 = (r1 > 0)
    builder.jump_if_false_to_label(4, loop_end); // if r1 <= 0, exit loop
    builder.mul_i64(0, 1, 0); // r0 = r0 * r1
    builder.sub_i64(1, 2, 1); // r1 = r1 - 1
    builder.jmp_to_label(loop_start); // jump back to loop start

    builder.place_label(loop_end);

    let bytecode = builder.build();

    println!("=== test_factorial_loop_with_labels bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 120);
}

#[test]
fn test_mixed_arithmetic() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.load_i64(10, 1); // r1 = 10
    builder.i64_to_f64(1, 2); // r2 = 10.0
    builder.load_f64(2.5, 3); // r3 = 2.5
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
    let bytecode = vec![LOAD_I64, 1]; // Missing the i64 value

    println!("=== test_unexpected_end_of_program bytecode ===");
    print_bytecode(&bytecode);
    println!();

    let result = vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)));
    assert!(matches!(result, Err(VmError::UnexpectedEndOfProgram)));
}

#[test]
fn test_invalid_jump_target() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.load_i64(1, 0);
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
    let mut builder = BytecodeBuilder::new();

    builder.load_f64(10.5, 1);
    builder.load_f64(3.2, 2);
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
fn test_f64_comparison() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    // Test case 1: 5.5 > 3.2 should be true (1)
    builder.load_f64(5.5, 1);
    builder.load_f64(3.2, 2);
    builder.gt_f64(1, 2, 0); // r0 = 1

    // Test case 2: 2.1 > 2.1 should be false (0)
    builder.load_f64(2.1, 3);
    builder.load_f64(2.1, 4);
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
    let mut builder = BytecodeBuilder::new();

    // Test: -1.5 > -2.7 should be true (1)
    builder.load_f64(-1.5, 1);
    builder.load_f64(-2.7, 2);
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
    let mut builder = BytecodeBuilder::new();

    // Calculate (10.0 - 3.0) and check if result > 5.0
    builder.load_f64(10.0, 1);
    builder.load_f64(3.0, 2);
    builder.sub_f64(1, 2, 3); // r3 = 7.0
    builder.load_f64(5.0, 4);
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
fn test_jump_forward_if_true_with_labels() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let target_label = builder.create_label();

    // Set up a true condition
    builder.load_i64(1, 1); // r1 = 1 (true)

    builder.jump_if_true_to_label(1, target_label); // Jump because r1 != 0
    builder.load_i64(100, 0); // r0 = 100 (should be skipped)

    builder.place_label(target_label);
    builder.load_i64(200, 0); // r0 = 200 (should be executed)

    let bytecode = builder.build();

    println!("=== test_jump_forward_if_true_with_labels bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 200); // Should have jumped to the target
}

#[test]
fn test_jump_forward_if_true_false_condition() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let target_label = builder.create_label();

    // Set up a false condition
    builder.load_i64(0, 1); // r1 = 0 (false)

    builder.jump_if_true_to_label(1, target_label); // Don't jump since r1 == 0
    builder.load_i64(100, 0); // r0 = 100 (should be executed)

    builder.place_label(target_label);
    builder.load_i64(200, 0); // r0 = 200 (should also be executed)

    let bytecode = builder.build();

    println!("=== test_jump_forward_if_true_false_condition bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 200); // Should execute both loads, ending with 200
}

#[test]
fn test_jump_backward_if_true_loop() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let loop_start = builder.create_label();

    builder.load_i64(3, 1); // r1 = 3 (counter)
    builder.load_i64(0, 0); // r0 = 0 (accumulator)
    builder.load_i64(1, 2); // r2 = 1 (decrement value)

    // Loop start
    builder.place_label(loop_start);
    builder.add_i64(0, 1, 0); // r0 = r0 + r1
    builder.sub_i64(1, 2, 1); // r1 = r1 - 1
    builder.jump_if_true_to_label(1, loop_start); // Jump back if r1 != 0

    let bytecode = builder.build();

    println!("=== test_jump_backward_if_true_loop bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 6); // 3 + 2 + 1 = 6
    assert_eq!(vm.get_register_i64(1), 0); // Counter should be 0
}

#[test]
fn test_jump_backward_if_false_exit_loop() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let loop_start = builder.create_label();

    builder.load_i64(0, 1); // r1 = 0 (counter)
    builder.load_i64(0, 0); // r0 = 0 (accumulator)
    builder.load_i64(1, 2); // r2 = 1 (increment value)
    builder.load_i64(5, 3); // r3 = 5 (target value)

    // Loop start
    builder.place_label(loop_start);
    builder.add_i64(0, 2, 0); // r0 = r0 + 1
    builder.add_i64(1, 2, 1); // r1 = r1 + 1
    builder.gt_i64(1, 3, 4); // r4 = (r1 > 5) ? 1 : 0
    builder.jump_if_false_to_label(4, loop_start); // Jump back if r1 <= 5

    let bytecode = builder.build();

    println!("=== test_jump_backward_if_false_exit_loop bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 6); // Should increment 6 times
    assert_eq!(vm.get_register_i64(1), 6); // Counter should be 6
}

#[test]
fn test_nested_conditional_jumps_with_labels() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let else_branch = builder.create_label();
    let inner_else = builder.create_label();
    let end_label = builder.create_label();

    builder.load_i64(1, 1); // r1 = 1 (first condition - true)
    builder.load_i64(0, 2); // r2 = 0 (second condition - false)

    builder.jump_if_false_to_label(1, else_branch); // Don't jump since r1 != 0

    // Inner conditional
    builder.jump_if_false_to_label(2, inner_else); // Jump since r2 == 0
    builder.load_i64(300, 0); // r0 = 300 (should be skipped)
    builder.jmp_to_label(end_label);

    // Inner else
    builder.place_label(inner_else);
    builder.load_i64(200, 0); // r0 = 200 (should be executed)
    builder.jmp_to_label(end_label);

    // Outer else
    builder.place_label(else_branch);
    builder.load_i64(100, 0); // r0 = 100 (should be skipped)

    builder.place_label(end_label);

    let bytecode = builder.build();

    println!("=== test_nested_conditional_jumps_with_labels bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 200);
}

#[test]
fn test_target_based_jumps() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    builder.load_i64(10, 1); // r1 = 10
    builder.load_i64(5, 2); // r2 = 5
    builder.gt_i64(1, 2, 3); // r3 = 1 (10 > 5)

    // Calculate target position for the jump
    let current_pos = builder.current_pos();
    let skip_instruction_size = 10; // LOAD_I64 instruction size
    let target = current_pos + 3 + skip_instruction_size; // +3 for jump instruction size

    builder.jump_forward_if_false_to(3, target); // Won't jump since r3 != 0
    builder.load_i64(100, 0); // r0 = 100 (should be executed)
    // target is here
    builder.load_i64(200, 0); // r0 = 200 (should also be executed)

    let bytecode = builder.build();

    println!("=== test_target_based_jumps bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 200); // Both instructions executed
}

#[test]
fn test_backward_jump_bounds_check() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    builder.load_i64(1, 1); // r1 = 1
    // Try to jump backward too far (offset larger than current position)
    builder.jump_backward_if_true(1, 13); // This should cause an error when executed

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
    let mut builder = BytecodeBuilder::new();

    // Simple program that should complete quickly
    builder.load_i64(10, 1);
    builder.load_i64(5, 2);
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
    builder.load_i64(100000, 1); // r1 = 100000 (large counter)
    builder.load_i64(0, 0); // r0 = 0 (accumulator)
    builder.load_i64(1, 2); // r2 = 1 (decrement)
    builder.load_i64(0, 3); // r3 = 0 (comparison)

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
    let mut builder = BytecodeBuilder::new();

    builder.load_i64(42, 1);
    builder.load_i64(8, 2);
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
        builder.load_i64(i, 1);
        builder.load_i64(1, 2);
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
    let mut builder = BytecodeBuilder::new();

    builder.load_i64(100, 1);
    builder.load_i64(50, 2);
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

// === NEW ENHANCED FUNCTIONALITY TESTS ===

#[test]
fn test_fibonacci_with_labels() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let loop_start = builder.create_label();
    let loop_end = builder.create_label();

    // Calculate 10th Fibonacci number
    builder.load_i64(10, 1); // r1 = n (target)
    builder.load_i64(0, 2); // r2 = a (previous)
    builder.load_i64(1, 3); // r3 = b (current)
    builder.load_i64(1, 4); // r4 = counter
    builder.load_i64(1, 5); // r5 = increment

    builder.place_label(loop_start);
    // Check if counter >= n
    builder.gt_i64(4, 1, 6); // r6 = (counter > n)
    builder.jump_if_true_to_label(6, loop_end);

    // temp = a + b
    builder.add_i64(2, 3, 7); // r7 = a + b
    // a = b
    builder.add_i64(3, 0, 2); // r2 = b (using add with 0 as copy)
    // b = temp
    builder.add_i64(7, 0, 3); // r3 = temp
    // counter++
    builder.add_i64(4, 5, 4); // r4 = counter + 1

    builder.jmp_to_label(loop_start);

    builder.place_label(loop_end);
    // Result is in r3 (b)
    builder.add_i64(3, 0, 0); // r0 = result

    let bytecode = builder.build();

    println!("=== test_fibonacci_with_labels bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), 55); // 10th Fibonacci number
}

#[test]
fn test_comparison_old_vs_new_approach() {
    // This test shows the same logic implemented both ways

    let mut vm1 = VirtualMachine::new();
    let mut vm2 = VirtualMachine::new();

    // OLD APPROACH: Manual patching
    let mut old_builder = BytecodeBuilder::new();
    old_builder.load_i64(10, 1);
    old_builder.load_i64(5, 2);
    old_builder.gt_i64(1, 2, 3);

    let target_pos = old_builder.jump_forward_if_false(3);
    old_builder.load_i64(100, 0);
    let end_pos = old_builder.current_pos();
    old_builder.load_i64(200, 0);

    old_builder.patch_target(target_pos, end_pos - target_pos);
    let old_bytecode = old_builder.build();

    // NEW APPROACH: Label-based
    let mut new_builder = BytecodeBuilder::new();
    let skip_label = new_builder.create_label();

    new_builder.load_i64(10, 1);
    new_builder.load_i64(5, 2);
    new_builder.gt_i64(1, 2, 3);
    new_builder.jump_if_false_to_label(3, skip_label);
    new_builder.load_i64(100, 0);
    new_builder.place_label(skip_label);
    new_builder.load_i64(200, 0);

    let new_bytecode = new_builder.build();

    println!("=== OLD APPROACH bytecode ===");
    print_bytecode(&old_bytecode);
    println!();

    println!("=== NEW APPROACH bytecode ===");
    print_bytecode(&new_bytecode);
    println!();

    // Both should produce identical results
    vm1.eval_program(&old_bytecode).unwrap();
    vm2.eval_program(&new_bytecode).unwrap();

    assert_eq!(vm1.get_register_i64(0), vm2.get_register_i64(0));
    assert_eq!(vm1.get_register_i64(0), 200);
}

#[test]
fn test_complex_control_flow() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let check_positive = builder.create_label();
    let positive_branch = builder.create_label();
    let negative_branch = builder.create_label();
    let zero_branch = builder.create_label();
    let end_label = builder.create_label();

    // Classify a number as positive, negative, or zero
    builder.load_i64(-5, 1); // r1 = -5 (test value)
    builder.load_i64(0, 2); // r2 = 0 (comparison)

    // Check if equal to zero
    builder.sub_i64(1, 2, 3); // r3 = value - 0
    builder.jump_if_false_to_label(3, zero_branch); // if value == 0

    // Check if positive
    builder.place_label(check_positive);
    builder.gt_i64(1, 2, 4); // r4 = (value > 0)
    builder.jump_if_true_to_label(4, positive_branch);
    builder.jmp_to_label(negative_branch);

    builder.place_label(positive_branch);
    builder.load_i64(1, 0); // r0 = 1 (positive)
    builder.jmp_to_label(end_label);

    builder.place_label(negative_branch);
    builder.load_i64(-1, 0); // r0 = -1 (negative)
    builder.jmp_to_label(end_label);

    builder.place_label(zero_branch);
    builder.load_i64(0, 0); // r0 = 0 (zero)

    builder.place_label(end_label);

    let bytecode = builder.build();

    println!("=== test_complex_control_flow bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program_with_timeout(&bytecode, Some(Duration::from_secs(1)))
        .unwrap();
    assert_eq!(vm.get_register_i64(0), -1); // Should identify as negative
}

#[test]
fn test_error_handling_with_labels() {
    // Test that unresolved labels cause panics
    let result = std::panic::catch_unwind(|| {
        let mut builder = BytecodeBuilder::new();
        builder.load_i64(1, 1);
        builder.jump_if_true_to_label(1, 999); // Non-existent label
        let _bytecode = builder.build(); // Should panic here
    });

    assert!(result.is_err()); // Should have panicked
    println!("Correctly panicked on unresolved label");
}

#[test]
fn test_target_based_error_handling() {
    // Test forward jump with invalid target
    let result = std::panic::catch_unwind(|| {
        let mut builder = BytecodeBuilder::new();
        builder.load_i64(1, 1);
        let current = builder.current_pos();
        builder.jump_forward_if_false_to(1, current - 1); // Invalid: backwards target
    });
    assert!(result.is_err());

    // Test backward jump with invalid target
    let result = std::panic::catch_unwind(|| {
        let mut builder = BytecodeBuilder::new();
        builder.load_i64(1, 1);
        let current = builder.current_pos();
        builder.jump_backward_if_false_to(1, current + 10); // Invalid: forwards target
    });
    assert!(result.is_err());

    println!("Target-based error handling tests passed");
}
