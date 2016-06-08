use rustc_serialize::json;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use item;
use datetime;
use date_interval;

#[derive(Debug)]
pub struct ScheduledItem<'a> {
    pub item: &'a item::Item,
    pub timeshift: i32,
    pub schedule_seconds: Vec<i32>
}

impl <'a> ScheduledItem<'a> {
    pub fn new(item: &'a item::Item) -> ScheduledItem {
        let mut result = ScheduledItem { item: item, timeshift: 0, schedule_seconds: Vec::new() };
        result.schedule_seconds = result.calculate_schedule_seconds();
        result
    }

    pub fn calculate_schedule_seconds(&self) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        if self.item.playbacks_count != 0 {
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
        self.schedule_seconds.iter().map(|i| datetime::time_secs_since_midnight_to_string(i) ).collect()
    }

    pub fn period_seconds(&self) -> i32 {
        (self.item.end_time - self.item.begin_time) / self.item.playbacks_count
    }

    pub fn max_positive_allowed_shift(&self) -> i32 {
        self.item.end_time - self.schedule_seconds.last().unwrap()
    }

    pub fn max_negative_allowed_shift(&self) -> i32 {
        self.schedule_seconds.first().unwrap() - self.item.begin_time
    }

    pub fn shift_time(&mut self, value: i32) -> bool {
        if value == 0 || !self.allow_shift(value){
            return false;
        }
        self.timeshift += value;
        for i in self.schedule_seconds.iter_mut() {
            *i = *i + value;
        }
        true
    }

    pub fn reset_shift(&mut self) {
        for i in self.schedule_seconds.iter_mut() {
            *i = *i - self.timeshift;
        }
        self.timeshift = 0;
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

pub fn schedule<'a>(item_index: usize, scheduled_items: &'a Vec<Rc<RefCell<ScheduledItem<'a>>>>) -> bool {
    if item_index >= scheduled_items.len() {
        return true;
    }
    loop {
        let items_with_times = items_with_times(scheduled_items, item_index);
        if overlap(&items_with_times).is_none() && schedule(item_index + 1, scheduled_items) {
            return true;
        }
        if !scheduled_items[item_index].borrow_mut().shift_time(60) {
            break;
        }
    }
    //scheduled_items[item_index].borrow_mut().reset_shift();
    false
}

pub fn items_with_times<'a>(items: &'a Vec<Rc<RefCell<ScheduledItem<'a>>>>, upto_index: usize) -> Vec<(i32, Rc<RefCell<ScheduledItem<'a>>>)> {
    let mut res: Vec<(i32, Rc<RefCell<ScheduledItem>>)> = Vec::new();
    for (index, i) in items.iter().enumerate() {
        let seconds = i.borrow().schedule_seconds.clone();
        for j in seconds {
            res.push((j, i.clone()));
        }
        if index == upto_index {
            break;
        }
    }
    res.sort_by(|a, b| a.0.cmp(&b.0));
    res
}

pub fn is_near<'a>(one: &(i32, Rc<RefCell<ScheduledItem<'a>>>), two: &(i32, Rc<RefCell<ScheduledItem<'a>>>)) -> bool {
    let delta = 60;
    (one.0 - two.0).abs() < delta
}

pub fn overlap<'a>(items_with_times: &Vec<(i32, Rc<RefCell<ScheduledItem<'a>>>)>) -> Option<(Rc<RefCell<ScheduledItem<'a>>>, Rc<RefCell<ScheduledItem<'a>>>)> {
    let mut i = 0;
    while i < (items_with_times.len() - 1) {
        let current = &items_with_times[i];
        let next = &items_with_times[i + 1];
        i += 1;

        if is_near(&current, &next) {
            return Some((current.1.clone(), next.1.clone()));
        }
    }
    None
}

pub fn vec_to_json<'a>(inervals: &'a HashMap<&date_interval::DateInterval, Vec<Rc<RefCell<ScheduledItem<'a>>>>>) -> String {
    let mut v = Vec::new();
    for (interval, scheduled_items) in inervals.iter() {

        for item in scheduled_items.iter() {

            let ov: Vec<i32> = match overlap(&items_with_times(scheduled_items, scheduled_items.len())) {
                None => Vec::new(),
                Some((f, s)) => vec![f.borrow().item.id, s.borrow().item.id],
            };

            let i = item.borrow();
            v.push(
                ScheduledItemRaw {
                    id: i.item.id,
                    begin_date: date_interval::DateInterval::to_date_string(interval.begin),
                    end_date: date_interval::DateInterval::to_date_string(interval.end),
                    schedule: i.schedule_times(),
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
