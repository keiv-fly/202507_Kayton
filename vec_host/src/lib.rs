use std::collections::HashMap;
use std::ptr::NonNull;

pub use kayton::vm::HostFunctionMetadata;

// We store heap-allocated Vec<u64> pointers in registers as u64
// Layout per call:
// base+0: return value(s) start
// base+1..: params

fn read_ptr(reg: u64) -> Result<NonNull<Vec<u64>>, String> {
    let ptr = reg as *mut Vec<u64>;
    NonNull::new(ptr).ok_or_else(|| "null pointer".to_string())
}

#[unsafe(no_mangle)]
pub fn vec_host_new(registers: &mut [u64]) -> Result<(), String> {
    let v: Box<Vec<u64>> = Box::new(Vec::new());
    let ptr = Box::into_raw(v) as u64;
    if let Some(r0) = registers.get_mut(0) {
        *r0 = ptr;
        Ok(())
    } else {
        // Can't return pointer
        unsafe {
            drop(Box::from_raw(ptr as *mut Vec<u64>));
        }
        Err("missing return register".to_string())
    }
}

#[unsafe(no_mangle)]
pub fn vec_host_drop(registers: &mut [u64]) -> Result<(), String> {
    if registers.len() < 2 {
        return Err("insufficient registers".to_string());
    }
    let ptr_val = registers[1];
    match read_ptr(ptr_val) {
        Ok(nn) => unsafe {
            drop(Box::from_raw(nn.as_ptr()));
        },
        Err(e) => return Err(e),
    }
    registers[0] = 0;
    Ok(())
}

// append(vec_ptr, value)
#[unsafe(no_mangle)]
pub fn vec_host_append(registers: &mut [u64]) -> Result<(), String> {
    if registers.len() < 3 {
        return Err("insufficient registers".to_string());
    }
    let ptr_val = registers[1];
    let value = registers[2];
    let nn = match read_ptr(ptr_val) {
        Ok(nn) => nn,
        Err(e) => return Err(e),
    };
    unsafe {
        (*nn.as_ptr()).push(value);
    }
    registers[0] = 0;
    Ok(())
}

// get(vec_ptr, index) -> value
#[unsafe(no_mangle)]
pub fn vec_host_get(registers: &mut [u64]) -> Result<(), String> {
    if registers.len() < 3 {
        return Err("insufficient registers".to_string());
    }
    let ptr_val = registers[1];
    let index = registers[2] as usize;
    let nn = match read_ptr(ptr_val) {
        Ok(nn) => nn,
        Err(e) => return Err(e),
    };
    let value = unsafe { (*nn.as_ptr()).get(index).copied() };
    match value {
        Some(v) => {
            registers[0] = v;
            Ok(())
        }
        None => Err("index out of bounds".to_string()),
    }
}

// set(vec_ptr, index, value)
#[unsafe(no_mangle)]
pub fn vec_host_set(registers: &mut [u64]) -> Result<(), String> {
    if registers.len() < 4 {
        return Err("insufficient registers".to_string());
    }
    let ptr_val = registers[1];
    let index = registers[2] as usize;
    let value = registers[3];
    let nn = match read_ptr(ptr_val) {
        Ok(nn) => nn,
        Err(e) => return Err(e),
    };
    let vec_ref = unsafe { &mut *nn.as_ptr() };
    if index >= vec_ref.len() {
        return Err("index out of bounds".to_string());
    }
    vec_ref[index] = value;
    registers[0] = 0;
    Ok(())
}

// len(vec_ptr) -> len
#[unsafe(no_mangle)]
pub fn vec_host_len(registers: &mut [u64]) -> Result<(), String> {
    if registers.len() < 2 {
        return Err("insufficient registers".to_string());
    }
    let ptr_val = registers[1];
    let nn = match read_ptr(ptr_val) {
        Ok(nn) => nn,
        Err(e) => return Err(e),
    };
    let len = unsafe { (*nn.as_ptr()).len() as u64 };
    registers[0] = len;
    Ok(())
}

#[unsafe(no_mangle)]
pub fn vec_host_meta_data() -> HashMap<&'static str, HostFunctionMetadata> {
    let mut m = HashMap::new();
    // name, num_return_registers, num_params, num_registers
    m.insert(
        "vec_host_new",
        HostFunctionMetadata {
            name: "vec_host_new",
            num_return_registers: 1,
            num_params: 0,
            num_registers: 1,
        },
    );
    m.insert(
        "vec_host_drop",
        HostFunctionMetadata {
            name: "vec_host_drop",
            num_return_registers: 1,
            num_params: 1,
            num_registers: 2,
        },
    );
    m.insert(
        "vec_host_append",
        HostFunctionMetadata {
            name: "vec_host_append",
            num_return_registers: 1,
            num_params: 2,
            num_registers: 3,
        },
    );
    m.insert(
        "vec_host_get",
        HostFunctionMetadata {
            name: "vec_host_get",
            num_return_registers: 1,
            num_params: 2,
            num_registers: 3,
        },
    );
    m.insert(
        "vec_host_set",
        HostFunctionMetadata {
            name: "vec_host_set",
            num_return_registers: 1,
            num_params: 3,
            num_registers: 4,
        },
    );
    m.insert(
        "vec_host_len",
        HostFunctionMetadata {
            name: "vec_host_len",
            num_return_registers: 1,
            num_params: 1,
            num_registers: 2,
        },
    );
    m
}
