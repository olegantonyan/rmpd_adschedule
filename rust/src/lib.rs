extern crate libc;
extern crate rustc_serialize;
extern crate time;
extern crate itertools;

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
                values.push(scheduled_item::ScheduledItem::new(*j, 0));
            }
        }
        if !values.is_empty() { // start scheduling from the most dense element
            values.sort_by( |a, b| a.density().partial_cmp(&b.density()).unwrap() );
            values.reverse();


            let mut last_schedule = scheduled_item::schedule(0, values);;
            let mut last_distance = 0.0;
            let mut break_point = false;
            loop {
                let mut new_values = Vec::new();
                for i in last_schedule.iter() {
                    let new_shift = if i.timeshift == 0 {
                        0
                    } else {
                        let shift_value = 60;
                        if i.allow_shift(shift_value) {
                            i.timeshift + shift_value
                        } else {
                            break_point = true;
                            0
                        }
                    };
                    new_values.push(scheduled_item::ScheduledItem::new(i.item, new_shift))
                }
                let current_schedule = scheduled_item::schedule(0, new_values.clone());
                let current_distance = scheduled_item::Axis::new(&current_schedule, current_schedule.len()).rms_distance();

                //println!("last: {:?} current: {:?}",last_distance, current_distance);

                if break_point || ((current_distance > last_distance) && last_distance > 0.0) || (current_distance < 0.0 && last_distance < 0.0) {
                    hash_intervals.insert(i, last_schedule);
                    break;
                } else {
                    last_distance = current_distance;
                    last_schedule = new_values;
                }
            }


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
