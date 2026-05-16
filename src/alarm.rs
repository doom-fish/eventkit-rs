use serde::{Deserialize, Serialize};

use crate::error::EventKitError;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};
use crate::structured_location::EKStructuredLocation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EKAlarmProximity {
    None,
    Enter,
    Leave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EKAlarmType {
    Display,
    Audio,
    Procedure,
    Email,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EKAlarm {
    pub absolute_date: Option<String>,
    pub relative_offset: Option<f64>,
    pub structured_location: Option<EKStructuredLocation>,
    pub proximity: Option<EKAlarmProximity>,
    pub alarm_type: Option<EKAlarmType>,
    pub email_address: Option<String>,
    pub sound_name: Option<String>,
    pub url: Option<String>,
}

impl EKAlarm {
    pub fn relative(relative_offset: f64) -> Self {
        Self {
            relative_offset: Some(relative_offset),
            ..Self::default()
        }
    }

    pub fn absolute(absolute_date: impl Into<String>) -> Self {
        Self {
            absolute_date: Some(absolute_date.into()),
            ..Self::default()
        }
    }

    pub fn with_structured_location(mut self, structured_location: EKStructuredLocation) -> Self {
        self.structured_location = Some(structured_location);
        self
    }

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
