/// A time of day without a timezone. Hours are stored in 24-hour form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeValue {
    hours: u8,
    minutes: u8,
}
impl Default for TimeValue {
    fn default() -> Self {
        Self {
            hours: 9,
            minutes: 30,
        }
    }
}
impl TimeValue {
    pub fn new(hours: u8, minutes: u8) -> Option<Self> {
        (hours < 24 && minutes < 60).then_some(Self { hours, minutes })
    }
    pub fn hours(self) -> u8 {
        self.hours
    }
    pub fn minutes(self) -> u8 {
        self.minutes
    }
    pub fn is_pm(self) -> bool {
        self.hours >= 12
    }
    pub fn shown_hour(self, hour24: bool) -> u8 {
        if hour24 {
            self.hours
        } else {
            let h = self.hours % 12;
            if h == 0 { 12 } else { h }
        }
    }
    pub fn format(self, hour24: bool) -> String {
        let time = format!("{:02}:{:02}", self.shown_hour(hour24), self.minutes);
        if hour24 {
            time
        } else {
            format!("{time} {}", if self.is_pm() { "PM" } else { "AM" })
        }
    }
    pub(crate) fn set_period(self, pm: bool) -> Self {
        Self {
            hours: self.hours % 12 + if pm { 12 } else { 0 },
            ..self
        }
    }
    pub(crate) fn set_hour(self, shown: u8, hour24: bool) -> Option<Self> {
        if hour24 {
            Self::new(shown, self.minutes)
        } else if (1..=12).contains(&shown) {
            Self::new(shown % 12 + if self.is_pm() { 12 } else { 0 }, self.minutes)
        } else {
            None
        }
    }
    pub(crate) fn set_minute(self, minutes: u8) -> Option<Self> {
        Self::new(self.hours, minutes)
    }
    pub(crate) fn parse_field(raw: &str, max: u8, min: u8) -> Option<u8> {
        let s = raw.trim();
        if s.is_empty() || s.len() > 2 || !s.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        s.parse::<u8>().ok().filter(|n| *n >= min && *n <= max)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn noon_and_midnight_keep_their_period() {
        let midnight = TimeValue::new(0, 0).unwrap();
        assert_eq!(midnight.format(false), "12:00 AM");
        assert_eq!(midnight.set_period(true).format(false), "12:00 PM");
        assert_eq!(
            TimeValue::new(23, 59)
                .unwrap()
                .set_hour(12, false)
                .unwrap()
                .hours(),
            12
        );
        assert!(TimeValue::new(24, 0).is_none());
        assert!(TimeValue::new(0, 60).is_none());
    }
    #[test]
    fn time_input_rejects_partial_or_out_of_range_values() {
        assert_eq!(TimeValue::parse_field("09", 23, 0), Some(9));
        for s in ["", "24", "2a", "999", "-1"] {
            assert!(TimeValue::parse_field(s, 23, 0).is_none());
        }
        assert!(TimeValue::parse_field("0", 12, 1).is_none());
        assert_eq!(TimeValue::new(13, 5).unwrap().format(true), "13:05");
    }
}
