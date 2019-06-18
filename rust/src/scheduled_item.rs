use rustc_serialize::json;
use std::collections::HashMap;

use item;
use datetime;
use date_interval;

#[derive(Debug, Copy, Clone)]
pub struct ScheduledItem {
    pub item: item::Item,
    pub timeshift: i32
}

impl ScheduledItem {
    pub fn new(item: item::Item, timeshift: i32) -> ScheduledItem {
        ScheduledItem { item: item, timeshift: timeshift }
    }

    pub fn schedule_seconds(&self) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        if self.item.playbacks_count > 0 {
            for i in 0..self.item.playbacks_count {
                let value = self.item.begin_time + self.timeshift + i * self.period_seconds();
                v.push(value);
            }
        };
        v
    }

    pub fn density(&self) -> f32 {
        1.0 / self.period_seconds() as f32
    }

    pub fn schedule_times(&self) -> Vec<String> {
        self.schedule_seconds().iter().map(|i| datetime::time_secs_since_midnight_to_string(i) ).collect()
    }

    pub fn period_seconds(&self) -> i32 {
        (self.item.end_time - self.item.begin_time) / self.item.playbacks_count
    }

    pub fn max_positive_allowed_shift(&self) -> i32 {
        self.item.end_time - self.schedule_seconds().last().unwrap()
    }

    pub fn max_negative_allowed_shift(&self) -> i32 {
        self.schedule_seconds().first().unwrap() - self.item.begin_time
    }

    pub fn allow_shift(&self, value: i32) -> bool {
        if value > 0 {
            self.max_positive_allowed_shift() >= value
        } else if value < 0 {
            self.max_negative_allowed_shift() >= -value
        } else {
            false
        }
    }
}

pub fn schedule(item_index: usize, scheduled_items: Vec<ScheduledItem>) -> Vec<ScheduledItem> {
    if item_index >= scheduled_items.len() {
        return scheduled_items;
    }
    let mut items_copy = scheduled_items;

    loop {
        let ids_with_times = items_ids_with_times(&items_copy, item_index);
        if overlap(&ids_with_times).is_none() {
            return schedule(item_index + 1, items_copy.clone());
        }

        let timeshift = 60;
        if items_copy[item_index].allow_shift(timeshift) {
            let mut new_items_copy: Vec<ScheduledItem> = Vec::new();
            for (index, i) in items_copy.iter().enumerate() {
                if index == item_index {
                    new_items_copy.push(ScheduledItem::new(i.item, i.timeshift + timeshift))
                } else {
                    new_items_copy.push(*i);
                }
            }
            items_copy = new_items_copy;
        } else {
            break;
        }
    }

    items_copy
}

pub fn items_ids_with_times(items: &Vec<ScheduledItem>, upto_index: usize) -> Vec<(i32, i32)> {
    let mut res: Vec<(i32, i32)> = Vec::new();
    for (index, i) in items.iter().enumerate() {
        for j in i.schedule_seconds() {
            res.push((j, i.item.id));
        }
        if index == upto_index {
            break;
        }
    }
    res.sort_by(|a, b| a.0.cmp(&b.0));
    res
}

pub fn is_near(one: i32, two: i32) -> bool {
    let delta = 60; // seconds
    (one - two).abs() < delta
}

pub fn overlap(items_ids_with_times: &Vec<(i32, i32)>) -> Option<(i32, i32)> {
    let mut i = 0;
    while i < (items_ids_with_times.len() - 1) {
        let current = &items_ids_with_times[i];
        let next = &items_ids_with_times[i + 1];
        i += 1;

        if is_near(current.0, next.0) {
            return Some((current.1, next.1));
        }
    }
    None
}

pub fn vec_to_json(inervals: HashMap<&date_interval::DateInterval, Vec<ScheduledItem>>) -> String {
    let mut v = Vec::new();
    for (interval, scheduled_items) in inervals.iter() {

        for item in scheduled_items.iter() {

            let ov: Vec<i32> = match overlap(&items_ids_with_times(scheduled_items, scheduled_items.len())) {
                None => Vec::new(),
                Some((f, s)) => vec![f, s],
            };

            v.push(
                ScheduledItemRaw {
                    id: item.item.id,
                    begin_date: date_interval::DateInterval::to_date_string(interval.begin),
                    end_date: date_interval::DateInterval::to_date_string(interval.end),
                    schedule: item.schedule_times(),
                    overlap: ov,
                }
            );
        }
    }
    json::encode(&v).unwrap()
}

#[derive(Debug, RustcEncodable)]
struct ScheduledItemRaw {
    id: i32,
    begin_date: String,
    end_date: String,
    schedule: Vec<String>,
    overlap: Vec<i32>,
}
