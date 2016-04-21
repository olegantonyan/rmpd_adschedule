extern crate time;

pub fn time_string_to_secs_since_midnight(s: &str) -> u32 {
    match time::strptime(s, "%H:%M:%S") {
        Ok(r) => return time_tm_to_secs_since_midnight(r),
        Err(e) => panic!("error parsing time string '{}': {}", s, e)
    }
}

pub fn date_string_to_days_since_epoch(s: &str) -> u32 {
    match time::strptime(s, "%d.%m.%Y") {
        Ok(r) => return date_tm_to_days_since_epoch(r),
        Err(e) => panic!("error parsing date string '{}': {}", s, e)
    }
}

fn time_tm_to_secs_since_midnight(tm: time::Tm) -> u32 {
    (tm.tm_hour * 3600 + tm.tm_min * 60 + tm.tm_sec) as u32
}

fn date_tm_to_days_since_epoch(tm: time::Tm) -> u32 {
    (tm.to_timespec().sec / 86400) as u32
}
