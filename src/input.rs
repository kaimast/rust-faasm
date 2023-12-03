use std::ffi::CString;

use faasm_sys::{__faasm_read_input, __faasm_write_output};

fn get_input_size() -> usize {
    // Passing zero buffer len returns total size
    let mut buf = [0u8; 1];
    unsafe { __faasm_read_input(buf.as_mut_ptr(), 0) }
        .try_into()
        .expect("Failed to get input size")
}

pub fn get_input() -> Vec<u8> {
    let len = get_input_size();

    if len == 0 {
        return vec![];
    }

    let mut input = vec![0u8; len];
    unsafe { __faasm_read_input(input.as_mut_ptr(), len as i32) };

    input
}

pub fn set_output(output: &str) {
    let len = output.len() as i32;
    let output = CString::new(output).expect("Not a valid string");
    unsafe { __faasm_write_output(output.as_bytes().as_ptr(), len) }
}
