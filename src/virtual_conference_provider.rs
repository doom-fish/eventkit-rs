use serde::{Deserialize, Serialize};

use crate::error::EventKitError;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};

pub const EK_VIRTUAL_CONFERENCE_PROVIDER_IS_EXTENSION_ONLY: bool = true;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EKVirtualConferenceRoomTypeDescriptor {
    pub title: String,
    pub identifier: String,
}

impl EKVirtualConferenceRoomTypeDescriptor {
    pub fn new(title: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            identifier: identifier.into(),
        }
    }

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
pub struct EKVirtualConferenceURLDescriptor {
    pub title: Option<String>,
    pub url: String,
}

impl EKVirtualConferenceURLDescriptor {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            title: None,
            url: url.into(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

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
pub struct EKVirtualConferenceDescriptor {
    pub title: Option<String>,
    #[serde(default)]
    pub url_descriptors: Vec<EKVirtualConferenceURLDescriptor>,
    pub conference_details: Option<String>,
}

impl EKVirtualConferenceDescriptor {
    pub fn new(url_descriptors: Vec<EKVirtualConferenceURLDescriptor>) -> Self {
        Self {
            title: None,
            url_descriptors,
            conference_details: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_conference_details(mut self, conference_details: impl Into<String>) -> Self {
        self.conference_details = Some(conference_details.into());
        self
    }

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
