use rustc_serialize::json;
use std::collections::HashSet;

use datetime;
use date_interval::DateInterval;

pub struct Item {
    pub id: u32,
    pub begin_date: u32,
    pub end_date: u32,
    pub begin_time: u32,
    pub end_time: u32,
    pub playbacks_count: u16
}

impl Item {
    pub fn new(id: u32, begin_date: &str, end_date: &str, begin_time: &str, end_time: &str, playbacks_count: u16) -> Item {
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
}

pub fn vec_to_json() -> String {
    let mut v = Vec::new();
    v.push(
        ItemRaw {
            id: 1,
            begin_date: "12.12.1012".to_string(),
            end_date: "12.12.1012".to_string(),
            begin_time: "12.12.1012".to_string(),
            end_time: "12.12.1012".to_string(),
            playbacks_count: 2
        }
    );
    json::encode(&v).unwrap()
}

pub fn date_intervals_sorted(items: &Vec<Item>) -> Vec<DateInterval> {
    let mut set: HashSet<u32> = HashSet::new();
    for i in items.iter() {
        set.insert(i.begin_date);
        set.insert(i.end_date);
    }
    let mut vec: Vec<u32> = set.iter().cloned().collect();
    vec.sort();

    if vec.len() == 1 {
        let mut res: Vec<DateInterval> = Vec::new();
        res.push(DateInterval { begin: vec[0], end: vec[0] });
        return res;
    }

    let mut res: Vec<DateInterval> = Vec::new();
    let mut i = 0;
    while i + 1 < vec.len() {
        res.push(DateInterval { begin: vec[i], end: vec[i + 1] });
        i += 1;
    }
    res
}

#[derive(RustcDecodable, RustcEncodable)]
struct ItemRaw {
    id: u32,
    begin_date: String,
    end_date: String,
    begin_time: String,
    end_time: String,
    playbacks_count: u16
}
