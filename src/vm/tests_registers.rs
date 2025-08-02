use super::registers::Registers;

#[test]
fn basic_get_set() {
    let mut regs = Registers::new();
    assert_eq!(regs.get(0), 0);
    regs.set(0, 42);
    assert_eq!(regs.get(0), 42);
}

#[test]
fn overflow_fixed_register() {
    let mut regs = Registers::new();
    regs.set(255, 1);
    regs.set(256, 2);
    assert_eq!(regs.get(255), 1);
    assert_eq!(regs.get(256), 2);
}

#[test]
fn overflow_starting_vec_allocation() {
    let mut regs = Registers::new();
    // Fill up to the end of the initial spill capacity
    regs.set(511, 3);
    // This should grow the spill vec beyond its initial allocation
    regs.set(512, 4);
    assert_eq!(regs.get(511), 3);
    assert_eq!(regs.get(512), 4);
}

#[test]
fn default_zero_after_growth() {
    let mut regs = Registers::new();
    regs.set(512, 7);
    // an unset register after growth should still read as zero
    assert_eq!(regs.get(400), 0);
}
