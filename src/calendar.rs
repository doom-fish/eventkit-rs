//! EventKit calendar snapshots and draft helpers.

use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::EventKitError;
use crate::event_store::{EKEntityType, EKEventStore};
use crate::ffi;
use crate::object::EKObject;
use crate::private::{json_cstring, parse_json_ptr, Redacted};
use crate::source::EKSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit `EKCalendarType` value.
pub enum EKCalendarType {
    #[default]
    /// Matches the EventKit `local` case.
    Local,
    /// Matches the EventKit `calDav` case.
    CalDav,
    /// Matches the EventKit `exchange` case.
    Exchange,
    /// Matches the EventKit `subscription` case.
    Subscription,
    /// Matches the EventKit `birthday` case.
    Birthday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit availability values supported by a calendar.
pub enum EKCalendarEventAvailability {
    /// Matches the EventKit `busy` case.
    Busy,
    /// Matches the EventKit `free` case.
    Free,
    /// Matches the EventKit `tentative` case.
    Tentative,
    /// Matches the EventKit `unavailable` case.
    Unavailable,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents EventKit `EKCalendar` data.
pub struct EKCalendar {
    /// Mirrors the EventKit `identifier` property.
    pub identifier: String,
    /// Mirrors the EventKit `title` property.
    pub title: String,
    /// Mirrors the EventKit `calendarType` property.
    pub calendar_type: EKCalendarType,
    #[serde(default)]
    /// Mirrors the EventKit `allowedEntityTypes` property.
    pub allowed_entity_types: Vec<EKEntityType>,
    /// Mirrors the EventKit `color` property.
    pub color: Option<String>,
    /// Mirrors the EventKit `source` property.
    pub source: Option<EKSource>,
    /// Mirrors the EventKit `allowsContentModifications` property.
    pub allows_content_modifications: bool,
    /// Mirrors the EventKit `isSubscribed` property.
    pub is_subscribed: bool,
    /// Mirrors the EventKit `isImmutable` property.
    pub is_immutable: bool,
    #[serde(default)]
    /// Mirrors the EventKit `supportedEventAvailabilities` property.
    pub supported_event_availabilities: Vec<EKCalendarEventAvailability>,
}

impl core::fmt::Debug for EKCalendar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EKCalendar")
            .field("identifier", &self.identifier)
            .field("title", &Redacted)
            .field("calendar_type", &self.calendar_type)
            .field("allowed_entity_types", &self.allowed_entity_types)
            .field("color", &self.color)
            .field("source", &self.source)
            .field(
                "allows_content_modifications",
                &self.allows_content_modifications,
            )
            .field("is_subscribed", &self.is_subscribed)
            .field("is_immutable", &self.is_immutable)
            .field(
                "supported_event_availabilities",
                &self.supported_event_availabilities,
            )
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents editable EventKit `EKCalendar` input data.
pub struct EKCalendarDraft {
    /// Mirrors the EventKit `identifier` property.
    pub identifier: Option<String>,
    /// Mirrors the EventKit `entityType` property.
    pub entity_type: EKEntityType,
    /// Mirrors the EventKit `sourceIdentifier` property.
    pub source_identifier: Option<String>,
    /// Mirrors the EventKit `title` property.
    pub title: String,
    /// Mirrors the EventKit `color` property.
    pub color: Option<String>,
}

impl core::fmt::Debug for EKCalendarDraft {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EKCalendarDraft")
            .field("identifier", &self.identifier)
            .field("entity_type", &self.entity_type)
            .field("source_identifier", &self.source_identifier)
            .field("title", &Redacted)
            .field("color", &self.color)
            .finish()
    }
}

impl EKCalendarDraft {
    /// Creates a new EventKit `EKCalendarDraft` value.
    pub fn new(entity_type: EKEntityType, title: impl Into<String>) -> Self {
        Self {
            identifier: None,
            entity_type,
            source_identifier: None,
            title: title.into(),
            color: None,
        }
    }

    /// Sets the EventKit `identifier` property on this `EKCalendarDraft` value.
    pub fn with_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }

    /// Sets the EventKit `sourceIdentifier` property on this `EKCalendarDraft` value.
    pub fn with_source_identifier(mut self, source_identifier: impl Into<String>) -> Self {
        self.source_identifier = Some(source_identifier.into());
        self
    }

    /// Sets the EventKit `color` property on this `EKCalendarDraft` value.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Round-trips this EventKit calendar draft through the native bridge.
    pub fn roundtrip(&self, store: &EKEventStore) -> Result<EKCalendar, EventKitError> {
        let payload = json_cstring(self, "EKCalendarDraft")?;
        let mut error = ptr::null_mut();
        let json = unsafe {
            ffi::calendar::ek_calendar_roundtrip_json(
                store.as_raw_ptr(),
                payload.as_ptr(),
                &raw mut error,
            )
        };
        if json.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "calendar roundtrip failed") })
        } else {
            unsafe { parse_json_ptr(json, "EKCalendar") }
        }
    }

    /// Wraps this EventKit calendar draft as a live `EKObject` in the given store.
    pub fn as_object_in(&self, store: &EKEventStore) -> Result<EKObject, EventKitError> {
        EKObject::from_calendar_draft(store, self)
    }
}
