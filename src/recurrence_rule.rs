//! EventKit recurrence rule types and helpers.

use serde::{Deserialize, Serialize};

use crate::error::EventKitError;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit recurrence frequency.
pub enum EKRecurrenceFrequency {
    /// Matches the EventKit `daily` case.
    Daily,
    /// Matches the EventKit `weekly` case.
    Weekly,
    /// Matches the EventKit `monthly` case.
    Monthly,
    /// Matches the EventKit `yearly` case.
    Yearly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit weekday value used by recurrence rules.
pub enum EKWeekday {
    /// Matches the EventKit `sunday` case.
    Sunday,
    /// Matches the EventKit `monday` case.
    Monday,
    /// Matches the EventKit `tuesday` case.
    Tuesday,
    /// Matches the EventKit `wednesday` case.
    Wednesday,
    /// Matches the EventKit `thursday` case.
    Thursday,
    /// Matches the EventKit `friday` case.
    Friday,
    /// Matches the EventKit `saturday` case.
    Saturday,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit `EKRecurrenceDayOfWeek` value.
pub struct EKRecurrenceDayOfWeek {
    /// Mirrors the EventKit `dayOfTheWeek` property.
    pub day_of_the_week: EKWeekday,
    /// Mirrors the EventKit `weekNumber` property.
    pub week_number: i64,
}

impl EKRecurrenceDayOfWeek {
    /// Creates a new EventKit `EKRecurrenceDayOfWeek` value.
    pub const fn new(day_of_the_week: EKWeekday) -> Self {
        Self {
            day_of_the_week,
            week_number: 0,
        }
    }

    /// Sets the EventKit `weekNumber` property on this `EKRecurrenceDayOfWeek` value.
    pub const fn with_week_number(mut self, week_number: i64) -> Self {
        self.week_number = week_number;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit `EKRecurrenceEnd` value.
pub struct EKRecurrenceEnd {
    /// Mirrors the EventKit `endDate` property.
    pub end_date: Option<String>,
    /// Mirrors the EventKit `occurrenceCount` property.
    pub occurrence_count: Option<u64>,
}

impl EKRecurrenceEnd {
    /// Sets the EventKit `endDate` property on this `EKRecurrenceEnd` value.
    pub fn with_end_date(end_date: impl Into<String>) -> Self {
        Self {
            end_date: Some(end_date.into()),
            occurrence_count: None,
        }
    }

    /// Sets the EventKit `occurrenceCount` property on this `EKRecurrenceEnd` value.
    pub const fn with_occurrence_count(occurrence_count: u64) -> Self {
        Self {
            end_date: None,
            occurrence_count: Some(occurrence_count),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents EventKit `EKRecurrenceRule` data.
pub struct EKRecurrenceRule {
    /// Mirrors the EventKit `frequency` property.
    pub frequency: EKRecurrenceFrequency,
    /// Mirrors the EventKit `interval` property.
    pub interval: i64,
    /// Mirrors the EventKit `endDate` property.
    pub end_date: Option<String>,
    /// Mirrors the EventKit `occurrenceCount` property.
    pub occurrence_count: Option<u64>,
    /// Mirrors the EventKit `calendarIdentifier` property.
    pub calendar_identifier: Option<String>,
    /// Mirrors the EventKit `firstDayOfTheWeek` property.
    pub first_day_of_the_week: Option<EKWeekday>,
    #[serde(default)]
    /// Mirrors the EventKit `daysOfTheWeek` property.
    pub days_of_the_week: Vec<EKRecurrenceDayOfWeek>,
    #[serde(default)]
    /// Mirrors the EventKit `daysOfTheMonth` property.
    pub days_of_the_month: Vec<i64>,
    #[serde(default)]
    /// Mirrors the EventKit `monthsOfTheYear` property.
    pub months_of_the_year: Vec<i64>,
    #[serde(default)]
    /// Mirrors the EventKit `weeksOfTheYear` property.
    pub weeks_of_the_year: Vec<i64>,
    #[serde(default)]
    /// Mirrors the EventKit `daysOfTheYear` property.
    pub days_of_the_year: Vec<i64>,
    #[serde(default)]
    /// Mirrors the EventKit `setPositions` property.
    pub set_positions: Vec<i64>,
}

impl EKRecurrenceRule {
    /// Creates a new EventKit `EKRecurrenceRule` value.
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

    /// Sets the EventKit `interval` property on this `EKRecurrenceRule` value.
    pub const fn with_interval(mut self, interval: i64) -> Self {
        self.interval = interval;
        self
    }

    /// Sets the EventKit `endDate` property on this `EKRecurrenceRule` value.
    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self.occurrence_count = None;
        self
    }

    /// Sets the EventKit `occurrenceCount` property on this `EKRecurrenceRule` value.
    pub fn with_occurrence_count(mut self, occurrence_count: u64) -> Self {
        self.occurrence_count = Some(occurrence_count);
        self.end_date = None;
        self
    }

    /// Sets the EventKit `firstDayOfTheWeek` property on this `EKRecurrenceRule` value.
    pub const fn with_first_day_of_the_week(mut self, weekday: EKWeekday) -> Self {
        self.first_day_of_the_week = Some(weekday);
        self
    }

    /// Sets the EventKit `daysOfTheWeek` property on this `EKRecurrenceRule` value.
    pub fn with_days_of_the_week(
        mut self,
        days_of_the_week: impl IntoIterator<Item = EKRecurrenceDayOfWeek>,
    ) -> Self {
        self.days_of_the_week = days_of_the_week.into_iter().collect();
        self
    }

    /// Sets the EventKit `daysOfTheMonth` property on this `EKRecurrenceRule` value.
    pub fn with_days_of_the_month(
        mut self,
        days_of_the_month: impl IntoIterator<Item = i64>,
    ) -> Self {
        self.days_of_the_month = days_of_the_month.into_iter().collect();
        self
    }

    /// Sets the EventKit `monthsOfTheYear` property on this `EKRecurrenceRule` value.
    pub fn with_months_of_the_year(
        mut self,
        months_of_the_year: impl IntoIterator<Item = i64>,
    ) -> Self {
        self.months_of_the_year = months_of_the_year.into_iter().collect();
        self
    }

    /// Sets the EventKit `weeksOfTheYear` property on this `EKRecurrenceRule` value.
    pub fn with_weeks_of_the_year(
        mut self,
        weeks_of_the_year: impl IntoIterator<Item = i64>,
    ) -> Self {
        self.weeks_of_the_year = weeks_of_the_year.into_iter().collect();
        self
    }

    /// Sets the EventKit `daysOfTheYear` property on this `EKRecurrenceRule` value.
    pub fn with_days_of_the_year(
        mut self,
        days_of_the_year: impl IntoIterator<Item = i64>,
    ) -> Self {
        self.days_of_the_year = days_of_the_year.into_iter().collect();
        self
    }

    /// Sets the EventKit `setPositions` property on this `EKRecurrenceRule` value.
    pub fn with_set_positions(mut self, set_positions: impl IntoIterator<Item = i64>) -> Self {
        self.set_positions = set_positions.into_iter().collect();
        self
    }

    /// Returns the EventKit recurrence end represented by this rule.
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

    /// Round-trips this EventKit `EKRecurrenceRule` through the native bridge.
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
