use datetime;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct DateInterval {
    pub day: i32,
}

impl DateInterval {
    pub fn to_date_string(&self) -> String {
        datetime::date_secs_since_epoch_to_string(&self.day)
    }
}
