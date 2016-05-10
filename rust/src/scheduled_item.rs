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
}

impl <'a> ScheduledItem<'a> {
    pub fn schedule_seconds(&self) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        if self.item.playbacks_count != 0 {
            for i in 0..self.item.playbacks_count {
                let value = self.item.begin_time + self.timeshift + i * self.period_seconds();
                v.push(value);
            }
        };
        v
    }

    pub fn schedule_times(&self) -> Vec<String> {
        self.schedule_seconds().iter().map(|i| datetime::time_secs_since_midnight_to_string(i) ).collect()
    }

    pub fn period_seconds(&self) -> i32 {
        (self.item.end_time - self.item.begin_time) / (self.item.playbacks_count + 1)
    }

    pub fn max_positive_allowed_shift(&self) -> i32 {
        self.item.end_time - self.schedule_seconds().last().unwrap()
    }

    pub fn max_negative_allowed_shift(&self) -> i32 {
        self.schedule_seconds().first().unwrap() - self.item.begin_time
    }

    pub fn shift_time(&mut self, value: i32) {
        self.timeshift += value;
    }
 }

pub fn items_with_times<'a>(items: &'a Vec<Rc<RefCell<ScheduledItem<'a>>>>) -> Vec<(i32, Rc<RefCell<ScheduledItem<'a>>>)> {
    let mut res: Vec<(i32, Rc<RefCell<ScheduledItem>>)> = Vec::new();
    for i in items.iter() {
        let seconds = i.borrow().schedule_seconds();
        for j in seconds {
            res.push((j, i.clone()));
        }
    }
    res.sort_by(|a, b| a.0.cmp(&b.0));
    res
}

pub fn is_near(one: &i32, two: &i32) -> bool {
    let delta = 60;
    (one - two).abs() <= delta
}

pub fn shift_time(one: &Rc<RefCell<ScheduledItem>>, two: &Rc<RefCell<ScheduledItem>>) {
    let base_time_shift = 60;

    let mut shift_one = false;
    let mut shift_two = false;

    {
        if two.borrow().max_positive_allowed_shift() >= base_time_shift {
            shift_two = true;
        } else if one.borrow().max_negative_allowed_shift() >= base_time_shift {
            shift_one = true;
        }
    }

    if shift_two {
        two.borrow_mut().shift_time(base_time_shift);
    } else if shift_one {
        one.borrow_mut().shift_time(-base_time_shift);
    }
}

pub fn overlap<'a>(items_with_times: &Vec<(i32, Rc<RefCell<ScheduledItem<'a>>>)>) -> Option<(Rc<RefCell<ScheduledItem<'a>>>, Rc<RefCell<ScheduledItem<'a>>>)> {
    let mut i = 0;
    while i < (items_with_times.len() - 1) {
        let current = &items_with_times[i];
        let next = &items_with_times[i + 1];
        i += 1;

        if is_near(&current.0, &next.0) {
            return Some((current.1.clone(), next.1.clone()));
        }
    }
    None
}

pub fn vec_to_json<'a>(inervals: &'a HashMap<&date_interval::DateInterval, Vec<Rc<RefCell<ScheduledItem<'a>>>>>) -> String {
    let mut v = Vec::new();
    for (interval, scheduled_items) in inervals.iter() {

        for item in scheduled_items.iter() {

            let ov: Vec<i32> = match overlap(&items_with_times(scheduled_items)) {
                None => Vec::new(),
                Some((f, s)) => vec![f.borrow().item.id, s.borrow().item.id],
            };

            let i = item.borrow();
            v.push(
                ScheduledItemRaw {
                    id: i.item.id,
                    begin_date: interval.to_date_string(),
                    end_date: interval.to_date_string(),
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
