use super::*;

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

    vm.eval_program(&bytecode).unwrap();
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

    vm.eval_program(&bytecode).unwrap();
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

    vm.eval_program(&bytecode).unwrap();
    assert_eq!(vm.get_register_i64(0), 45);
    assert_eq!(vm.get_register_i64(4), 3);
    assert!((vm.get_register_f64(2) - 42.0).abs() < f64::EPSILON);
}

#[test]
fn test_comparison_and_jumps() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    builder.load_i64(10, 1); // r1 = 10
    builder.load_i64(5, 2); // r2 = 5
    builder.gt_i64(1, 2, 3); // r3 = 1 (10 > 5)

    let target_pos = builder.jump_forward_if_false(3); // Don't jump (r3 != 0)
    builder.load_i64(100, 0); // r0 = 100
    let jmp_target_pos = builder.jmp(0); // Jump to end
    let skip_pos = builder.current_pos();
    builder.load_i64(200, 0); // r0 = 200 (should be skipped)

    // Patch the conditional jump to point to the skipped instruction
    builder.patch_target(target_pos, skip_pos - target_pos);
    let end_pos = builder.current_pos() as u16;
    builder.patch_target(jmp_target_pos, end_pos);

    let bytecode = builder.build();

    println!("=== test_comparison_and_jumps bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program(&bytecode).unwrap();
    assert_eq!(vm.get_register_i64(0), 100);
}

#[test]
fn test_conditional_jump_with_zero() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    builder.load_i64(5, 1); // r1 = 5
    builder.load_i64(5, 2); // r2 = 5
    builder.gt_i64(1, 2, 3); // r3 = 0 (5 > 5 is false)

    let target_pos = builder.jump_forward_if_false(3); // Jump because r3 == 0
    builder.load_i64(100, 0); // r0 = 100 (should be skipped)
    let end_pos = builder.current_pos() + 3; // After the jmp instruction
    builder.jmp(end_pos); // (should be skipped)
    let false_branch_pos = builder.current_pos();
    builder.load_i64(200, 0); // r0 = 200 (should be executed)

    // Patch the conditional jump to point to the false branch
    builder.patch_target(target_pos, false_branch_pos - target_pos);

    let bytecode = builder.build();

    println!("=== test_conditional_jump_with_zero bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program(&bytecode).unwrap();
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

    vm.eval_program(&bytecode).unwrap();
    assert_eq!(vm.get_register_i64(0), -5);
}

#[test]
fn test_factorial_loop() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    builder.load_i64(5, 1); // r1 = 5 (counter)
    builder.load_i64(1, 0); // r0 = 1 (result)
    builder.load_i64(1, 2); // r2 = 1 (decrement)
    builder.load_i64(0, 3); // r3 = 0 (comparison)

    // Loop start
    let loop_start = builder.current_pos();
    builder.gt_i64(1, 3, 4); // r4 = (r1 > 0)
    let target_pos = builder.jump_forward_if_false(4); // if r1 <= 0, exit loop
    builder.mul_i64(0, 1, 0); // r0 = r0 * r1
    builder.sub_i64(1, 2, 1); // r1 = r1 - 1
    builder.jmp(loop_start); // jump back to loop start

    let loop_end = builder.current_pos();
    // Patch the conditional jump to point to loop end
    builder.patch_target(target_pos, loop_end - target_pos);

    let bytecode = builder.build();

    println!("=== test_factorial_loop bytecode ===");
    print_bytecode(&bytecode);
    println!();

    vm.eval_program(&bytecode).unwrap();
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

    vm.eval_program(&bytecode).unwrap();
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

    let result = vm.eval_program(&bytecode);
    assert!(matches!(result, Err(VmError::InvalidOpcode(0xFF))));
}

#[test]
fn test_unexpected_end_of_program() {
    let mut vm = VirtualMachine::new();
    let bytecode = vec![LOAD_I64, 1]; // Missing the i64 value

    println!("=== test_unexpected_end_of_program bytecode ===");
    print_bytecode(&bytecode);
    println!();

    let result = vm.eval_program(&bytecode);
    assert!(matches!(result, Err(VmError::UnexpectedEndOfProgram)));
}

#[test]
fn test_invalid_jump_target() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.load_i64(1, 0);
    builder.jmp(1000); // Invalid target - beyond program length
    let bytecode = builder.build();

    println!("=== test_invalid_jump_target bytecode ===");
    print_bytecode(&bytecode);
    println!();

    let result = vm.eval_program(&bytecode);
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

    vm.eval_program(&bytecode).unwrap();
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

    vm.eval_program(&bytecode).unwrap();
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

    vm.eval_program(&bytecode).unwrap();
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

    vm.eval_program(&bytecode).unwrap();
    assert_eq!(vm.get_register_i64(0), 1); // 7.0 > 5.0 is true
    assert!((vm.get_register_f64(3) - 7.0).abs() < f64::EPSILON);
}
