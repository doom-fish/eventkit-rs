use std::ptr;

use serde::{Deserialize, Serialize};

use crate::alarm::EKAlarm;
use crate::calendar::EKCalendar;
use crate::error::EventKitError;
use crate::event_store::EKEventStore;
use crate::ffi;
use crate::participant::EKParticipant;
use crate::private::{json_cstring, parse_json_ptr};
use crate::recurrence_rule::EKRecurrenceRule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NSDateComponents {
    pub era: Option<i32>,
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
    pub hour: Option<i32>,
    pub minute: Option<i32>,
    pub second: Option<i32>,
    pub nanosecond: Option<i32>,
    pub weekday: Option<i32>,
    pub weekday_ordinal: Option<i32>,
    pub quarter: Option<i32>,
    pub week_of_month: Option<i32>,
    pub week_of_year: Option<i32>,
    pub year_for_week_of_year: Option<i32>,
    pub is_leap_month: Option<bool>,
    pub time_zone_identifier: Option<String>,
    pub calendar_identifier: Option<String>,
}

impl NSDateComponents {
    pub fn date(year: i32, month: i32, day: i32) -> Self {
        Self {
            year: Some(year),
            month: Some(month),
            day: Some(day),
            ..Self::default()
        }
    }

    pub fn with_time(mut self, hour: i32, minute: i32, second: i32) -> Self {
        self.hour = Some(hour);
        self.minute = Some(minute);
        self.second = Some(second);
        self
    }

    pub fn with_time_zone_identifier(mut self, time_zone_identifier: impl Into<String>) -> Self {
        self.time_zone_identifier = Some(time_zone_identifier.into());
        self
    }

    pub fn with_calendar_identifier(mut self, calendar_identifier: impl Into<String>) -> Self {
        self.calendar_identifier = Some(calendar_identifier.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EKReminderPriority {
    None,
    High,
    Medium,
    Low,
    Custom(u64),
}

impl EKReminderPriority {
    pub const fn as_raw(self) -> u64 {
        match self {
            Self::None => 0,
            Self::High => 1,
            Self::Medium => 5,
            Self::Low => 9,
            Self::Custom(value) => value,
        }
    }

    pub const fn from_raw(raw: u64) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::High,
            5 => Self::Medium,
            9 => Self::Low,
            other => Self::Custom(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EKReminder {
    pub identifier: Option<String>,
    pub title: String,
    pub calendar_identifier: Option<String>,
    pub calendar: Option<EKCalendar>,
    pub due_date_components: Option<NSDateComponents>,
    pub is_completed: bool,
    pub priority: u64,
    pub notes: Option<String>,
    #[serde(default)]
    pub alarms: Vec<EKAlarm>,
    #[serde(default)]
    pub recurrence_rules: Vec<EKRecurrenceRule>,
    pub start_date_components: Option<NSDateComponents>,
    pub completion_date: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub last_modified_date: Option<String>,
    pub creation_date: Option<String>,
    pub time_zone_identifier: Option<String>,
    pub has_alarms: bool,
    pub has_recurrence_rules: bool,
    pub has_attendees: bool,
    pub has_notes: bool,
    #[serde(default)]
    pub attendees: Vec<EKParticipant>,
}

impl EKReminder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            identifier: None,
            title: title.into(),
            calendar_identifier: None,
            calendar: None,
            due_date_components: None,
            is_completed: false,
            priority: EKReminderPriority::None.as_raw(),
            notes: None,
            alarms: Vec::new(),
            recurrence_rules: Vec::new(),
            start_date_components: None,
            completion_date: None,
            location: None,
            url: None,
            last_modified_date: None,
            creation_date: None,
            time_zone_identifier: None,
            has_alarms: false,
            has_recurrence_rules: false,
            has_attendees: false,
            has_notes: false,
            attendees: Vec::new(),
        }
    }

    pub fn with_calendar_identifier(mut self, calendar_identifier: impl Into<String>) -> Self {
        self.calendar_identifier = Some(calendar_identifier.into());
        self
    }

    pub fn with_priority_kind(mut self, priority: EKReminderPriority) -> Self {
        self.priority = priority.as_raw();
        self
    }

    pub const fn priority_kind(&self) -> EKReminderPriority {
        EKReminderPriority::from_raw(self.priority)
    }

    pub fn roundtrip_in(&self, store: &EKEventStore) -> Result<Self, EventKitError> {
        let payload = json_cstring(self, "EKReminder")?;
        let mut error = ptr::null_mut();
        let json = unsafe {
            ffi::reminder::ek_reminder_roundtrip_json(
                store.as_raw_ptr(),
                payload.as_ptr(),
                &mut error,
            )
        };
        if json.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "reminder roundtrip failed") })
        } else {
            unsafe { parse_json_ptr(json, "EKReminder") }
        }
    }
}
