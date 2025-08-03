use super::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::{Registers, VirtualMachine};
use crate::vm::const_pool::ValueType;
use std::sync::{Mutex, OnceLock};

static OUTPUT: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
fn output() -> &'static Mutex<Vec<String>> {
    OUTPUT.get_or_init(|| Mutex::new(Vec::new()))
}

static TEST_MUTEX: Mutex<()> = Mutex::new(());

fn host_print(base: usize, registers: &mut Registers) -> Result<(), String> {
    let val = registers.get(base + 1);
    let len = registers.get(base + 2);
    if len == 0 {
        output().lock().unwrap().push(format!("{}", val as i64));
    } else {
        unsafe {
            let slice = std::slice::from_raw_parts(val as *const u8, len as usize);
            let s = String::from_utf8(slice.to_vec()).map_err(|e| e.to_string())?;
            output().lock().unwrap().push(s);
        }
    }
    Ok(())
}

fn setup_vm() -> (VirtualMachine, u16) {
    let mut vm = VirtualMachine::new();
    let print_idx = vm
        .host_functions
        .register("print", 0, 1, 3, host_print);
    let const_idx = vm
        .const_pool
        .add_value("", print_idx as u64, ValueType::FuncHost) as u16;
    (vm, const_idx)
}

#[test]
fn program1_codegen() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let src = r#"x = 12
x = x + 1
print(x)
"#;
    let tokens = Lexer::new(src).tokenize();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse_program();

    let (mut vm, print_const) = setup_vm();
    output().lock().unwrap().clear();
    let bytecode = generate_bytecode(&stmts, &mut vm, print_const);
    vm.eval_program(&bytecode).unwrap();

    let out = output().lock().unwrap().clone();
    assert_eq!(out, vec!["13".to_string()]);
}

#[test]
fn program2_codegen() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let src = r#"print("Hello, World")"#;
    let tokens = Lexer::new(src).tokenize();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse_program();

    let (mut vm, print_const) = setup_vm();
    output().lock().unwrap().clear();
    let bytecode = generate_bytecode(&stmts, &mut vm, print_const);
    vm.eval_program(&bytecode).unwrap();

    let out = output().lock().unwrap().clone();
    assert_eq!(out, vec!["Hello, World".to_string()]);
}
