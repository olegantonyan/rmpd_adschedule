use datetime;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct DateInterval {
    pub begin: i32,
    pub end: i32
}

impl DateInterval {
    pub fn to_date_string(day: i32) -> String {
        datetime::date_secs_since_epoch_to_string(&day)
    }
}
