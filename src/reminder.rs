//! EventKit reminder snapshots and date components.

use std::ptr;

use serde::{Deserialize, Serialize};

use crate::alarm::EKAlarm;
use crate::calendar::EKCalendar;
use crate::error::EventKitError;
use crate::event_store::EKEventStore;
use crate::ffi;
use crate::object::EKObject;
use crate::participant::EKParticipant;
use crate::private::{json_cstring, parse_json_ptr};
use crate::recurrence_rule::EKRecurrenceRule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents EventKit `NSDateComponents` data used by reminders.
pub struct NSDateComponents {
    /// Mirrors the EventKit `era` property.
    pub era: Option<i32>,
    /// Mirrors the EventKit `year` property.
    pub year: Option<i32>,
    /// Mirrors the EventKit `month` property.
    pub month: Option<i32>,
    /// Mirrors the EventKit `day` property.
    pub day: Option<i32>,
    /// Mirrors the EventKit `hour` property.
    pub hour: Option<i32>,
    /// Mirrors the EventKit `minute` property.
    pub minute: Option<i32>,
    /// Mirrors the EventKit `second` property.
    pub second: Option<i32>,
    /// Mirrors the EventKit `nanosecond` property.
    pub nanosecond: Option<i32>,
    /// Mirrors the EventKit `weekday` property.
    pub weekday: Option<i32>,
    /// Mirrors the EventKit `weekdayOrdinal` property.
    pub weekday_ordinal: Option<i32>,
    /// Mirrors the EventKit `quarter` property.
    pub quarter: Option<i32>,
    /// Mirrors the EventKit `weekOfMonth` property.
    pub week_of_month: Option<i32>,
    /// Mirrors the EventKit `weekOfYear` property.
    pub week_of_year: Option<i32>,
    /// Mirrors the EventKit `yearForWeekOfYear` property.
    pub year_for_week_of_year: Option<i32>,
    /// Mirrors the EventKit `isLeapMonth` property.
    pub is_leap_month: Option<bool>,
    /// Mirrors the EventKit `timeZoneIdentifier` property.
    pub time_zone_identifier: Option<String>,
    /// Mirrors the EventKit `calendarIdentifier` property.
    pub calendar_identifier: Option<String>,
}

impl NSDateComponents {
    /// Creates EventKit date components for a calendar date.
    pub fn date(year: i32, month: i32, day: i32) -> Self {
        Self {
            year: Some(year),
            month: Some(month),
            day: Some(day),
            ..Self::default()
        }
    }

    /// Sets the EventKit `time` property on this `NSDateComponents` value.
    pub fn with_time(mut self, hour: i32, minute: i32, second: i32) -> Self {
        self.hour = Some(hour);
        self.minute = Some(minute);
        self.second = Some(second);
        self
    }

    /// Sets the EventKit `timeZoneIdentifier` property on this `NSDateComponents` value.
    pub fn with_time_zone_identifier(mut self, time_zone_identifier: impl Into<String>) -> Self {
        self.time_zone_identifier = Some(time_zone_identifier.into());
        self
    }

