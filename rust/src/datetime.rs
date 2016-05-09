use time;

pub fn time_string_to_secs_since_midnight(s: &str) -> i32 {
    match time::strptime(s, "%H:%M:%S") {
        Ok(r) => return time_tm_to_secs_since_midnight(r),
        Err(e) => panic!("error parsing time string '{}': {}", s, e)
    }
}

pub fn date_string_to_secs_since_epoch(s: &str) -> i32 {
    match time::strptime(s, "%d.%m.%Y") {
        Ok(r) => return date_tm_to_secs_since_epoch(r),
        Err(e) => panic!("error parsing date string '{}': {}", s, e)
    }
}

fn time_tm_to_secs_since_midnight(tm: time::Tm) -> i32 {
    (tm.tm_hour * 3600 + tm.tm_min * 60 + tm.tm_sec) as i32
}

fn date_tm_to_secs_since_epoch(tm: time::Tm) -> i32 {
    tm.to_timespec().sec as i32
}

pub fn time_secs_since_midnight_to_string(a: &i32) -> String {
    let timespec = time::Timespec::new(*a as i64, 0);
    let tim = time::at_utc(timespec);
    time::strftime("%H:%M:%S", &tim).unwrap()
}

pub fn date_secs_since_epoch_to_string(s: &i32) -> String {
    let timespec = time::Timespec::new(*s as i64, 0);
    let tim = time::at_utc(timespec);
    time::strftime("%d.%m.%Y", &tim).unwrap()
}
