//! EventKit error and authorization types.

use core::fmt;
use std::ffi::CStr;

use serde::{Deserialize, Serialize};

use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
/// Represents the EventKit authorization status for an entity type.
pub enum EKAuthorizationStatus {
    /// Matches the EventKit `notDetermined` case.
    NotDetermined,
    /// Matches the EventKit `restricted` case.
    Restricted,
    /// Matches the EventKit `denied` case.
    Denied,
    /// Matches the EventKit `fullAccess` case.
    FullAccess,
    /// Matches the EventKit `writeOnly` case.
    WriteOnly,
    /// Preserves an unknown raw EventKit authorization status.
    Unknown(i32),
}

impl EKAuthorizationStatus {
    pub(crate) const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::NotDetermined,
            1 => Self::Restricted,
            2 => Self::Denied,
            3 => Self::FullAccess,
            4 => Self::WriteOnly,
            other => Self::Unknown(other),
        }
    }

    /// Returns whether this EventKit authorization status permits access.
    pub const fn is_authorized(self) -> bool {
        matches!(self, Self::FullAccess | Self::WriteOnly)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Captures an `NSError` surfaced by EventKit.
pub struct NSErrorInfo {
    /// Mirrors the EventKit `domain` property.
    pub domain: String,
    /// Mirrors the EventKit `code` property.
    pub code: i64,
    /// Mirrors the EventKit `message` property.
    pub message: String,
}

impl fmt::Display for NSErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) [{}]", self.message, self.code, self.domain)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Represents errors returned while working with EventKit.
pub enum EventKitError {
    /// Reports an argument error detected before calling EventKit.
    InvalidArgument(String),
    /// Wraps an `NSError` returned by EventKit.
    Framework(NSErrorInfo),
    /// Reports an EventKit operation failure message.
    OperationFailed(String),
}

impl fmt::Display for EventKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::Framework(error) => write!(f, "EventKit.framework error: {error}"),
            Self::OperationFailed(message) => write!(f, "eventkit operation failed: {message}"),
        }
    }
}

impl std::error::Error for EventKitError {}

impl EventKitError {
    pub(crate) unsafe fn from_error_ptr(error_ptr: *mut core::ffi::c_char, fallback: &str) -> Self {
        if error_ptr.is_null() {
            return Self::OperationFailed(fallback.to_owned());
        }

        let message = CStr::from_ptr(error_ptr).to_string_lossy().into_owned();
        ffi::ek_string_free(error_ptr);

        if let Ok(payload) = serde_json::from_str::<NSErrorInfo>(&message) {
            Self::Framework(payload)
        } else {
            Self::OperationFailed(message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn framework_error() -> NSErrorInfo {
        NSErrorInfo {
            domain: "EKErrorDomain".to_owned(),
            code: 2,
            message: "write denied".to_owned(),
        }
    }

    #[test]
    fn authorization_status_maps_known_framework_values() {
        assert_eq!(EKAuthorizationStatus::from_raw(0), EKAuthorizationStatus::NotDetermined);
        assert_eq!(EKAuthorizationStatus::from_raw(1), EKAuthorizationStatus::Restricted);
        assert_eq!(EKAuthorizationStatus::from_raw(2), EKAuthorizationStatus::Denied);
        assert_eq!(EKAuthorizationStatus::from_raw(3), EKAuthorizationStatus::FullAccess);
        assert_eq!(EKAuthorizationStatus::from_raw(4), EKAuthorizationStatus::WriteOnly);
    }

    #[test]
    fn authorization_status_preserves_unknown_values() {
        assert_eq!(EKAuthorizationStatus::from_raw(99), EKAuthorizationStatus::Unknown(99));
    }

    #[test]
    fn authorization_status_detects_authorized_states() {
        assert!(!EKAuthorizationStatus::Denied.is_authorized());
        assert!(EKAuthorizationStatus::FullAccess.is_authorized());
        assert!(EKAuthorizationStatus::WriteOnly.is_authorized());
    }

    #[test]
    fn ns_error_info_display_includes_message_code_and_domain() {
        assert_eq!(framework_error().to_string(), "write denied (2) [EKErrorDomain]");
    }

    #[test]
    fn eventkit_error_display_formats_each_variant() {
        assert_eq!(
            EventKitError::InvalidArgument("bad input".to_owned()).to_string(),
            "invalid argument: bad input"
        );
        assert_eq!(
            EventKitError::Framework(framework_error()).to_string(),
            "EventKit.framework error: write denied (2) [EKErrorDomain]"
        );
        assert_eq!(
            EventKitError::OperationFailed("bridge failed".to_owned()).to_string(),
            "eventkit operation failed: bridge failed"
        );
    }
}
