extern crate libc;
extern crate rustc_serialize;
extern crate time;

use std::collections::HashMap;

mod item;
mod datetime;
mod date_interval;
mod scheduled_item;

#[no_mangle]
pub extern "C" fn ffi_calculate(c_ptr: *const libc::c_char) -> *const libc::c_char {
    let items = item::Item::new_vec_from_json(&string_from_c_ptr(c_ptr));
    let intervals = item::date_intervals_sorted(&items);

    let mut hash_intervals = HashMap::new();
    for i in intervals.iter() {
        let mut values: Vec<scheduled_item::ScheduledItem> = Vec::new();

        for j in items.iter() {
            if j.is_appropriate_at(i) { // associate this item with this interval
                values.push(scheduled_item::ScheduledItem::new(j));
            }
        }
        if !values.is_empty() { // start scheduling from the most dense element
            values.sort_by( |a, b| a.density().partial_cmp(&b.density()).unwrap() );
            values.reverse();
            hash_intervals.insert(i, scheduled_item::schedule(0, values));
        }
    }

    let r = scheduled_item::vec_to_json(hash_intervals);
    c_ptr_from_string(&r)
}

#[no_mangle]
pub extern "C" fn ffi_free(c_ptr: *mut libc::c_void) {
    unsafe {
        libc::free(c_ptr);
    }
}

fn string_from_c_ptr(c_ptr: *const libc::c_char) -> String {
    let c_str = unsafe {
        assert!(!c_ptr.is_null());
        std::ffi::CStr::from_ptr(c_ptr)
    };
    c_str.to_str().unwrap().to_string()
}

fn c_ptr_from_string(s: &str) -> *const libc::c_char {
    std::ffi::CString::new(s).unwrap().into_raw()
}
