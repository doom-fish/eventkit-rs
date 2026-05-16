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
pub enum EKEventAvailability {
    NotSupported,
    #[default]
    Busy,
    Free,
    Tentative,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EKEventStatus {
    #[default]
    None,
    Confirmed,
    Tentative,
    Canceled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EKEvent {
    pub identifier: Option<String>,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub calendar_identifier: Option<String>,
    pub calendar: Option<EKCalendar>,
    pub notes: Option<String>,
    pub location: Option<String>,
    #[serde(default)]
    pub alarms: Vec<EKAlarm>,
    #[serde(default)]
    pub recurrence_rules: Vec<EKRecurrenceRule>,
    pub calendar_item_identifier: Option<String>,
    pub calendar_item_external_identifier: Option<String>,
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
    pub all_day: bool,
    pub structured_location: Option<EKStructuredLocation>,
    pub organizer: Option<EKParticipant>,
    pub availability: EKEventAvailability,
    pub status: EKEventStatus,
    pub is_detached: bool,
    pub occurrence_date: Option<String>,
    pub birthday_contact_identifier: Option<String>,
    pub birthday_person_id: Option<i64>,
    pub birthday_person_unique_id: Option<String>,
}

impl EKEvent {
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

    pub fn with_calendar_identifier(mut self, calendar_identifier: impl Into<String>) -> Self {
        self.calendar_identifier = Some(calendar_identifier.into());
        self
    }

    pub fn with_all_day(mut self, all_day: bool) -> Self {
        self.all_day = all_day;
        self
    }

    pub fn with_structured_location(mut self, structured_location: EKStructuredLocation) -> Self {
        self.structured_location = Some(structured_location);
        self
    }

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

    pub fn as_object_in(&self, store: &EKEventStore) -> Result<EKObject, EventKitError> {
        EKObject::from_event(store, self)
    }

    pub fn refresh_in(&self, store: &EKEventStore) -> Result<Option<Self>, EventKitError> {
        let Some(identifier) = &self.identifier else {
            return Ok(None);
        };
        store.refresh_event(identifier)
    }
}
