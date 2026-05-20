//! EventKit event snapshots and helpers.

use std::cmp::Ordering;
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
use crate::structured_location::EKStructuredLocation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit `EKEventAvailability` value.
pub enum EKEventAvailability {
    /// Matches the EventKit `notSupported` case.
    NotSupported,
    #[default]
    /// Matches the EventKit `busy` case.
    Busy,
    /// Matches the EventKit `free` case.
    Free,
    /// Matches the EventKit `tentative` case.
    Tentative,
    /// Matches the EventKit `unavailable` case.
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit `EKEventStatus` value.
pub enum EKEventStatus {
    #[default]
    /// Matches the EventKit `none` case.
    None,
    /// Matches the EventKit `confirmed` case.
    Confirmed,
    /// Matches the EventKit `tentative` case.
    Tentative,
    /// Matches the EventKit `canceled` case.
    Canceled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents EventKit `EKEvent` data.
pub struct EKEvent {
    /// Mirrors the EventKit `identifier` property.
    pub identifier: Option<String>,
    /// Mirrors the EventKit `title` property.
    pub title: String,
    /// Mirrors the EventKit `startDate` property.
    pub start_date: String,
    /// Mirrors the EventKit `endDate` property.
    pub end_date: String,
    /// Mirrors the EventKit `calendarIdentifier` property.
    pub calendar_identifier: Option<String>,
    /// Mirrors the EventKit `calendar` property.
    pub calendar: Option<EKCalendar>,
    /// Mirrors the EventKit `notes` property.
    pub notes: Option<String>,
    /// Mirrors the EventKit `location` property.
    pub location: Option<String>,
    #[serde(default)]
    /// Mirrors the EventKit `alarms` property.
    pub alarms: Vec<EKAlarm>,
    #[serde(default)]
    /// Mirrors the EventKit `recurrenceRules` property.
    pub recurrence_rules: Vec<EKRecurrenceRule>,
    /// Mirrors the EventKit `calendarItemIdentifier` property.
    pub calendar_item_identifier: Option<String>,
    /// Mirrors the EventKit `calendarItemExternalIdentifier` property.
    pub calendar_item_external_identifier: Option<String>,
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
    /// Mirrors the EventKit `allDay` property.
    pub all_day: bool,
    /// Mirrors the EventKit `structuredLocation` property.
    pub structured_location: Option<EKStructuredLocation>,
    /// Mirrors the EventKit `organizer` property.
    pub organizer: Option<EKParticipant>,
    /// Mirrors the EventKit `availability` property.
    pub availability: EKEventAvailability,
    /// Mirrors the EventKit `status` property.
    pub status: EKEventStatus,
    /// Mirrors the EventKit `isDetached` property.
    pub is_detached: bool,
    /// Mirrors the EventKit `occurrenceDate` property.
    pub occurrence_date: Option<String>,
    /// Mirrors the EventKit `birthdayContactIdentifier` property.
    pub birthday_contact_identifier: Option<String>,
    /// Mirrors the EventKit `birthdayPersonId` property.
    pub birthday_person_id: Option<i64>,
    /// Mirrors the EventKit `birthdayPersonUniqueId` property.
    pub birthday_person_unique_id: Option<String>,
}

impl EKEvent {
    /// Creates a new EventKit `EKEvent` value.
    pub fn new(
        title: impl Into<String>,
        start_date: impl Into<String>,
        end_date: impl Into<String>,
    ) -> Self {
        Self {
            identifier: None,
            title: title.into(),
            start_date: start_date.into(),
            end_date: end_date.into(),
            calendar_identifier: None,
            calendar: None,
            notes: None,
            location: None,
            alarms: Vec::new(),
            recurrence_rules: Vec::new(),
            calendar_item_identifier: None,
            calendar_item_external_identifier: None,
            url: None,
            last_modified_date: None,
            creation_date: None,
            time_zone_identifier: None,
            has_alarms: false,
            has_recurrence_rules: false,
            has_attendees: false,
            has_notes: false,
            attendees: Vec::new(),
            all_day: false,
            structured_location: None,
            organizer: None,
            availability: EKEventAvailability::Busy,
            status: EKEventStatus::None,
            is_detached: false,
            occurrence_date: None,
            birthday_contact_identifier: None,
            birthday_person_id: None,
            birthday_person_unique_id: None,
        }
    }

