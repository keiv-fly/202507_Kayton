use super::const_pool::{SliceType, ValueType};
use super::global_vars::{GlobalVars, GlobalVarType, PtrType};
use super::VirtualMachine;

#[test]
fn test_insert_and_get_value_var() {
    let mut gv = GlobalVars::new();
    gv.insert("counter", 1, GlobalVarType::Value(ValueType::I64));
    let var = gv.get("counter").expect("var exists");
    assert_eq!(var.register_id, 1);
    assert!(matches!(var.meta.typ, GlobalVarType::Value(ValueType::I64)));
}

#[test]
fn test_insert_and_get_ptr_var() {
    let mut gv = GlobalVars::new();
    gv.insert(
        "greeting",
        2,
        GlobalVarType::Ptr(PtrType::Slice(SliceType::Utf8Str)),
    );
    let var = gv.get("greeting").unwrap();
    assert_eq!(var.register_id, 2);
    assert!(matches!(
        var.meta.typ,
        GlobalVarType::Ptr(PtrType::Slice(SliceType::Utf8Str))
    ));
}

#[test]
fn test_vm_has_global_vars() {
    let vm = VirtualMachine::new();
    assert!(vm.global_vars.is_empty());
}

