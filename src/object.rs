//! EventKit live-object wrappers.

use core::ffi::{c_char, c_void};
use std::fmt;
use std::ptr::{self, NonNull};

use crate::calendar::EKCalendarDraft;
use crate::error::EventKitError;
use crate::event::EKEvent;
use crate::event_store::EKEventStore;
use crate::ffi;
use crate::private::json_cstring;
use crate::reminder::EKReminder;

/// Wraps a live EventKit `EKObject` instance.
pub struct EKObject {
    raw: NonNull<c_void>,
}

impl fmt::Debug for EKObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EKObject").finish_non_exhaustive()
    }
}

impl EKObject {
    fn from_raw(
        raw: *mut c_void,
        error: *mut c_char,
        context: &str,
    ) -> Result<Self, EventKitError> {
        NonNull::new(raw).map(|raw| Self { raw }).ok_or_else(|| {
            if error.is_null() {
                EventKitError::OperationFailed(format!("failed to create {context}"))
            } else {
                unsafe { EventKitError::from_error_ptr(error, &format!("{context} failed")) }
            }
        })
    }

    /// Wraps a live EventKit event as `EKObject`.
    pub fn from_event(store: &EKEventStore, event: &EKEvent) -> Result<Self, EventKitError> {
        let payload = json_cstring(event, "EKEvent")?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::object::ek_object_from_event_json(store.as_raw_ptr(), payload.as_ptr(), &mut error)
        };
        Self::from_raw(raw, error, "EKObject from EKEvent")
    }

    /// Wraps a live EventKit reminder as `EKObject`.
    pub fn from_reminder(
        store: &EKEventStore,
        reminder: &EKReminder,
    ) -> Result<Self, EventKitError> {
        let payload = json_cstring(reminder, "EKReminder")?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::object::ek_object_from_reminder_json(
                store.as_raw_ptr(),
                payload.as_ptr(),
                &mut error,
            )
        };
        Self::from_raw(raw, error, "EKObject from EKReminder")
    }

    /// Wraps a live EventKit calendar draft as `EKObject`.
    pub fn from_calendar_draft(
        store: &EKEventStore,
        calendar: &EKCalendarDraft,
    ) -> Result<Self, EventKitError> {
        let payload = json_cstring(calendar, "EKCalendarDraft")?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::object::ek_object_from_calendar_draft_json(
                store.as_raw_ptr(),
                payload.as_ptr(),
                &mut error,
            )
        };
        Self::from_raw(raw, error, "EKObject from EKCalendarDraft")
    }

    /// Returns whether the wrapped EventKit object has unsaved changes.
    pub fn has_changes(&self) -> bool {
        unsafe { ffi::object::ek_object_has_changes(self.raw.as_ptr()) }
    }

    /// Returns whether the wrapped EventKit object is new to EventKit.
    pub fn is_new(&self) -> bool {
        unsafe { ffi::object::ek_object_is_new(self.raw.as_ptr()) }
    }

    /// Resets the wrapped EventKit object to its last committed state.
    pub fn reset(&self) {
        unsafe { ffi::object::ek_object_reset(self.raw.as_ptr()) };
    }

    /// Rolls back pending EventKit changes on the wrapped object.
    pub fn rollback(&self) {
        unsafe { ffi::object::ek_object_rollback(self.raw.as_ptr()) };
    }

    /// Refreshes the wrapped EventKit object from the store.
    pub fn refresh(&self) -> bool {
        unsafe { ffi::object::ek_object_refresh(self.raw.as_ptr()) }
    }
}

impl Drop for EKObject {
    fn drop(&mut self) {
        unsafe { ffi::object::ek_object_release(self.raw.as_ptr()) };
    }
}
