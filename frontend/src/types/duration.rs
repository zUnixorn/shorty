use std::fmt::{Display, Formatter};

use crate::components::duration_input::Selection;

pub struct Duration {
    pub seconds: i64,
}

impl Duration {
    const MAX_SECONDS: i64 = 99 * 60 * 60 + 59 * 60 + 59;
    pub const ZERO: Self = Self { seconds: 0 };

    pub fn to_parts(&self) -> Parts {
        let hours = self.seconds / Parts::SECONDS_HOUR;
        let reminder = self.seconds % Parts::SECONDS_HOUR;
        let minutes = reminder / Parts::SECONDS_MINUTES;
        let seconds = reminder % Parts::SECONDS_MINUTES;

        Parts {
            hours,
            minutes,
            seconds,
        }
    }

    pub fn from_parts(parts: Parts) -> Self {
        Self {
            seconds: parts.to_seconds(),
        }
    }

    pub fn add_parts(&mut self, parts: Parts) {
        let seconds = self.seconds
            + parts.seconds
            + parts.minutes * Parts::SECONDS_MINUTES
            + parts.hours * Parts::SECONDS_HOUR;

        if seconds > Self::MAX_SECONDS || seconds < 0 {
            return;
        }

        self.seconds = seconds;
    }

    pub fn reset(&mut self) {
        self.seconds = 0;
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parts = self.to_parts();

        write!(
            f,
            "{:02}:{:02}:{:02}",
            parts.hours, parts.minutes, parts.seconds
        )
    }
}

#[derive(Copy, Clone)]
pub struct Parts {
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

impl Parts {
    const SECONDS_HOUR: i64 = 60 * 60;
    const SECONDS_MINUTES: i64 = 60;

    pub fn zero_selection(&mut self, selection: Selection) {
        match selection {
            Selection::Hours => self.hours = 0,
            Selection::Minutes => self.minutes = 0,
            Selection::Seconds => self.seconds = 0,
        }
    }

    pub fn to_seconds(&self) -> i64 {
        self.hours * Self::SECONDS_HOUR + self.minutes * Self::SECONDS_MINUTES + self.seconds
    }

    pub fn valid(&self) -> bool {
        self.seconds < 60 && self.minutes < 60 && self.hours < 100
    }
}

impl TryFrom<&str> for Parts {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let values = value
            .split(':')
            .map(|v| v.parse::<i64>())
            .collect::<Result<Vec<_>, _>>();

        if values.is_err() {
            return Err(());
        }

        let values = values.unwrap();

        if values.len() != 3 {
            return Err(());
        }

        Ok(Self {
            hours: values[0],
            minutes: values[1],
            seconds: values[2],
        })
    }
}
