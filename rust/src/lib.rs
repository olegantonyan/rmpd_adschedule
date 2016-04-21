extern crate libc;
extern crate rustc_serialize;

use std::ffi::{CStr,CString};

mod item;

#[no_mangle]
pub extern "C" fn ffi_calculate(c_ptr: *const libc::c_char) -> *const libc::c_char {
    let ruby_string = string_from_c_ptr(c_ptr);

    let it = item::Item::new_vec_from_json(&ruby_string);
    println!("item begin_time: {}", it[0].begin_time);

    c_ptr_from_string("from rust with love")
}

fn string_from_c_ptr(c_ptr: *const libc::c_char) -> String {
    let c_str = unsafe {
        assert!(!c_ptr.is_null());
        CStr::from_ptr(c_ptr)
    };
    c_str.to_str().unwrap().to_string()
}

fn c_ptr_from_string(s: &str) -> *const libc::c_char {
    CString::new(s).unwrap().into_raw()
}
