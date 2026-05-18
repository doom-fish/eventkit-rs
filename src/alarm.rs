//! EventKit alarm types and helpers.

use serde::{Deserialize, Serialize};

use crate::error::EventKitError;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};
use crate::structured_location::EKStructuredLocation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit `EKAlarmProximity` value.
pub enum EKAlarmProximity {
    /// Matches the EventKit `none` case.
    None,
    /// Matches the EventKit `enter` case.
    Enter,
    /// Matches the EventKit `leave` case.
    Leave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit `EKAlarmType` value.
pub enum EKAlarmType {
    /// Matches the EventKit `display` case.
    Display,
    /// Matches the EventKit `audio` case.
    Audio,
    /// Matches the EventKit `procedure` case.
    Procedure,
    /// Matches the EventKit `email` case.
    Email,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents EventKit `EKAlarm` data.
pub struct EKAlarm {
    /// Mirrors the EventKit `absoluteDate` property.
    pub absolute_date: Option<String>,
    /// Mirrors the EventKit `relativeOffset` property.
    pub relative_offset: Option<f64>,
    /// Mirrors the EventKit `structuredLocation` property.
    pub structured_location: Option<EKStructuredLocation>,
    /// Mirrors the EventKit `proximity` property.
    pub proximity: Option<EKAlarmProximity>,
    /// Mirrors the EventKit `alarmType` property.
    pub alarm_type: Option<EKAlarmType>,
    /// Mirrors the EventKit `emailAddress` property.
    pub email_address: Option<String>,
    /// Mirrors the EventKit `soundName` property.
    pub sound_name: Option<String>,
    /// Mirrors the EventKit `url` property.
    pub url: Option<String>,
}

impl EKAlarm {
    /// Creates an EventKit alarm with a relative offset.
    pub fn relative(relative_offset: f64) -> Self {
        Self {
            relative_offset: Some(relative_offset),
            ..Self::default()
        }
    }

    /// Creates an EventKit alarm with an absolute date.
    pub fn absolute(absolute_date: impl Into<String>) -> Self {
        Self {
            absolute_date: Some(absolute_date.into()),
            ..Self::default()
        }
    }

    /// Sets the EventKit `structuredLocation` property on this `EKAlarm` value.
    pub fn with_structured_location(mut self, structured_location: EKStructuredLocation) -> Self {
        self.structured_location = Some(structured_location);
        self
    }

    /// Round-trips this EventKit `EKAlarm` through the native bridge.
    pub fn roundtrip(&self) -> Result<Self, EventKitError> {
        let payload = json_cstring(self, "EKAlarm")?;
        let mut error = core::ptr::null_mut();
        let json = unsafe { ffi::alarm::ek_alarm_roundtrip_json(payload.as_ptr(), &mut error) };
        if json.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "alarm roundtrip failed") })
        } else {
            unsafe { parse_json_ptr(json, "EKAlarm") }
        }
    }
}
