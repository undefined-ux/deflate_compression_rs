use chrono::prelude::*;
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct DateTime {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl DateTime {
    pub fn new(timestamp: u64, timezone: i32) -> Self {
        let timezone = chrono::FixedOffset::east_opt(timezone).expect("Invalid timezone");
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
            .expect("Invalid timestamp")
            .with_timezone(&timezone);
        DateTime {
            year: datetime.year() as u32,
            month: datetime.month() as u32,
            day: datetime.day() as u32,
            hour: datetime.hour() as u32,
            minute: datetime.minute() as u32,
            second: datetime.second() as u32,
        }
    }

    pub fn local() -> Self {
        let now = Local::now().fixed_offset();
        let offset = now.timezone().local_minus_utc();
        let timestamp = now.timestamp();
        return Self::new(timestamp as u64, offset);
    }

    pub fn utc() -> Self {
        let now = Utc::now().timestamp();
        Self::new(now as u64, 0)
    }

    pub fn msdos_date(&self) -> u16 {
        ((self.year - 1980) as u16) << 9 | (self.month << 5) as u16 | self.day as u16
    }

    pub fn msdos_time(&self) -> u16 {
        (self.hour as u16) << 11 | (self.minute as u16) << 5 | ((self.second / 2) as u16)
    }
}

impl From<SystemTime> for DateTime {
    fn from(time: SystemTime) -> Self {
        let now = time.duration_since(UNIX_EPOCH).unwrap().as_secs();

        return DateTime::new(now, 0);
    }
}

impl Into<u64> for DateTime {
    fn into(self) -> u64 {
        return Utc
            .with_ymd_and_hms(
                self.year as i32,
                self.month,
                self.day,
                self.hour,
                self.minute,
                self.second,
            )
            .unwrap()
            .timestamp() as u64;
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}
