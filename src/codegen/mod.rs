use crate::parser::{Expr, Stmt, BinOp};
use crate::vm::{BytecodeBuilder, VirtualMachine};
use crate::vm::const_pool::{SliceType, ValueType};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum ValueKind {
    Int,
    Str,
}

struct CodeGenerator<'a> {
    builder: BytecodeBuilder,
    vars: HashMap<String, u8>,
    types: HashMap<String, ValueKind>,
    next_reg: u8,
    vm: &'a mut VirtualMachine,
    print_const: u16,
}

impl<'a> CodeGenerator<'a> {
    fn new(vm: &'a mut VirtualMachine, print_const: u16) -> Self {
        Self {
            builder: BytecodeBuilder::new(),
            vars: HashMap::new(),
            types: HashMap::new(),
            next_reg: 1, // reserve register 0 for call base
            vm,
            print_const,
        }
    }

    fn compile(mut self, stmts: &[Stmt]) -> Vec<u8> {
        for stmt in stmts {
            self.gen_stmt(stmt);
        }
        self.builder.build()
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign { name, expr } => {
                let reg = *self.vars.entry(name.clone()).or_insert_with(|| {
                    let r = self.next_reg;
                    self.next_reg += 1;
                    r
                });
                let (_r, kind) = self.gen_expr(expr, Some(reg));
                self.types.insert(name.clone(), kind);
            }
            Stmt::ExprStmt(expr) => {
                if let Expr::Call { func, args } = expr {
                    if let Expr::Ident(fname) = &**func {
                        if fname == "print" && args.len() == 1 {
                            self.gen_print(&args[0]);
                            return;
                        }
                    }
                    panic!("unsupported call expression");
                } else {
                    self.gen_expr(expr, None);
                }
            }
        }
    }

    fn gen_print(&mut self, arg: &Expr) {
        let (reg, kind) = self.gen_expr(arg, None);
        let base = reg - 1;
        if self.next_reg <= base + 2 {
            self.next_reg = base + 3;
        }
        self.builder.load_const_value(self.print_const, base);
        if kind == ValueKind::Int {
            let zero_idx = self.vm.const_pool.add_value("", 0, ValueType::I64) as u16;
            self.builder.load_const_value(zero_idx, base + 2);
        }
        self.builder.call_host(base as u16);
    }

    fn gen_expr(&mut self, expr: &Expr, target: Option<u8>) -> (u8, ValueKind) {
        match expr {
            Expr::Int(n) => {
                let reg = target.unwrap_or_else(|| {
                    let r = self.next_reg;
                    self.next_reg += 1;
                    r
                });
                let idx = self
                    .vm
                    .const_pool
                    .add_value("", *n as u64, ValueType::I64) as u16;
                self.builder.load_const_value(idx, reg);
                (reg, ValueKind::Int)
            }
            Expr::Str(s) => {
                let reg = target.unwrap_or_else(|| {
                    let r = self.next_reg;
                    self.next_reg += 2;
                    r
                });
                if target.is_some() && self.next_reg <= reg + 1 {
                    self.next_reg = reg + 2;
                }
                let idx = self
                    .vm
                    .const_pool
                    .add_slice("", s.as_bytes(), SliceType::Utf8Str) as u16;
                self.builder.load_const_slice(idx, reg);
                (reg, ValueKind::Str)
            }
            Expr::Ident(name) => {
                let reg = *self
                    .vars
                    .get(name)
                    .expect("undefined variable");
                let kind = *self
                    .types
                    .get(name)
                    .expect("unknown type");
                (reg, kind)
            }
            Expr::Binary { left, op: BinOp::Add, right } => {
                let (lreg, _) = self.gen_expr(left, target);
                let (rreg, _) = self.gen_expr(right, None);
                let dst = target.unwrap_or(lreg);
                self.builder.add_i64(lreg, rreg, dst);
                (dst, ValueKind::Int)
            }
            Expr::Call { .. } => panic!("call expressions not supported here"),
            Expr::InterpolatedString(_) => unimplemented!("f-strings not supported"),
        }
    }
}

pub fn generate_bytecode(
    stmts: &[Stmt],
    vm: &mut VirtualMachine,
    print_const: u16,
) -> Vec<u8> {
    CodeGenerator::new(vm, print_const).compile(stmts)
}

#[cfg(test)]
mod tests;
