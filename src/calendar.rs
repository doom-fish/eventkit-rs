use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::EventKitError;
use crate::event_store::{EKEntityType, EKEventStore};
use crate::ffi;
use crate::object::EKObject;
use crate::private::{json_cstring, parse_json_ptr};
use crate::source::EKSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EKCalendarType {
    #[default]
    Local,
    CalDav,
    Exchange,
    Subscription,
    Birthday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EKCalendarEventAvailability {
    Busy,
    Free,
    Tentative,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EKCalendar {
    pub identifier: String,
    pub title: String,
    pub calendar_type: EKCalendarType,
    #[serde(default)]
    pub allowed_entity_types: Vec<EKEntityType>,
    pub color: Option<String>,
    pub source: Option<EKSource>,
    pub allows_content_modifications: bool,
    pub is_subscribed: bool,
    pub is_immutable: bool,
    #[serde(default)]
    pub supported_event_availabilities: Vec<EKCalendarEventAvailability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EKCalendarDraft {
    pub identifier: Option<String>,
    pub entity_type: EKEntityType,
    pub source_identifier: Option<String>,
    pub title: String,
    pub color: Option<String>,
}

impl EKCalendarDraft {
    pub fn new(entity_type: EKEntityType, title: impl Into<String>) -> Self {
        Self {
            identifier: None,
            entity_type,
            source_identifier: None,
            title: title.into(),
            color: None,
        }
    }

    pub fn with_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }

    pub fn with_source_identifier(mut self, source_identifier: impl Into<String>) -> Self {
        self.source_identifier = Some(source_identifier.into());
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn roundtrip(&self, store: &EKEventStore) -> Result<EKCalendar, EventKitError> {
        let payload = json_cstring(self, "EKCalendarDraft")?;
        let mut error = ptr::null_mut();
        let json = unsafe {
            ffi::calendar::ek_calendar_roundtrip_json(
                store.as_raw_ptr(),
                payload.as_ptr(),
                &mut error,
            )
        };
        if json.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "calendar roundtrip failed") })
        } else {
            unsafe { parse_json_ptr(json, "EKCalendar") }
        }
    }

    pub fn as_object_in(&self, store: &EKEventStore) -> Result<EKObject, EventKitError> {
        EKObject::from_calendar_draft(store, self)
    }
}
