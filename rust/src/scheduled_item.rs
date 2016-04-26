use item;

#[derive(Debug)]
pub struct ScheduledItem<'a> {
    pub item: &'a item::Item,
    pub timeshift: i32
}

impl <'a> ScheduledItem<'a> {
    pub fn schedule_seconds(&self) -> Vec<u32> {
        let v: Vec<u32> = if self.item.playbacks_count == 0 {
            Vec::new()
        } else {
            (0..self.item.playbacks_count).map(|i| (self.item.begin_time as i32 + self.timeshift + i as i32 * self.period_seconds() as i32) as u32).collect()
        };
        v
    }

    pub fn period_seconds(&self) -> u32 {
        (self.item.end_time - self.item.begin_time) / (self.item.playbacks_count + 1)
    }
}

pub fn items_with_times<'a>(items: &'a [ScheduledItem<'a>]) -> Vec<(u32, &'a ScheduledItem<'a>)> {
    let mut res: Vec<(u32, &ScheduledItem)> = Vec::new();
    for i in items.iter() {
        let mut r: Vec<(u32, &ScheduledItem)> = Vec::new();
        let seconds = i.schedule_seconds();
        for j in seconds {
            r.push((j, &i));
        }
        res.extend_from_slice(&r);
    }
    res
}
