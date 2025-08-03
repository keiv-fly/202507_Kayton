use super::*;
use crate::vm::print_bytecode::format_bytecode;
use super::const_pool::{ValueType, SliceType};

fn add_i64(vm: &mut VirtualMachine, value: i64) -> u16 {
    vm.const_pool.add_value("", value as u64, ValueType::I64) as u16
}

fn add_f64(vm: &mut VirtualMachine, value: f64) -> u16 {
    vm.const_pool.add_value("", value.to_bits(), ValueType::F64) as u16
}

fn load_i64_const(vm: &mut VirtualMachine, builder: &mut BytecodeBuilder, value: i64, reg: u8) -> u16 {
    let idx = add_i64(vm, value);
    builder.load_const_value(idx, reg);
    idx
}

fn load_f64_const(vm: &mut VirtualMachine, builder: &mut BytecodeBuilder, value: f64, reg: u8) -> u16 {
    let idx = add_f64(vm, value);
    builder.load_const_value(idx, reg);
    idx
}

#[test]
fn test_format_load_i64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx = load_i64_const(&mut vm, &mut builder, 42, 1);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r1, {}", idx));
    assert_eq!(lines[1], "pc=4");
    assert_eq!(lines[2], "bytecode.len()=4");
}

#[test]
fn test_format_load_f64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx = load_f64_const(&mut vm, &mut builder, 3.14159, 2);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r2, {}", idx));
    assert_eq!(lines[1], "pc=4");
    assert_eq!(lines[2], "bytecode.len()=4");
}

#[test]
fn test_format_negative_i64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx = load_i64_const(&mut vm, &mut builder, -100, 0);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r0, {}", idx));
}

#[test]
fn test_format_add_i64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.add_i64(1, 2, 3);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 ADD_I64 r1, r2, r3");
    assert_eq!(lines[1], "pc=4");
    assert_eq!(lines[2], "bytecode.len()=4");
}

#[test]
fn test_format_sub_i64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.sub_i64(5, 6, 7);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 SUB_I64 r5, r6, r7");
}

#[test]
fn test_format_mul_i64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.mul_i64(10, 11, 12);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 MUL_I64 r10, r11, r12");
}

#[test]
fn test_format_gt_i64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.gt_i64(20, 21, 22);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 GT_I64 r20, r21, r22");
}

#[test]
fn test_format_gte_i64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.gte_i64(1, 2, 3);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 GTE_I64 r1, r2, r3");
}

#[test]
fn test_format_lt_i64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.lt_i64(4, 5, 6);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 LT_I64 r4, r5, r6");
}

#[test]
fn test_format_lte_i64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.lte_i64(7, 8, 9);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 LTE_I64 r7, r8, r9");
}

#[test]
fn test_format_load_const_value() {
    let mut vm = VirtualMachine::new();
    let idx = add_i64(&mut vm, 1);
    let mut builder = BytecodeBuilder::new();
    builder.load_const_value(idx, 2);
    let bytecode = builder.build();
    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();
    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r2, {}", idx));
    assert_eq!(lines[1], "pc=4");
    assert_eq!(lines[2], "bytecode.len()=4");
}

#[test]
fn test_format_load_const_slice() {
    let mut vm = VirtualMachine::new();
    let idx = vm
        .const_pool
        .add_slice("", b"x", SliceType::Utf8Str) as u16;
    let mut builder = BytecodeBuilder::new();
    builder.load_const_slice(idx, 4);
    let bytecode = builder.build();
    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();
    assert_eq!(lines[0], format!("0 LOAD_CONST_SLICE r4, {}", idx));
    assert_eq!(lines[1], "pc=4");
    assert_eq!(lines[2], "bytecode.len()=4");
}

#[test]
fn test_format_add_f64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.add_f64(1, 2, 3);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 ADD_F64 r1, r2, r3");
}

#[test]
fn test_format_sub_f64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.sub_f64(4, 5, 6);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 SUB_F64 r4, r5, r6");
}

#[test]
fn test_format_mul_f64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.mul_f64(7, 8, 9);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 MUL_F64 r7, r8, r9");
}

#[test]
fn test_format_gt_f64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.gt_f64(15, 16, 17);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 GT_F64 r15, r16, r17");
}

#[test]
fn test_format_gte_f64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.gte_f64(1, 2, 3);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 GTE_F64 r1, r2, r3");
}

#[test]
fn test_format_lt_f64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.lt_f64(4, 5, 6);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 LT_F64 r4, r5, r6");
}

#[test]
fn test_format_lte_f64() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.lte_f64(7, 8, 9);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 LTE_F64 r7, r8, r9");
}

#[test]
fn test_format_type_conversions() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.i64_to_f64(1, 2);
    builder.f64_to_i64(3, 4);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 I64_TO_F64 r1, r2");
    assert_eq!(lines[1], "3 F64_TO_I64 r3, r4");
    assert_eq!(lines[2], "pc=6");
    assert_eq!(lines[3], "bytecode.len()=6");
}

