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
    let last_fixed = Registers::FIXED_COUNT - 1;
    let first_spill = Registers::FIXED_COUNT;
    regs.set(last_fixed, 1);
    regs.set(first_spill, 2);
    assert_eq!(regs.get(last_fixed), 1);
    assert_eq!(regs.get(first_spill), 2);
}

#[test]
fn overflow_starting_vec_allocation() {
    let mut regs = Registers::new();
    // Fill up to the end of the initial spill capacity
    let last_initial_spill = Registers::FIXED_COUNT + Registers::SPILL_INIT - 1;
    regs.set(last_initial_spill, 3);
    // This should grow the spill vec beyond its initial allocation
    let beyond_initial_spill = Registers::FIXED_COUNT + Registers::SPILL_INIT;
    regs.set(beyond_initial_spill, 4);
    assert_eq!(regs.get(last_initial_spill), 3);
    assert_eq!(regs.get(beyond_initial_spill), 4);
}

#[test]
fn default_zero_after_growth() {
    let mut regs = Registers::new();
    let beyond_initial_spill = Registers::FIXED_COUNT + Registers::SPILL_INIT;
    regs.set(beyond_initial_spill, 7);
    // an unset register after growth should still read as zero
    let unset = Registers::FIXED_COUNT + 144;
    assert_eq!(regs.get(unset), 0);
}
