use rustc_serialize::json;

mod datetime;

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
            begin_date: datetime::date_string_to_days_since_epoch(begin_date),
            end_date: datetime::date_string_to_days_since_epoch(end_date),
            begin_time: datetime::time_string_to_secs_since_midnight(begin_time),
            end_time: datetime::time_string_to_secs_since_midnight(end_time),
            playbacks_count: playbacks_count
        }
    }

    pub fn new_vec_from_json(json: &str) -> Vec<Item> {
        let da: Vec<ItemRaw> = json::decode(&json).unwrap();
        let mut res: Vec<Item> = Vec::new();
        for d in da.iter() {
            res.push(Item::new(d.id, &d.begin_date, &d.end_date, &d.begin_time, &d.end_time, d.playbacks_count));
        }
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

#[derive(RustcDecodable, RustcEncodable)]
struct ItemRaw {
    id: u32,
    begin_date: String,
    end_date: String,
    begin_time: String,
    end_time: String,
    playbacks_count: u16
}