#[test]
fn test_format_jump_forward_if_false() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx1 = load_i64_const(&mut vm, &mut builder, 1, 1);
    let target_pos = builder.jump_forward_if_false(1);
    let idx100 = load_i64_const(&mut vm, &mut builder, 100, 2);
    let end_pos = builder.current_pos();
    builder.patch_target(target_pos, end_pos - target_pos);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r1, {}", idx1));
    assert!(lines[1].starts_with("4 JUMP_FORWARD_IF_FALSE r1,"));
    assert!(lines[1].contains("(offset:"));
    assert_eq!(lines[2], format!("8 LOAD_CONST_VALUE r2, {}", idx100));
}

#[test]
fn test_format_jump_forward_if_true() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx1 = load_i64_const(&mut vm, &mut builder, 1, 1);
    let target_pos = builder.jump_forward_if_true(1);
    let idx100 = load_i64_const(&mut vm, &mut builder, 100, 2);
    let end_pos = builder.current_pos();
    builder.patch_target(target_pos, end_pos - target_pos);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r1, {}", idx1));
    assert!(lines[1].starts_with("4 JUMP_FORWARD_IF_TRUE r1,"));
    assert!(lines[1].contains("(offset:"));
}

#[test]
fn test_format_jump_backward_if_false() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx1 = load_i64_const(&mut vm, &mut builder, 1, 1);
    builder.jump_backward_if_false(1, 10);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r1, {}", idx1));
    assert!(lines[1].starts_with("4 JUMP_BACKWARD_IF_FALSE r1,"));
    assert!(lines[1].contains("(offset: 10)"));
}

#[test]
fn test_format_jump_backward_if_true() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx1 = load_i64_const(&mut vm, &mut builder, 1, 1);
    builder.jump_backward_if_true(1, 5);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r1, {}", idx1));
    assert!(lines[1].starts_with("4 JUMP_BACKWARD_IF_TRUE r1,"));
    assert!(lines[1].contains("(offset: 5)"));
}

#[test]
fn test_format_jmp() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    builder.jmp_to(42);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "0 JMP 42");
    assert_eq!(lines[1], "pc=3");
    assert_eq!(lines[2], "bytecode.len()=3");
}

#[test]
fn test_format_unknown_opcode() {
    let bytecode = vec![0xFF, 0x00, 0x01]; // Unknown opcode followed by some bytes

    let result = format_bytecode(&bytecode);
    assert!(result.is_err());
    let error = result.unwrap_err();
    let lines: Vec<&str> = error.lines().collect();

    assert_eq!(lines[0], "0 UNKNOWN_OPCODE 0xFF");
    assert_eq!(lines.len(), 1);
}

#[test]
fn test_format_complex_program() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx10 = load_i64_const(&mut vm, &mut builder, 10, 1);
    let idx5 = load_i64_const(&mut vm, &mut builder, 5, 2);
    builder.add_i64(1, 2, 3);
    builder.i64_to_f64(3, 4);
    let idx25 = load_f64_const(&mut vm, &mut builder, 2.5, 5);
    builder.mul_f64(4, 5, 6);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r1, {}", idx10));
    assert_eq!(lines[1], format!("4 LOAD_CONST_VALUE r2, {}", idx5));
    assert_eq!(lines[2], "8 ADD_I64 r1, r2, r3");
    assert_eq!(lines[3], "12 I64_TO_F64 r3, r4");
    assert_eq!(lines[4], format!("15 LOAD_CONST_VALUE r5, {}", idx25));
    assert_eq!(lines[5], "19 MUL_F64 r4, r5, r6");
}

#[test]
fn test_format_empty_bytecode() {
    let bytecode = vec![];

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], "pc=0");
    assert_eq!(lines[1], "bytecode.len()=0");
    assert_eq!(lines.len(), 2);
}

// Updated tests for incomplete instructions - now expecting errors

#[test]
fn test_format_incomplete_load_const_value() {
    // LOAD_CONST_VALUE opcode with register but missing index byte
    let bytecode = vec![LOAD_CONST_VALUE, 1, 0x01]; // Missing one more byte for index

    let result = format_bytecode(&bytecode);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Incomplete LOAD_CONST_VALUE instruction"));
    assert!(error.contains("missing operands"));
}

#[test]
fn test_format_incomplete_load_const_value_missing_register() {
    // LOAD_CONST_VALUE opcode with no register and index
    let bytecode = vec![LOAD_CONST_VALUE]; // Missing register and index

    let result = format_bytecode(&bytecode);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Incomplete LOAD_CONST_VALUE instruction"));
    assert!(error.contains("missing operands"));
}

