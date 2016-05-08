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
    let h = a / 3600;
    let rem = a % 3600;
    let m = rem / 60;
    let s = rem % 60;
    let tim = time::Tm { tm_hour: h, tm_min: m, tm_sec: s, tm_nsec: 0, tm_wday: 0, tm_isdst: 0, tm_mon: 0, tm_yday: 0, tm_year: 0, tm_utcoff: 0, tm_mday: 0 };
    time::strftime("%H:%M:%S", &tim).unwrap()
}
