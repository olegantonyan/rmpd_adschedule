use rustc_serialize::json;
use std::collections::HashMap;
use itertools::Itertools;

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
        let axis = Axis::new(&items_copy, item_index);
        let distance = axis.rms_distance();
        if distance > 0.0 {
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

pub fn vec_to_json(inervals: HashMap<&date_interval::DateInterval, Vec<ScheduledItem>>) -> String {
    let mut v = Vec::new();
    for (interval, scheduled_items) in inervals.iter() {

        for item in scheduled_items.iter() {

            let distance = Axis::new(scheduled_items, scheduled_items.len()).rms_distance();

            v.push(
                ScheduledItemRaw {
                    id: item.item.id,
                    begin_date: date_interval::DateInterval::to_date_string(interval.begin),
                    end_date: date_interval::DateInterval::to_date_string(interval.end),
                    schedule: item.schedule_times(),
                    distance: distance,
                }
            );
        }
    }
    json::encode(&v).unwrap()
}

#[derive(Debug, RustcEncodable)]
struct ScheduledItemRaw {
    id: i64,
    begin_date: String,
    end_date: String,
    schedule: Vec<String>,
    distance: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub item_id: i64,
    pub seconds: i32
}

#[derive(Debug)]
pub struct Axis {
    pub points: Vec<Point>
}

impl Axis {
    pub fn new(items: &Vec<ScheduledItem>, upto_index: usize) -> Axis {
        let mut res = Axis { points: Vec::new() };
        for (index, i) in items.iter().enumerate() {
            for j in i.schedule_seconds() {
                res.points.push(Point { item_id: i.item.id, seconds: j });
            }
            if index == upto_index {
                break;
            }
        }
        res.points.sort_by(|a, b| a.seconds.cmp(&b.seconds));
        res
    }

    pub fn rms_distance(&self) -> f64 {
        let delta_overlap = 60; // seconds

        let mut sum = 0;
        for (a, b) in self.points.iter().tuple_windows() {
            let distance = b.seconds - a.seconds;
            if distance.abs() < delta_overlap {
                return -1.0;
            }
            sum += distance.pow(2);
        }
        (sum as f64).sqrt()
    }
}