    /// Sets the EventKit `calendarIdentifier` property on this `NSDateComponents` value.
    pub fn with_calendar_identifier(mut self, calendar_identifier: impl Into<String>) -> Self {
        self.calendar_identifier = Some(calendar_identifier.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents the EventKit reminder priority.
pub enum EKReminderPriority {
    /// Matches the EventKit `none` case.
    None,
    /// Matches the EventKit `high` case.
    High,
    /// Matches the EventKit `medium` case.
    Medium,
    /// Matches the EventKit `low` case.
    Low,
    /// Stores a custom raw EventKit reminder priority.
    Custom(u64),
}

impl EKReminderPriority {
    /// Returns the raw EventKit priority value.
    pub const fn as_raw(self) -> u64 {
        match self {
            Self::None => 0,
            Self::High => 1,
            Self::Medium => 5,
            Self::Low => 9,
            Self::Custom(value) => value,
        }
    }

    /// Builds an EventKit reminder priority from a raw value.
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
/// Represents EventKit `EKReminder` data.
pub struct EKReminder {
    /// Mirrors the EventKit `identifier` property.
    pub identifier: Option<String>,
    /// Mirrors the EventKit `title` property.
    pub title: String,
    /// Mirrors the EventKit `calendarIdentifier` property.
    pub calendar_identifier: Option<String>,
    /// Mirrors the EventKit `calendar` property.
    pub calendar: Option<EKCalendar>,
    /// Mirrors the EventKit `dueDateComponents` property.
    pub due_date_components: Option<NSDateComponents>,
    /// Mirrors the EventKit `isCompleted` property.
    pub is_completed: bool,
    /// Mirrors the EventKit `priority` property.
    pub priority: u64,
    /// Mirrors the EventKit `notes` property.
    pub notes: Option<String>,
    #[serde(default)]
    /// Mirrors the EventKit `alarms` property.
    pub alarms: Vec<EKAlarm>,
    #[serde(default)]
    /// Mirrors the EventKit `recurrenceRules` property.
    pub recurrence_rules: Vec<EKRecurrenceRule>,
    /// Mirrors the EventKit `startDateComponents` property.
    pub start_date_components: Option<NSDateComponents>,
    /// Mirrors the EventKit `completionDate` property.
    pub completion_date: Option<String>,
    /// Mirrors the EventKit `location` property.
    pub location: Option<String>,
    /// Mirrors the EventKit `url` property.
    pub url: Option<String>,
    /// Mirrors the EventKit `lastModifiedDate` property.
    pub last_modified_date: Option<String>,
    /// Mirrors the EventKit `creationDate` property.
    pub creation_date: Option<String>,
    /// Mirrors the EventKit `timeZoneIdentifier` property.
    pub time_zone_identifier: Option<String>,
    /// Mirrors the EventKit `hasAlarms` property.
    pub has_alarms: bool,
    /// Mirrors the EventKit `hasRecurrenceRules` property.
    pub has_recurrence_rules: bool,
    /// Mirrors the EventKit `hasAttendees` property.
    pub has_attendees: bool,
    /// Mirrors the EventKit `hasNotes` property.
    pub has_notes: bool,
    #[serde(default)]
    /// Mirrors the EventKit `attendees` property.
    pub attendees: Vec<EKParticipant>,
}

impl EKReminder {
    /// Creates a new EventKit `EKReminder` value.
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

    /// Sets the EventKit `calendarIdentifier` property on this `EKReminder` value.
    pub fn with_calendar_identifier(mut self, calendar_identifier: impl Into<String>) -> Self {
        self.calendar_identifier = Some(calendar_identifier.into());
        self
    }

    /// Sets the EventKit `priorityKind` property on this `EKReminder` value.
    pub fn with_priority_kind(mut self, priority: EKReminderPriority) -> Self {
        self.priority = priority.as_raw();
        self
    }

    /// Returns this reminder's EventKit priority as an enum.
    pub const fn priority_kind(&self) -> EKReminderPriority {
        EKReminderPriority::from_raw(self.priority)
    }

    /// Wraps this EventKit reminder as a live `EKObject` in the given store.
    pub fn as_object_in(&self, store: &EKEventStore) -> Result<EKObject, EventKitError> {
        EKObject::from_reminder(store, self)
    }

    /// Round-trips this EventKit reminder through the native bridge using the given store.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_components_date_helper_sets_calendar_date() {
        let components = NSDateComponents::date(2026, 1, 15);

        assert_eq!(components.year, Some(2026));
        assert_eq!(components.month, Some(1));
        assert_eq!(components.day, Some(15));
    }

    #[test]
    fn date_components_time_and_identifiers_set_fields() {
        let components = NSDateComponents::date(2026, 5, 20)
            .with_time(9, 30, 45)
            .with_time_zone_identifier("Europe/Stockholm")
            .with_calendar_identifier("gregorian");

        assert_eq!(components.hour, Some(9));
        assert_eq!(components.minute, Some(30));
        assert_eq!(components.second, Some(45));
        assert_eq!(components.time_zone_identifier.as_deref(), Some("Europe/Stockholm"));
        assert_eq!(components.calendar_identifier.as_deref(), Some("gregorian"));
    }

    #[test]
    fn reminder_priority_round_trips_raw_values() {
        for priority in [
            EKReminderPriority::None,
            EKReminderPriority::High,
            EKReminderPriority::Medium,
            EKReminderPriority::Low,
            EKReminderPriority::Custom(7),
        ] {
            assert_eq!(EKReminderPriority::from_raw(priority.as_raw()), priority);
        }
    }

    #[test]
    fn reminder_builder_sets_priority_and_calendar_identifier() {
        let reminder = EKReminder::new("Ship 0.3.7")
            .with_calendar_identifier("calendar-1")
            .with_priority_kind(EKReminderPriority::High);

        assert_eq!(reminder.title, "Ship 0.3.7");
        assert_eq!(reminder.calendar_identifier.as_deref(), Some("calendar-1"));
        assert_eq!(reminder.priority_kind(), EKReminderPriority::High);
        assert!(!reminder.is_completed);
        assert!(!reminder.has_alarms);
    }

    #[test]
    fn reminder_priority_kind_maps_custom_values() {
        let mut reminder = EKReminder::new("Investigate custom priority");
        reminder.priority = 7;

        assert_eq!(reminder.priority_kind(), EKReminderPriority::Custom(7));
    }
}