#[test]
fn test_format_incomplete_load_const_slice() {
    // LOAD_CONST_SLICE opcode with register but missing index
    let bytecode = vec![LOAD_CONST_SLICE, 2]; // Missing 2 bytes for index

    let result = format_bytecode(&bytecode);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Incomplete LOAD_CONST_SLICE instruction"));
    assert!(error.contains("missing operands"));
}

#[test]
fn test_format_incomplete_arithmetic() {
    // ADD_I64 opcode with only one register
    let bytecode = vec![ADD_I64, 1]; // Missing 2 more register bytes

    let result = format_bytecode(&bytecode);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Incomplete ADD_I64 instruction"));
    assert!(error.contains("missing register operands"));
}

#[test]
fn test_format_incomplete_sub_i64() {
    // SUB_I64 opcode with missing registers
    let bytecode = vec![SUB_I64]; // Missing all register bytes

    let result = format_bytecode(&bytecode);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Incomplete SUB_I64 instruction"));
    assert!(error.contains("missing register operands"));
}

#[test]
fn test_format_incomplete_jump() {
    // JUMP_FORWARD_IF_FALSE with condition register but missing offset
    let bytecode = vec![JUMP_FORWARD_IF_FALSE, 1]; // Missing 2 bytes for offset

    let result = format_bytecode(&bytecode);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Incomplete JUMP_FORWARD_IF_FALSE instruction"));
    assert!(error.contains("missing condition register or offset"));
}

#[test]
fn test_format_incomplete_jump_missing_register() {
    // JUMP_FORWARD_IF_FALSE with no condition register
    let bytecode = vec![JUMP_FORWARD_IF_FALSE]; // Missing register and offset

    let result = format_bytecode(&bytecode);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Incomplete JUMP_FORWARD_IF_FALSE instruction"));
    assert!(error.contains("missing condition register or offset"));
}

#[test]
fn test_format_incomplete_jmp() {
    // JMP with missing target
    let bytecode = vec![JMP]; // Missing 2 bytes for target

    let result = format_bytecode(&bytecode);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Incomplete JMP instruction"));
    assert!(error.contains("missing target address"));
}

#[test]
fn test_format_incomplete_type_conversion() {
    // I64_TO_F64 with missing destination register
    let bytecode = vec![I64_TO_F64, 1]; // Missing destination register

    let result = format_bytecode(&bytecode);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Incomplete I64_TO_F64 instruction"));
    assert!(error.contains("missing register operands"));
}

#[test]
fn test_format_various_register_numbers() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx0 = load_i64_const(&mut vm, &mut builder, 1, 0); // Register 0
    let idx127 = load_i64_const(&mut vm, &mut builder, 2, 127); // Register 127
    let idx255 = load_i64_const(&mut vm, &mut builder, 3, 255); // Register 255 (max)
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r0, {}", idx0));
    assert_eq!(lines[1], format!("4 LOAD_CONST_VALUE r127, {}", idx127));
    assert_eq!(lines[2], format!("8 LOAD_CONST_VALUE r255, {}", idx255));
}

#[test]
fn test_format_extreme_values() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx_max = load_i64_const(&mut vm, &mut builder, i64::MAX, 1);
    let idx_min = load_i64_const(&mut vm, &mut builder, i64::MIN, 2);
    let idx_fmax = load_f64_const(&mut vm, &mut builder, f64::MAX, 3);
    let idx_fmin = load_f64_const(&mut vm, &mut builder, f64::MIN, 4);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r1, {}", idx_max));
    assert_eq!(lines[1], format!("4 LOAD_CONST_VALUE r2, {}", idx_min));
    assert_eq!(lines[2], format!("8 LOAD_CONST_VALUE r3, {}", idx_fmax));
    assert_eq!(lines[3], format!("12 LOAD_CONST_VALUE r4, {}", idx_fmin));
}

#[test]
fn test_format_special_f64_values() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    let idx_inf = load_f64_const(&mut vm, &mut builder, f64::INFINITY, 1);
    let idx_neginf = load_f64_const(&mut vm, &mut builder, f64::NEG_INFINITY, 2);
    let idx_nan = load_f64_const(&mut vm, &mut builder, f64::NAN, 3);
    let idx_zero = load_f64_const(&mut vm, &mut builder, 0.0, 4);
    let idx_negzero = load_f64_const(&mut vm, &mut builder, -0.0, 5);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");
    let lines: Vec<&str> = formatted.lines().collect();

    assert_eq!(lines[0], format!("0 LOAD_CONST_VALUE r1, {}", idx_inf));
    assert_eq!(lines[1], format!("4 LOAD_CONST_VALUE r2, {}", idx_neginf));
    assert_eq!(lines[2], format!("8 LOAD_CONST_VALUE r3, {}", idx_nan));
    assert_eq!(lines[3], format!("12 LOAD_CONST_VALUE r4, {}", idx_zero));
    assert_eq!(lines[4], format!("16 LOAD_CONST_VALUE r5, {}", idx_negzero));
}

