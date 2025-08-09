use vec_host::*;

#[test]
fn vec_lifecycle_and_ops() {
    // new
    let mut regs_new = vec![0u64; 1];
    assert_eq!(vec_host_new(&mut regs_new), Ok(()));
    let ptr = regs_new[0];
    assert_ne!(ptr, 0);

    // len == 0
    let mut regs_len = vec![0u64, ptr];
    assert_eq!(vec_host_len(&mut regs_len), Ok(()));
    assert_eq!(regs_len[0], 0);

    // append 10
    let mut regs_append = vec![0u64, ptr, 10];
    assert_eq!(vec_host_append(&mut regs_append), Ok(()));

    // len == 1
    let mut regs_len2 = vec![0u64, ptr];
    assert_eq!(vec_host_len(&mut regs_len2), Ok(()));
    assert_eq!(regs_len2[0], 1);

    // get index 0 == 10
    let mut regs_get = vec![0u64, ptr, 0];
    assert_eq!(vec_host_get(&mut regs_get), Ok(()));
    assert_eq!(regs_get[0], 10);

    // set index 0 = 20
    let mut regs_set = vec![0u64, ptr, 0, 20];
    assert_eq!(vec_host_set(&mut regs_set), Ok(()));

    // get index 0 == 20
    let mut regs_get2 = vec![0u64, ptr, 0];
    assert_eq!(vec_host_get(&mut regs_get2), Ok(()));
    assert_eq!(regs_get2[0], 20);

    // drop
    let mut regs_drop = vec![0u64, ptr];
    assert_eq!(vec_host_drop(&mut regs_drop), Ok(()));
}

#[test]
fn vec_get_out_of_bounds_returns_err() {
    // new
    let mut regs_new = vec![0u64; 1];
    assert_eq!(vec_host_new(&mut regs_new), Ok(()));
    let ptr = regs_new[0];

    // get index 1 (out of bounds)
    let mut regs_get = vec![0u64, ptr, 1];
    assert!(vec_host_get(&mut regs_get).is_err());

    // cleanup
    let mut regs_drop = vec![0u64, ptr];
    assert_eq!(vec_host_drop(&mut regs_drop), Ok(()));
}
