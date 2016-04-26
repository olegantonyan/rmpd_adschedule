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
            if j.is_appropriate_at(i.day) {
                // associate this item with this interval
                let si = scheduled_item::ScheduledItem { item: &j, timeshift: 0 };
                values.push(si);
            }
        }
        if !values.is_empty() {
            hash_intervals.insert(i, values);
        }
    }

    for (_, scheduled_items) in hash_intervals.iter() {
        let items_with_times = scheduled_item::items_with_times(&scheduled_items);

        let mut i = 0;
        while i < (items_with_times.len() - 1) {
            let current = items_with_times[i];
            let next = items_with_times[i + 1];
            i += 1;

            println!("*********");
            println!("current {:?}", current);
            println!("next {:?}", next);
            println!("*********");
        }
    }

    let r = item::vec_to_json();
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
