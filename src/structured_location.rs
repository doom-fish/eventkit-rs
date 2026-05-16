use serde::{Deserialize, Serialize};

use crate::error::EventKitError;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EKGeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub horizontal_accuracy: Option<f64>,
    pub vertical_accuracy: Option<f64>,
}

impl EKGeoLocation {
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            altitude: None,
            horizontal_accuracy: None,
            vertical_accuracy: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EKStructuredLocation {
    pub title: Option<String>,
    pub geo_location: Option<EKGeoLocation>,
    pub radius: f64,
}

impl EKStructuredLocation {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            ..Self::default()
        }
    }

    pub fn with_geo_location(mut self, geo_location: EKGeoLocation) -> Self {
        self.geo_location = Some(geo_location);
        self
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn roundtrip(&self) -> Result<Self, EventKitError> {
        let payload = json_cstring(self, "EKStructuredLocation")?;
        let mut error = core::ptr::null_mut();
        let json = unsafe {
            ffi::structured_location::ek_structured_location_roundtrip_json(
                payload.as_ptr(),
                &mut error,
            )
        };
        if json.is_null() {
            Err(unsafe {
                EventKitError::from_error_ptr(error, "structured location roundtrip failed")
            })
        } else {
            unsafe { parse_json_ptr(json, "EKStructuredLocation") }
        }
    }
}
