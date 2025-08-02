use super::const_pool::{ConstPool, SliceType, ValueType};

#[test]
fn test_add_and_get_value_constants() {
    let mut pool = ConstPool::new();

    let i: i64 = 42;
    let idx_i = pool.add_value("42", i as u64, ValueType::I64);
    assert_eq!(idx_i, 0);
    assert_eq!(pool.get_value("42"), Some(42u64));
    let meta_i = &pool.value_metadata[idx_i];
    assert_eq!(meta_i.name, "42");
    assert!(matches!(meta_i.typ, ValueType::I64));
    assert_eq!(meta_i.index, idx_i);

    let f: f64 = 3.14;
    let idx_f = pool.add_value("3.14", f.to_bits(), ValueType::F64);
    assert_eq!(idx_f, 1);
    assert_eq!(pool.get_value("3.14"), Some(f.to_bits()));
    let meta_f = &pool.value_metadata[idx_f];
    assert_eq!(meta_f.name, "3.14");
    assert!(matches!(meta_f.typ, ValueType::F64));
    assert_eq!(meta_f.index, idx_f);

    let b: bool = true;
    let idx_b = pool.add_value("true", b as u64, ValueType::Bool);
    assert_eq!(idx_b, 2);
    assert_eq!(pool.get_value("true"), Some(1u64));
    let meta_b = &pool.value_metadata[idx_b];
    assert_eq!(meta_b.name, "true");
    assert!(matches!(meta_b.typ, ValueType::Bool));
    assert_eq!(meta_b.index, idx_b);

    assert_eq!(pool.get_value("false"), None);
}

#[test]
fn test_add_and_get_slice_constants() {
    let mut pool = ConstPool::new();

    let text: &str = "hello";
    let idx_text = pool.add_slice("hello", text.as_bytes(), SliceType::Utf8Str);
    assert_eq!(idx_text, 0);
    assert_eq!(pool.get_slice("hello"), Some(text.as_bytes()));
    let meta_text = &pool.slice_metadata[idx_text];
    assert_eq!(meta_text.name, "hello");
    assert!(matches!(meta_text.typ, SliceType::Utf8Str));
    assert_eq!(meta_text.index, idx_text);

    let binary: &[u8] = b"bin";
    let idx_bin = pool.add_slice("bin", binary, SliceType::Binary);
    assert_eq!(idx_bin, 1);
    assert_eq!(pool.get_slice("bin"), Some(binary));
    let meta_bin = &pool.slice_metadata[idx_bin];
    assert_eq!(meta_bin.name, "bin");
    assert!(matches!(meta_bin.typ, SliceType::Binary));
    assert_eq!(meta_bin.index, idx_bin);

    assert_eq!(pool.get_slice("missing"), None);
}

