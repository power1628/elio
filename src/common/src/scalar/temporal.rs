//! - Date:
//! - LocalTime:
//! - LocalDateTime:
//! - ZonedTime:
//! - ZonedDateTime:
//! - Duration:

use chrono::{FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

/// Date without timezone
/// days since unix epoch
/// may be negative numbers
/// e.g. 2007-12-03
/// chrono::NaiveDate
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Date(pub i64);

impl TryFrom<&str> for Date {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .map_err(|_| format!("date format error, expected yyyy-MM-dd, actual {}", value))
            .map(Into::into)
    }
}

impl From<NaiveDate> for Date {
    fn from(value: NaiveDate) -> Self {
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let days = value.signed_duration_since(epoch).num_days();
        Date(days)
    }
}

impl From<Date> for NaiveDate {
    fn from(value: Date) -> Self {
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        epoch
            .checked_add_signed(chrono::Duration::days(value.0))
            .expect("date out of range for chrono")
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NaiveDate::from(*self))
    }
}

/// time of a day without timezone
// nanoseconds since midnight
// chrono::NaiveTime
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LocalTime(pub u64);

impl From<NaiveTime> for LocalTime {
    fn from(value: NaiveTime) -> Self {
        let secs = value.num_seconds_from_midnight() as u64;
        let nanos = value.nanosecond() as u64;
        let time = secs * 1_000_000_000 + nanos;
        LocalTime(time)
    }
}

impl From<LocalTime> for NaiveTime {
    fn from(value: LocalTime) -> Self {
        let total_ns = value.0;
        let secs = (total_ns / 1_000_000_000) as u32;
        let nanos = (total_ns % 1_000_000_000) as u32;
        NaiveTime::from_hms_nano_opt(0, 0, secs, nanos).expect("invalid naive time")
    }
}

impl std::fmt::Display for LocalTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NaiveTime::from(*self))
    }
}

/// Date and time without timezone
/// chrono::NaiveDatetime
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalDateTime {
    // seconds since unix epoch
    pub seconds: i64,
    // nanoseconds fraction, range from 0 to 999_999_999
    pub nanoseconds: u32,
}

impl LocalDateTime {
    pub fn to_date(&self) -> Date {
        let days = self.seconds / 86_400;
        Date(days)
    }
}

impl From<NaiveDateTime> for LocalDateTime {
    fn from(value: NaiveDateTime) -> Self {
        Self {
            seconds: value.and_utc().timestamp(),
            nanoseconds: value.and_utc().timestamp_subsec_nanos(),
        }
    }
}

impl From<LocalDateTime> for NaiveDateTime {
    fn from(value: LocalDateTime) -> Self {
        NaiveDateTime::from_timestamp(value.seconds, value.nanoseconds)
    }
}
impl std::fmt::Display for LocalDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NaiveDateTime::from(*self))
    }
}

/// Time of a day with timezone
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ZonedTime {
    // nanoseconds since midnight
    pub nanoseconds: u64,
    // offset in seconds from UTC
    pub tz_offset_seconds: i32,
}

impl std::fmt::Display for ZonedTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nt = NaiveTime::from(LocalTime(self.nanoseconds));
        let offset = FixedOffset::east_opt(self.tz_offset_seconds).unwrap();
        write!(f, "{}{}", nt, offset)
    }
}

/// Date, time and timezone
/// chrono::DateTime<FixedOffset>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ZonedDateTime {
    // seconds since unix epoch
    pub seconds: i64,
    // nanoseconds fraction, range from 0 to 999_999_999
    pub nanoseconds: u32,
    // offset in seconds from UTC
    pub tz_offset_seconds: i32,
}

impl ZonedDateTime {
    pub fn to_date(&self) -> Date {
        let days = self.seconds / 86_400;
        Date(days)
    }
}

impl From<chrono::DateTime<FixedOffset>> for ZonedDateTime {
    fn from(value: chrono::DateTime<FixedOffset>) -> Self {
        Self {
            seconds: value.timestamp(),
            nanoseconds: value.timestamp_subsec_nanos(),
            tz_offset_seconds: value.offset().local_minus_utc(),
        }
    }
}

impl From<ZonedDateTime> for chrono::DateTime<FixedOffset> {
    fn from(value: ZonedDateTime) -> Self {
        let naive = NaiveDateTime::from_timestamp_opt(value.seconds, value.nanoseconds).unwrap();
        let offset = FixedOffset::east_opt(value.tz_offset_seconds).unwrap();
        chrono::DateTime::from_naive_utc_and_offset(naive, offset)
    }
}

impl std::fmt::Display for ZonedDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", chrono::DateTime::<FixedOffset>::from(*self))
    }
}

/// Duration
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, derive_more::Display)]
#[display("P{months}M{days}DT{seconds}S{nanoseconds}")]
pub struct Duration {
    pub months: i64,
    pub days: i64,
    pub seconds: i64,
    pub nanoseconds: i64,
}