    /// Sets the EventKit `calendarIdentifier` property on this `EKEvent` value.
    pub fn with_calendar_identifier(mut self, calendar_identifier: impl Into<String>) -> Self {
        self.calendar_identifier = Some(calendar_identifier.into());
        self
    }

    /// Sets the EventKit `allDay` property on this `EKEvent` value.
    pub fn with_all_day(mut self, all_day: bool) -> Self {
        self.all_day = all_day;
        self
    }

    /// Sets the EventKit `structuredLocation` property on this `EKEvent` value.
    pub fn with_structured_location(mut self, structured_location: EKStructuredLocation) -> Self {
        self.structured_location = Some(structured_location);
        self
    }

    /// Compares this EventKit event to another by start date.
    pub fn compare_start_date(&self, other: &Self) -> Result<Ordering, EventKitError> {
        let lhs = json_cstring(self, "EKEvent")?;
        let rhs = json_cstring(other, "EKEvent")?;
        let mut error = ptr::null_mut();
        let value = unsafe {
            ffi::event::ek_event_compare_start_date_json(lhs.as_ptr(), rhs.as_ptr(), &mut error)
        };
        if error.is_null() {
            Ok(match value {
                x if x < 0 => Ordering::Less,
                0 => Ordering::Equal,
                _ => Ordering::Greater,
            })
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "compareStartDateWithEvent failed") })
        }
    }

    /// Round-trips this EventKit event through the native bridge using the given store.
    pub fn roundtrip_in(&self, store: &EKEventStore) -> Result<Self, EventKitError> {
        let payload = json_cstring(self, "EKEvent")?;
        let mut error = ptr::null_mut();
        let json = unsafe {
            ffi::event::ek_event_roundtrip_json(store.as_raw_ptr(), payload.as_ptr(), &mut error)
        };
        if json.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "event roundtrip failed") })
        } else {
            unsafe { parse_json_ptr(json, "EKEvent") }
        }
    }

    /// Wraps this EventKit event as a live `EKObject` in the given store.
    pub fn as_object_in(&self, store: &EKEventStore) -> Result<EKObject, EventKitError> {
        EKObject::from_event(store, self)
    }

    /// Reloads this EventKit event from the given store when it has an identifier.
    pub fn refresh_in(&self, store: &EKEventStore) -> Result<Option<Self>, EventKitError> {
        let Some(identifier) = &self.identifier else {
            return Ok(None);
        };
        store.refresh_event(identifier)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn event_status_round_trips_through_json() {
        let value = serde_json::to_string(&EKEventStatus::Tentative).unwrap();
        let decoded: EKEventStatus = serde_json::from_str(&value).unwrap();

        assert_eq!(decoded, EKEventStatus::Tentative);
    }

    #[test]
    fn event_builder_sets_defaults() {
        let event = EKEvent::new("Demo", "2026-01-01T10:00:00Z", "2026-01-01T11:00:00Z");

        assert_eq!(event.title, "Demo");
        assert_eq!(event.availability, EKEventAvailability::Busy);
        assert_eq!(event.status, EKEventStatus::None);
        assert!(!event.all_day);
        assert!(!event.has_alarms);
    }

    #[test]
    fn event_builder_helpers_set_optional_fields() {
        let location = EKStructuredLocation::new("HQ");
        let event = EKEvent::new("Demo", "2026-01-01T10:00:00Z", "2026-01-01T11:00:00Z")
            .with_calendar_identifier("calendar-1")
            .with_all_day(true)
            .with_structured_location(location);

        assert_eq!(event.calendar_identifier.as_deref(), Some("calendar-1"));
        assert!(event.all_day);
        assert_eq!(
            event.structured_location.and_then(|value| value.title),
            Some("HQ".to_owned())
        );
    }

    #[test]
    fn events_compare_by_start_date() {
        let earlier = EKEvent::new("Earlier", "2026-01-01T09:00:00Z", "2026-01-01T10:00:00Z");
        let later = EKEvent::new("Later", "2026-01-01T11:00:00Z", "2026-01-01T12:00:00Z");

        assert_eq!(earlier.compare_start_date(&later).unwrap(), Ordering::Less);
    }
}
