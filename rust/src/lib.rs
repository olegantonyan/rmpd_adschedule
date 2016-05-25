extern crate libc;
extern crate rustc_serialize;
extern crate time;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

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
        let mut values: Vec<Rc<RefCell<scheduled_item::ScheduledItem>>> = Vec::new();
        for j in items.iter() {
            if j.is_appropriate_at(i) {
                // associate this item with this interval
                let si = RefCell::new(scheduled_item::ScheduledItem { item: j, timeshift: 0 });
                let rc = Rc::new(si);
                values.push(rc);
            }
        }
        if !values.is_empty() {
            hash_intervals.insert(i, values);
        }
    }

    for (_, scheduled_items) in hash_intervals.iter() {
        let max_iterations = scheduled_items.len() * 50;

        let mut iterations = 0;
        'outer: while iterations < max_iterations {
            iterations += 1;

            let items_with_times = scheduled_item::items_with_times(scheduled_items);

            let mut i = 0;
            while i < (items_with_times.len() - 1) {
                let current = &items_with_times[i];
                let next = &items_with_times[i + 1];
                i += 1;

                if scheduled_item::is_near(&current.0, &next.0) {
                    scheduled_item::shift_time(&current.1, &next.1);
                    continue 'outer;
                }
            }

            match scheduled_item::overlap(&items_with_times) {
                None => break,
                _ => (),
            }
        }

    }

    let r = scheduled_item::vec_to_json(&hash_intervals);
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