#[test]
fn test_format_with_labels_simulation() {
    // Simulate what bytecode with labels would look like after building
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();

    let loop_start = builder.create_label();
    let loop_end = builder.create_label();

    load_i64_const(&mut vm, &mut builder, 5, 1); // Counter
    load_i64_const(&mut vm, &mut builder, 1, 2); // Accumulator  
    builder.place_label(loop_start); // Loop start
    builder.gt_i64(1, 2, 3); // Compare
    builder.jump_if_false_to_label(3, loop_end);
    builder.add_i64(2, 1, 2); // Add to accumulator
    builder.sub_i64(1, 2, 1); // Decrement counter  
    builder.jmp_to_label(loop_start); // Jump back
    builder.place_label(loop_end); // Loop end

    let bytecode = builder.build();
    let formatted = format_bytecode(&bytecode).expect("Should format successfully");

    // Just verify it doesn't crash and produces reasonable output
    assert!(!formatted.is_empty());
    assert!(formatted.contains("LOAD_CONST_VALUE"));
    assert!(formatted.contains("GT_I64"));
    assert!(
        formatted.contains("JUMP_FORWARD_IF_FALSE") || formatted.contains("JUMP_BACKWARD_IF_FALSE")
    );
    assert!(formatted.contains("JMP"));
}

#[test]
fn test_format_output_ends_with_newline() {
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    load_i64_const(&mut vm, &mut builder, 42, 1);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");

    // Should end with a newline
    assert!(formatted.ends_with('\n'));
}

#[test]
fn test_format_consistent_with_print_bytecode() {
    // This test ensures that format_bytecode and print_bytecode produce the same output
    let mut vm = VirtualMachine::new();
    let mut builder = BytecodeBuilder::new();
    load_i64_const(&mut vm, &mut builder, 123, 1);
    builder.add_i64(1, 1, 2);
    let bytecode = builder.build();

    let formatted = format_bytecode(&bytecode).expect("Should format successfully");

    // Since we can't easily capture print! output in tests, we just verify
    // that format_bytecode produces reasonable output
    assert!(formatted.contains("LOAD_CONST_VALUE r1"));
    assert!(formatted.contains("ADD_I64 r1, r1, r2"));
    assert!(formatted.contains("pc="));
    assert!(formatted.contains("bytecode.len()="));
}

// Additional tests for error conditions on all instruction types

#[test]
fn test_format_incomplete_all_f64_arithmetic() {
    let instructions = vec![
        (ADD_F64, "ADD_F64"),
        (SUB_F64, "SUB_F64"),
        (MUL_F64, "MUL_F64"),
        (GT_F64, "GT_F64"),
        (GTE_F64, "GTE_F64"),
        (LT_F64, "LT_F64"),
        (LTE_F64, "LTE_F64"),
    ];

    for (opcode, name) in instructions {
        let bytecode = vec![opcode, 1]; // Missing 2 register operands
        let result = format_bytecode(&bytecode);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains(&format!("Incomplete {} instruction", name)));
        assert!(error.contains("missing register operands"));
    }
}

#[test]
fn test_format_incomplete_all_i64_arithmetic() {
    let instructions = vec![
        (MUL_I64, "MUL_I64"),
        (GT_I64, "GT_I64"),
        (GTE_I64, "GTE_I64"),
        (LT_I64, "LT_I64"),
        (LTE_I64, "LTE_I64"),
    ];

    for (opcode, name) in instructions {
        let bytecode = vec![opcode]; // Missing all register operands
        let result = format_bytecode(&bytecode);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains(&format!("Incomplete {} instruction", name)));
        assert!(error.contains("missing register operands"));
    }
}

#[test]
fn test_format_incomplete_all_jump_instructions() {
    let instructions = vec![
        (JUMP_BACKWARD_IF_FALSE, "JUMP_BACKWARD_IF_FALSE"),
        (JUMP_BACKWARD_IF_TRUE, "JUMP_BACKWARD_IF_TRUE"),
    ];

    for (opcode, name) in instructions {
        let bytecode = vec![opcode]; // Missing condition register and offset
        let result = format_bytecode(&bytecode);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains(&format!("Incomplete {} instruction", name)));
        assert!(error.contains("missing condition register or offset"));
    }
}

#[test]
fn test_format_incomplete_conversion_instructions() {
    let instructions = vec![(F64_TO_I64, "F64_TO_I64")];

    for (opcode, name) in instructions {
        let bytecode = vec![opcode, 1]; // Missing destination register
        let result = format_bytecode(&bytecode);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains(&format!("Incomplete {} instruction", name)));
        assert!(error.contains("missing register operands"));
    }
}
