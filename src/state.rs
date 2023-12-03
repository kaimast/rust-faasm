use std::ffi::CString;

use faasm_sys::*;

fn read_state_size(key: &CString) -> usize {
    unsafe { __faasm_read_state(key.as_ptr(), std::ptr::null_mut(), 0) }
        .try_into()
        .expect("Failed to read state size")
}

/// Read data located at the particular key
pub fn read_state(key: &str) -> Option<Vec<u8>> {
    let key = CString::new(key).expect("Not a valid string");
    let len = read_state_size(&key);

    if len == 0 {
        return None;
    }

    let mut buffer = vec![0u8; len];
    unsafe { __faasm_read_state(key.as_ptr(), buffer.as_mut_ptr(), len as i32) };

    Some(buffer)
}

/// Read padded data from disk
/// Will return None (if no entry exists) or some vector of size <= total_size
pub fn read_state_with_padding(key: &str, total_size: usize) -> Option<Vec<u8>> {
    let key = CString::new(key).expect("Not a valid string");
    let real_len = read_state_size(&key);

    if real_len == 0 {
        // Does not exist yet
        return None;
    } else if real_len != total_size {
        panic!("Size does not match padding");
    }

    let mut buffer = vec![0u8; total_size];
    unsafe { __faasm_read_state(key.as_ptr(), buffer.as_mut_ptr(), total_size as i32) };

    const INNER_LEN_LEN: usize = std::mem::size_of::<u32>();
    let len_data: [u8; INNER_LEN_LEN] = buffer[0..INNER_LEN_LEN].try_into().unwrap();
    let inner_len = u32::from_le_bytes(len_data);

    if inner_len == 0 {
        return None;
    }

    let mut result = vec![0u8; inner_len as usize];
    result.copy_from_slice(&buffer[INNER_LEN_LEN..INNER_LEN_LEN + (inner_len as usize)]);

    Some(result)
}

/// Write data to the specified key
pub fn write_state(key: &str, value: &[u8]) {
    let key = CString::new(key).expect("Not a valid string");
    unsafe { __faasm_write_state(key.as_ptr(), value.as_ptr(), value.len() as i32) };
}

/// Writes an entry and allocates extra padding
/// This allows to later overwrite with a larger entry
///
/// Note: You must ensure that value.len()+size_of(u32) <= total_size
pub fn write_state_with_padding(key: &str, value: &[u8], total_size: usize) {
    let key = CString::new(key).expect("Not a valid string");

    const INNER_LEN_LEN: usize = std::mem::size_of::<u32>();
    if total_size - INNER_LEN_LEN < value.len() {
        panic!("Out of bounds!");
    }

    let mut buffer = vec![0u8; total_size];
    let inner_len = value.len() as u32;
    buffer[0..INNER_LEN_LEN].copy_from_slice(&inner_len.to_le_bytes());
    buffer[INNER_LEN_LEN..INNER_LEN_LEN + (inner_len as usize)].copy_from_slice(value);

    unsafe { __faasm_write_state(key.as_ptr(), buffer.as_ptr(), total_size as i32) };
}

/// Push state from the local store to the global one
pub fn push_state(key: &str) {
    let key = CString::new(key).expect("Not a valid string");
    unsafe { __faasm_push_state(key.as_ptr()) };
}

/// Lock a key for exclusive access
pub fn lock_state_write(key: &str) {
    let key = CString::new(key).expect("Not a valid string");
    unsafe { __faasm_lock_state_write(key.as_ptr()) };
}

/// Lock a key for concurrent read-only access
pub fn lock_state_read(key: &str) {
    let key = CString::new(key).expect("Not a valid string");
    unsafe { __faasm_lock_state_read(key.as_ptr()) };
}

pub fn unlock_state_write(key: &str) {
    let key = CString::new(key).expect("Not a valid string");
    unsafe { __faasm_unlock_state_write(key.as_ptr()) };
}

pub fn unlock_state_read(key: &str) {
    let key = CString::new(key).expect("Not a valid string");
    unsafe { __faasm_unlock_state_read(key.as_ptr()) };
}
