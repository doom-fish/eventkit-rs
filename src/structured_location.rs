//! EventKit structured-location types and helpers.

use serde::{Deserialize, Serialize};

use crate::error::EventKitError;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit location coordinates used by structured locations.
pub struct EKGeoLocation {
    /// Mirrors the EventKit `latitude` property.
    pub latitude: f64,
    /// Mirrors the EventKit `longitude` property.
    pub longitude: f64,
    /// Mirrors the EventKit `altitude` property.
    pub altitude: Option<f64>,
    /// Mirrors the EventKit `horizontalAccuracy` property.
    pub horizontal_accuracy: Option<f64>,
    /// Mirrors the EventKit `verticalAccuracy` property.
    pub vertical_accuracy: Option<f64>,
}

impl EKGeoLocation {
    /// Creates a new EventKit `EKGeoLocation` value.
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
/// Represents EventKit `EKStructuredLocation` data.
pub struct EKStructuredLocation {
    /// Mirrors the EventKit `title` property.
    pub title: Option<String>,
    /// Mirrors the EventKit `geoLocation` property.
    pub geo_location: Option<EKGeoLocation>,
    /// Mirrors the EventKit `radius` property.
    pub radius: f64,
}

impl EKStructuredLocation {
    /// Creates a new EventKit `EKStructuredLocation` value.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            ..Self::default()
        }
    }

    /// Sets the EventKit `geoLocation` property on this `EKStructuredLocation` value.
    pub fn with_geo_location(mut self, geo_location: EKGeoLocation) -> Self {
        self.geo_location = Some(geo_location);
        self
    }

    /// Sets the EventKit `radius` property on this `EKStructuredLocation` value.
    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Round-trips this EventKit `EKStructuredLocation` through the native bridge.
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
