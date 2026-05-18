//! EventKit virtual conference provider descriptors.

use serde::{Deserialize, Serialize};

use crate::error::EventKitError;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};

/// Indicates that the EventKit virtual conference provider API is extension-only.
pub const EK_VIRTUAL_CONFERENCE_PROVIDER_IS_EXTENSION_ONLY: bool = true;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit room-type descriptor used by virtual conferences.
pub struct EKVirtualConferenceRoomTypeDescriptor {
    /// Mirrors the EventKit `title` property.
    pub title: String,
    /// Mirrors the EventKit `identifier` property.
    pub identifier: String,
}

impl EKVirtualConferenceRoomTypeDescriptor {
    /// Creates a new EventKit `EKVirtualConferenceRoomTypeDescriptor` value.
    pub fn new(title: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            identifier: identifier.into(),
        }
    }

    /// Round-trips this EventKit `EKVirtualConferenceRoomTypeDescriptor` through the native bridge.
    pub fn roundtrip(&self) -> Result<Self, EventKitError> {
        let payload = json_cstring(self, "EKVirtualConferenceRoomTypeDescriptor")?;
        let mut error = core::ptr::null_mut();
        let json = unsafe {
            ffi::virtual_conference_provider::ek_virtual_conference_room_type_roundtrip_json(
                payload.as_ptr(),
                &mut error,
            )
        };
        if json.is_null() {
            Err(unsafe {
                EventKitError::from_error_ptr(
                    error,
                    "virtual conference room type roundtrip failed",
                )
            })
        } else {
            unsafe { parse_json_ptr(json, "EKVirtualConferenceRoomTypeDescriptor") }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit URL descriptor used by virtual conferences.
pub struct EKVirtualConferenceURLDescriptor {
    /// Mirrors the EventKit `title` property.
    pub title: Option<String>,
    /// Mirrors the EventKit `url` property.
    pub url: String,
}

impl EKVirtualConferenceURLDescriptor {
    /// Creates a new EventKit `EKVirtualConferenceURLDescriptor` value.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            title: None,
            url: url.into(),
        }
    }

    /// Sets the EventKit `title` property on this `EKVirtualConferenceURLDescriptor` value.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Round-trips this EventKit `EKVirtualConferenceURLDescriptor` through the native bridge.
    pub fn roundtrip(&self) -> Result<Self, EventKitError> {
        let payload = json_cstring(self, "EKVirtualConferenceURLDescriptor")?;
        let mut error = core::ptr::null_mut();
        let json = unsafe {
            ffi::virtual_conference_provider::ek_virtual_conference_url_roundtrip_json(
                payload.as_ptr(),
                &mut error,
            )
        };
        if json.is_null() {
            Err(unsafe {
                EventKitError::from_error_ptr(error, "virtual conference URL roundtrip failed")
            })
        } else {
            unsafe { parse_json_ptr(json, "EKVirtualConferenceURLDescriptor") }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit virtual conference descriptor.
pub struct EKVirtualConferenceDescriptor {
    /// Mirrors the EventKit `title` property.
    pub title: Option<String>,
    #[serde(default)]
    /// Mirrors the EventKit `urlDescriptors` property.
    pub url_descriptors: Vec<EKVirtualConferenceURLDescriptor>,
    /// Mirrors the EventKit `conferenceDetails` property.
    pub conference_details: Option<String>,
}

impl EKVirtualConferenceDescriptor {
    /// Creates a new EventKit `EKVirtualConferenceDescriptor` value.
    pub fn new(url_descriptors: Vec<EKVirtualConferenceURLDescriptor>) -> Self {
        Self {
            title: None,
            url_descriptors,
            conference_details: None,
        }
    }

    /// Sets the EventKit `title` property on this `EKVirtualConferenceDescriptor` value.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the EventKit `conferenceDetails` property on this `EKVirtualConferenceDescriptor` value.
    pub fn with_conference_details(mut self, conference_details: impl Into<String>) -> Self {
        self.conference_details = Some(conference_details.into());
        self
    }

    /// Round-trips this EventKit `EKVirtualConferenceDescriptor` through the native bridge.
    pub fn roundtrip(&self) -> Result<Self, EventKitError> {
        let payload = json_cstring(self, "EKVirtualConferenceDescriptor")?;
        let mut error = core::ptr::null_mut();
        let json = unsafe {
            ffi::virtual_conference_provider::ek_virtual_conference_descriptor_roundtrip_json(
                payload.as_ptr(),
                &mut error,
            )
        };
        if json.is_null() {
            Err(unsafe {
                EventKitError::from_error_ptr(
                    error,
                    "virtual conference descriptor roundtrip failed",
                )
            })
        } else {
            unsafe { parse_json_ptr(json, "EKVirtualConferenceDescriptor") }
        }
    }
}
