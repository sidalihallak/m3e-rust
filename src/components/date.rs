//! Date-only Gregorian values. No timezone or time-of-day conversion is involved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CivilDate {
    year: u16,
    month: u8,
    day: u8,
}
impl CivilDate {
    pub fn new(year: u16, month: u8, day: u8) -> Option<Self> {
        if year == 0
            || year > 9999
            || !(1..=12).contains(&month)
            || day == 0
            || day > Self::days_in_month(year, month)
        {
            return None;
        }
        Some(Self { year, month, day })
    }
    pub const fn year(self) -> u16 {
        self.year
    }
    pub const fn month(self) -> u8 {
        self.month
    }
    pub const fn day(self) -> u8 {
        self.day
    }
    pub fn days_in_month(year: u16, month: u8) -> u8 {
        match month {
            2 => {
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }
    pub fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
    pub fn month_start(self) -> Self {
        Self { day: 1, ..self }
    }
    pub fn month_end(self) -> Self {
        Self {
            day: Self::days_in_month(self.year, self.month),
            ..self
        }
    }
    pub fn add_months(self, amount: i32) -> Self {
        let index = ((i64::from(self.year) - 1) * 12 + i64::from(self.month) - 1
            + i64::from(amount))
        .clamp(0, 9999 * 12 - 1);
        let year = (index / 12 + 1) as u16;
        let month = (index % 12 + 1) as u8;
        Self {
            year,
            month,
            day: self.day.min(Self::days_in_month(year, month)),
        }
    }
    pub fn add_days(self, amount: i32) -> Self {
        fn before_year(year: u16) -> i64 {
            let y = i64::from(year) - 1;
            365 * y + y / 4 - y / 100 + y / 400
        }
        let ordinal = before_year(self.year)
            + (1..self.month)
                .map(|m| i64::from(Self::days_in_month(self.year, m)))
                .sum::<i64>()
            + i64::from(self.day)
            - 1;
        let maximum = before_year(9999) + 364;
        let target = (ordinal + i64::from(amount)).clamp(0, maximum);
        let (mut low, mut high) = (1u16, 9999u16);
        while low < high {
            let mid = low + (high - low).div_ceil(2);
            if before_year(mid) <= target {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        let mut remaining = target - before_year(low);
        let mut month = 1;
        while remaining >= i64::from(Self::days_in_month(low, month)) {
            remaining -= i64::from(Self::days_in_month(low, month));
            month += 1;
        }
        Self {
            year: low,
            month,
            day: remaining as u8 + 1,
        }
    }
    pub fn weekday(self) -> u8 {
        let offsets = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let y = i32::from(self.year) - i32::from(self.month < 3);
        ((y + y / 4 - y / 100 + y / 400 + offsets[self.month as usize - 1] + i32::from(self.day))
            % 7) as u8
    }
    pub fn format(self, format: DateInputFormat) -> String {
        match format {
            DateInputFormat::MonthDayYear => {
                format!("{:02}/{:02}/{:04}", self.month, self.day, self.year)
            }
            DateInputFormat::DayMonthYear => {
                format!("{:02}/{:02}/{:04}", self.day, self.month, self.year)
            }
            DateInputFormat::YearMonthDay => {
                format!("{:04}/{:02}/{:02}", self.year, self.month, self.day)
            }
        }
    }
    /// Accept slashes, dashes, dots, spaces or eight compact digits. Never mask typing.
    pub fn parse(text: &str, format: DateInputFormat) -> Option<Self> {
        let text = text.trim();
        if text.is_empty() {
            return None;
        }
        let parts: Vec<&str> = text
            .split(|c: char| matches!(c, '/' | '-' | '.') || c.is_whitespace())
            .filter(|p| !p.is_empty())
            .collect();
        let numbers =
            if parts.len() == 1 && text.len() == 8 && text.bytes().all(|c| c.is_ascii_digit()) {
                match format {
                    DateInputFormat::YearMonthDay => vec![&text[..4], &text[4..6], &text[6..]],
                    _ => vec![&text[..2], &text[2..4], &text[4..]],
                }
            } else if parts.len() == 3 {
                parts
            } else {
                return None;
            };
        let (y, m, d) = match format {
            DateInputFormat::MonthDayYear => (numbers[2], numbers[0], numbers[1]),
            DateInputFormat::DayMonthYear => (numbers[2], numbers[1], numbers[0]),
            DateInputFormat::YearMonthDay => (numbers[0], numbers[1], numbers[2]),
        };
        if y.len() != 4
            || m.len() > 2
            || d.len() > 2
            || numbers
                .iter()
                .any(|p| !p.bytes().all(|c| c.is_ascii_digit()))
        {
            return None;
        }
        Self::new(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DateInputFormat {
    #[default]
    MonthDayYear,
    DayMonthYear,
    YearMonthDay,
}
impl DateInputFormat {
    pub fn hint(self) -> &'static str {
        match self {
            Self::MonthDayYear => "MM/DD/YYYY",
            Self::DayMonthYear => "DD/MM/YYYY",
            Self::YearMonthDay => "YYYY/MM/DD",
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DateLocale {
    pub months: [String; 12],
    pub short_months: [String; 12],
    pub weekdays: [String; 7],
    pub first_weekday: u8,
    pub format: DateInputFormat,
}
impl Default for DateLocale {
    fn default() -> Self {
        Self {
            months: [
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ]
            .map(str::to_string),
            short_months: [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ]
            .map(str::to_string),
            weekdays: [
                "Sunday",
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
            ]
            .map(str::to_string),
            first_weekday: 0,
            format: DateInputFormat::MonthDayYear,
        }
    }
}
impl DateLocale {
    pub fn full_date(&self, date: CivilDate) -> String {
        format!(
            "{}, {} {}, {}",
            self.weekdays[date.weekday() as usize],
            self.months[date.month as usize - 1],
            date.day,
            date.year
        )
    }
    pub fn headline(&self, date: CivilDate) -> String {
        format!(
            "{}, {} {}",
            self.weekdays[date.weekday() as usize]
                .chars()
                .take(3)
                .collect::<String>(),
            self.short_months[date.month as usize - 1],
            date.day
        )
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DateConstraints {
    pub min: Option<CivilDate>,
    pub max: Option<CivilDate>,
    pub disabled_dates: Vec<CivilDate>,
    pub disabled_weekdays: Vec<u8>,
    pub exclude_disabled_in_range: bool,
    pub first_year: u16,
    pub last_year: u16,
}
impl Default for DateConstraints {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
            disabled_dates: vec![],
            disabled_weekdays: vec![],
            exclude_disabled_in_range: false,
            first_year: 1900,
            last_year: 2100,
        }
    }
}
impl DateConstraints {
    pub fn allows(&self, date: CivilDate) -> bool {
        date.year >= self.first_year.max(1)
            && date.year <= self.last_year.min(9999)
            && self.min.is_none_or(|min| date >= min)
            && self.max.is_none_or(|max| date <= max)
            && !self.disabled_dates.contains(&date)
            && !self.disabled_weekdays.contains(&date.weekday())
    }
    pub fn allows_range(&self, start: CivilDate, end: CivilDate) -> bool {
        if start > end {
            return false;
        }
        if !self.exclude_disabled_in_range {
            return self.allows(start) && self.allows(end);
        }
        let mut date = start;
        loop {
            if !self.allows(date) {
                return false;
            }
            if date == end {
                return true;
            }
            let next = date.add_days(1);
            if next == date {
                return false;
            }
            date = next;
        }
    }
    pub(crate) fn bounds(&self) -> (u16, u16) {
        let first = self
            .first_year
            .clamp(1, 9999)
            .max(self.min.map(CivilDate::year).unwrap_or(1));
        let last = self
            .last_year
            .clamp(1, 9999)
            .min(self.max.map(CivilDate::year).unwrap_or(9999));
        (first, last)
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateRange {
    pub start: Option<CivilDate>,
    pub end: Option<CivilDate>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateSelection {
    Single(Option<CivilDate>),
    Range(DateRange),
}
impl Default for DateSelection {
    fn default() -> Self {
        Self::Single(None)
    }
}
impl DateSelection {
    pub fn start(self) -> Option<CivilDate> {
        match self {
            Self::Single(date) => date,
            Self::Range(range) => range.start,
        }
    }
    pub fn end(self) -> Option<CivilDate> {
        match self {
            Self::Single(_) => None,
            Self::Range(range) => range.end,
        }
    }
    pub fn is_range(self) -> bool {
        matches!(self, Self::Range(_))
    }
    pub fn valid(self, rules: &DateConstraints) -> bool {
        match self {
            Self::Single(date) => date.is_some_and(|d| rules.allows(d)),
            Self::Range(range) => range
                .start
                .zip(range.end)
                .is_some_and(|(a, b)| rules.allows_range(a, b)),
        }
    }
    pub fn choose(self, date: CivilDate) -> Self {
        match self {
            Self::Single(_) => Self::Single(Some(date)),
            Self::Range(range) => Self::Range(match (range.start, range.end) {
                (Some(start), None) if date >= start => DateRange {
                    start: Some(start),
                    end: Some(date),
                },
                _ => DateRange {
                    start: Some(date),
                    end: None,
                },
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dates_validate_and_cross_calendar_boundaries() {
        assert!(CivilDate::new(2100, 2, 29).is_none());
        let leap = CivilDate::new(2024, 2, 29).unwrap();
        assert_eq!(leap.add_months(12).iso(), "2025-02-28");
        assert_eq!(leap.add_days(1).iso(), "2024-03-01");
        assert_eq!(CivilDate::new(2026, 1, 1).unwrap().add_days(-1).iso(), "2025-12-31");
        assert_eq!(leap.add_days(i32::MAX).iso(), "9999-12-31");
        assert_eq!(leap.add_days(i32::MIN).iso(), "0001-01-01");
        assert_eq!(leap.add_months(i32::MAX).iso(), "9999-12-29");
        assert_eq!(leap.weekday(), 4);
    }
    #[test]
    fn manual_input_is_strict_but_accepts_unmasked_separators() {
        for text in ["02/29/2024", "02-29-2024", "02.29.2024", "02 29 2024", "02292024"] {
            assert_eq!(CivilDate::parse(text, DateInputFormat::MonthDayYear).unwrap().iso(), "2024-02-29");
        }
        for text in ["02/30/2024", "02/29/2100", "2/29/24", "2024-02-29abc"] {
            assert!(CivilDate::parse(text, DateInputFormat::MonthDayYear).is_none());
        }
        assert_eq!(CivilDate::parse("29/02/2024", DateInputFormat::DayMonthYear).unwrap().iso(), "2024-02-29");
    }
    #[test]
    fn range_drafts_restart_and_can_exclude_unavailable_days() {
        let a = CivilDate::new(2026, 10, 11).unwrap();
        let b = a.add_days(4);
        let selected = DateSelection::Range(DateRange::default()).choose(a).choose(b);
        assert!(selected.valid(&DateConstraints::default()));
        let rules = DateConstraints { disabled_dates: vec![a.add_days(2)], exclude_disabled_in_range: true, ..Default::default() };
        assert!(!selected.valid(&rules));
        assert_eq!(selected.choose(b).end(), None);
        assert!(!DateSelection::Range(DateRange { start: Some(b), end: Some(a) }).valid(&DateConstraints::default()));
    }
}
