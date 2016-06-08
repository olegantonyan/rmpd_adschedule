use rustc_serialize::json;
use std::collections::HashSet;

use datetime;
use date_interval::DateInterval;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Item {
    pub id: i32,
    pub begin_date: i32,
    pub end_date: i32,
    pub begin_time: i32,
    pub end_time: i32,
    pub playbacks_count: i32
}

impl Item {
    pub fn new(id: i32, begin_date: &str, end_date: &str, begin_time: &str, end_time: &str, playbacks_count: i32) -> Item {
        Item {
            id: id,
            begin_date: datetime::date_string_to_secs_since_epoch(begin_date),
            end_date: datetime::date_string_to_secs_since_epoch(end_date),
            begin_time: datetime::time_string_to_secs_since_midnight(begin_time),
            end_time: datetime::time_string_to_secs_since_midnight(end_time),
            playbacks_count: playbacks_count
        }
    }

    pub fn new_vec_from_json(json: &str) -> Vec<Item> {
        let da: Vec<ItemRaw> = json::decode(&json).unwrap();
        let res: Vec<Item> = da.iter().map(|d|
            Item::new(d.id, &d.begin_date, &d.end_date, &d.begin_time, &d.end_time, d.playbacks_count)
        ).collect();
        res
    }

    pub fn is_appropriate_at(&self, date_interval: &DateInterval) -> bool {
        self.begin_date <= date_interval.begin && date_interval.end <= self.end_date
    }
}

pub fn date_intervals_sorted(items: &Vec<Item>) -> Vec<DateInterval> {
    let mut set: HashSet<i32> = HashSet::new();
    for i in items.iter() {
        set.insert(i.begin_date);
        set.insert(i.end_date);
    }
    let mut res: Vec<DateInterval> = Vec::new();
    if set.len() == 0 {
        return res;
    }

    let min = *set.iter().min().unwrap();
    let max = *set.iter().max().unwrap();
    if set.len() == 2 || set.len() == 1 {
        res.push(DateInterval { begin: min.clone(), end: max.clone() });
        return res;
    }

    let mut arry: Vec<i32> = Vec::new();
    arry.extend(set.into_iter());
    arry.sort();

/*    println!("********");
    println!("{:?}", arry.iter().map(|i| DateInterval::to_date_string(*i)).collect::<Vec<String>>());
    println!("********");*/

    let mut prev = min;
    for i in arry.iter() {
        if *i == prev {
            continue;
        }
        if *i == max {
            res.push(DateInterval { begin: prev, end: *i });
            break;
        }
        res.push(DateInterval { begin: prev, end: (*i - 86400) });
        if (*i - 86400) != prev {
            res.push(DateInterval { begin: *i, end: *i });
            prev = *i + 86400;
        } else {
            prev = *i;
        }
    }
    res
}

#[derive(Debug, RustcDecodable)]
struct ItemRaw {
    id: i32,
    begin_date: String,
    end_date: String,
    begin_time: String,
    end_time: String,
    playbacks_count: i32
}
