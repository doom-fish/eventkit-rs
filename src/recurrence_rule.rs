use serde::{Deserialize, Serialize};

use crate::error::EventKitError;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EKRecurrenceFrequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EKWeekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EKRecurrenceDayOfWeek {
    pub day_of_the_week: EKWeekday,
    pub week_number: i64,
}

impl EKRecurrenceDayOfWeek {
    pub const fn new(day_of_the_week: EKWeekday) -> Self {
        Self {
            day_of_the_week,
            week_number: 0,
        }
    }

    pub const fn with_week_number(mut self, week_number: i64) -> Self {
        self.week_number = week_number;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EKRecurrenceEnd {
    pub end_date: Option<String>,
    pub occurrence_count: Option<u64>,
}

impl EKRecurrenceEnd {
    pub fn with_end_date(end_date: impl Into<String>) -> Self {
        Self {
            end_date: Some(end_date.into()),
            occurrence_count: None,
        }
    }

    pub const fn with_occurrence_count(occurrence_count: u64) -> Self {
        Self {
            end_date: None,
            occurrence_count: Some(occurrence_count),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EKRecurrenceRule {
    pub frequency: EKRecurrenceFrequency,
    pub interval: i64,
    pub end_date: Option<String>,
    pub occurrence_count: Option<u64>,
    pub calendar_identifier: Option<String>,
    pub first_day_of_the_week: Option<EKWeekday>,
    #[serde(default)]
    pub days_of_the_week: Vec<EKRecurrenceDayOfWeek>,
    #[serde(default)]
    pub days_of_the_month: Vec<i64>,
    #[serde(default)]
    pub months_of_the_year: Vec<i64>,
    #[serde(default)]
    pub weeks_of_the_year: Vec<i64>,
    #[serde(default)]
    pub days_of_the_year: Vec<i64>,
    #[serde(default)]
    pub set_positions: Vec<i64>,
}

impl EKRecurrenceRule {
    pub fn new(frequency: EKRecurrenceFrequency) -> Self {
        Self {
            frequency,
            interval: 1,
            end_date: None,
            occurrence_count: None,
            calendar_identifier: None,
            first_day_of_the_week: None,
            days_of_the_week: Vec::new(),
            days_of_the_month: Vec::new(),
            months_of_the_year: Vec::new(),
            weeks_of_the_year: Vec::new(),
            days_of_the_year: Vec::new(),
            set_positions: Vec::new(),
        }
    }

    pub const fn with_interval(mut self, interval: i64) -> Self {
        self.interval = interval;
        self
    }

    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self.occurrence_count = None;
        self
    }

    pub fn with_occurrence_count(mut self, occurrence_count: u64) -> Self {
        self.occurrence_count = Some(occurrence_count);
        self.end_date = None;
        self
    }

    pub const fn with_first_day_of_the_week(mut self, weekday: EKWeekday) -> Self {
        self.first_day_of_the_week = Some(weekday);
        self
    }

    pub fn with_days_of_the_week(
        mut self,
        days_of_the_week: impl IntoIterator<Item = EKRecurrenceDayOfWeek>,
    ) -> Self {
        self.days_of_the_week = days_of_the_week.into_iter().collect();
        self
    }

    pub fn with_days_of_the_month(
        mut self,
        days_of_the_month: impl IntoIterator<Item = i64>,
    ) -> Self {
        self.days_of_the_month = days_of_the_month.into_iter().collect();
        self
    }

    pub fn with_months_of_the_year(
        mut self,
        months_of_the_year: impl IntoIterator<Item = i64>,
    ) -> Self {
        self.months_of_the_year = months_of_the_year.into_iter().collect();
        self
    }

    pub fn with_weeks_of_the_year(
        mut self,
        weeks_of_the_year: impl IntoIterator<Item = i64>,
    ) -> Self {
        self.weeks_of_the_year = weeks_of_the_year.into_iter().collect();
        self
    }

    pub fn with_days_of_the_year(
        mut self,
        days_of_the_year: impl IntoIterator<Item = i64>,
    ) -> Self {
        self.days_of_the_year = days_of_the_year.into_iter().collect();
        self
    }

    pub fn with_set_positions(mut self, set_positions: impl IntoIterator<Item = i64>) -> Self {
        self.set_positions = set_positions.into_iter().collect();
        self
    }

    pub fn recurrence_end(&self) -> Option<EKRecurrenceEnd> {
        if self.end_date.is_none() && self.occurrence_count.is_none() {
            None
        } else {
            Some(EKRecurrenceEnd {
                end_date: self.end_date.clone(),
                occurrence_count: self.occurrence_count,
            })
        }
    }

    pub fn roundtrip(&self) -> Result<Self, EventKitError> {
        let payload = json_cstring(self, "EKRecurrenceRule")?;
        let mut error = core::ptr::null_mut();
        let json = unsafe {
            ffi::recurrence_rule::ek_recurrence_rule_roundtrip_json(payload.as_ptr(), &mut error)
        };
        if json.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "recurrence rule roundtrip failed") })
        } else {
            unsafe { parse_json_ptr(json, "EKRecurrenceRule") }
        }
    }
}
