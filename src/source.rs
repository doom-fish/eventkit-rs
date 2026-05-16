use std::ptr;

use serde::{Deserialize, Serialize};

use crate::calendar::EKCalendar;
use crate::error::EventKitError;
use crate::event_store::{EKEntityType, EKEventStore};
use crate::ffi;
use crate::private::{cstring_from_str, parse_json_ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EKSourceType {
    #[default]
    Local,
    Exchange,
    CalDav,
    MobileMe,
    Subscribed,
    Birthdays,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EKSource {
    pub identifier: String,
    pub source_type: EKSourceType,
    pub title: String,
    pub is_delegate: bool,
}

impl EKSource {
    pub fn calendars_for_entity_type(
        &self,
        store: &EKEventStore,
        entity_type: EKEntityType,
    ) -> Result<Vec<EKCalendar>, EventKitError> {
        let identifier = cstring_from_str(&self.identifier, "EKSource identifier")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::source::ek_store_source_calendars_json(
                store.as_raw_ptr(),
                identifier.as_ptr(),
                entity_type.as_raw(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                EventKitError::from_error_ptr(error, "source calendarsForEntityType failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "EKCalendar list") }
        }
    }
}
