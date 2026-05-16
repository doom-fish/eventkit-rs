use core::ffi::c_void;
use std::ptr::{self, NonNull};

use crate::error::{EKAuthorizationStatus, EventKitError};
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};
use crate::types::{
    EKCalendar, EKEntityType, EKEvent, EKEventPredicate, EKReminder, EKReminderPredicate, EKSpan,
};

#[derive(Debug)]
pub struct EKEventStore {
    raw: NonNull<c_void>,
}

fn calendar_identifiers(calendars: Option<&[EKCalendar]>) -> Option<Vec<String>> {
    if let Some(calendars) = calendars {
        Some(
            calendars
                .iter()
                .map(|calendar| calendar.identifier.clone())
                .collect(),
        )
    } else {
        None
    }
}

impl EKEventStore {
    pub fn new() -> Result<Self, EventKitError> {
        let raw = NonNull::new(unsafe { ffi::ek_store_new() }).ok_or_else(|| {
            EventKitError::OperationFailed("failed to create EKEventStore".to_owned())
        })?;
        Ok(Self { raw })
    }

    pub fn authorization_status(entity_type: EKEntityType) -> EKAuthorizationStatus {
        EKAuthorizationStatus::from_raw(unsafe {
            ffi::ek_authorization_status(entity_type.as_raw())
        })
    }

    pub fn request_access_to_events(&self) -> Result<bool, EventKitError> {
        let mut error = ptr::null_mut();
        let granted = unsafe { ffi::ek_store_request_access_events(self.raw.as_ptr(), &mut error) };
        if error.is_null() {
            Ok(granted)
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "request access to events failed") })
        }
    }

    pub fn request_access_to_reminders(&self) -> Result<bool, EventKitError> {
        let mut error = ptr::null_mut();
        let granted =
            unsafe { ffi::ek_store_request_access_reminders(self.raw.as_ptr(), &mut error) };
        if error.is_null() {
            Ok(granted)
        } else {
            Err(unsafe {
                EventKitError::from_error_ptr(error, "request access to reminders failed")
            })
        }
    }

    pub fn calendars_for_entity_type(
        &self,
        entity_type: EKEntityType,
    ) -> Result<Vec<EKCalendar>, EventKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::ek_store_calendars_json(self.raw.as_ptr(), entity_type.as_raw(), &mut error)
        };
        if payload.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "calendarsForEntityType failed") })
        } else {
            unsafe { parse_json_ptr(payload, "EKCalendar list") }
        }
    }

    pub fn predicate_for_events(
        &self,
        start_date: impl Into<String>,
        end_date: impl Into<String>,
        calendars: Option<&[EKCalendar]>,
    ) -> EKEventPredicate {
        let mut predicate = EKEventPredicate::new(start_date, end_date);
        predicate.calendar_identifiers = calendar_identifiers(calendars);
        predicate
    }

    pub fn predicate_for_reminders(&self, calendars: Option<&[EKCalendar]>) -> EKReminderPredicate {
        let mut predicate = EKReminderPredicate::new();
        predicate.calendar_identifiers = calendar_identifiers(calendars);
        predicate
    }

    pub fn events_matching(
        &self,
        predicate: &EKEventPredicate,
    ) -> Result<Vec<EKEvent>, EventKitError> {
        let predicate_json = json_cstring(predicate, "EKEventPredicate")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::ek_store_events_matching_json(
                self.raw.as_ptr(),
                predicate_json.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "eventsMatching failed") })
        } else {
            unsafe { parse_json_ptr(payload, "EKEvent list") }
        }
    }

    pub fn fetch_reminders_matching(
        &self,
        predicate: &EKReminderPredicate,
    ) -> Result<Vec<EKReminder>, EventKitError> {
        let predicate_json = json_cstring(predicate, "EKReminderPredicate")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::ek_store_fetch_reminders_json(
                self.raw.as_ptr(),
                predicate_json.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "fetchReminders failed") })
        } else {
            unsafe { parse_json_ptr(payload, "EKReminder list") }
        }
    }

    pub fn save_event(
        &self,
        event: &EKEvent,
        span: EKSpan,
        commit: bool,
    ) -> Result<(), EventKitError> {
        let event_json = json_cstring(event, "EKEvent")?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ek_store_save_event(
                self.raw.as_ptr(),
                event_json.as_ptr(),
                span.as_raw(),
                commit,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "saveEvent failed") })
        }
    }

    pub fn remove_event(
        &self,
        event: &EKEvent,
        span: EKSpan,
        commit: bool,
    ) -> Result<(), EventKitError> {
        let event_json = json_cstring(event, "EKEvent")?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ek_store_remove_event(
                self.raw.as_ptr(),
                event_json.as_ptr(),
                span.as_raw(),
                commit,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "removeEvent failed") })
        }
    }

    pub fn save_reminder(&self, reminder: &EKReminder, commit: bool) -> Result<(), EventKitError> {
        let reminder_json = json_cstring(reminder, "EKReminder")?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ek_store_save_reminder(
                self.raw.as_ptr(),
                reminder_json.as_ptr(),
                commit,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "saveReminder failed") })
        }
    }

    pub fn remove_reminder(
        &self,
        reminder: &EKReminder,
        commit: bool,
    ) -> Result<(), EventKitError> {
        let reminder_json = json_cstring(reminder, "EKReminder")?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ek_store_remove_reminder(
                self.raw.as_ptr(),
                reminder_json.as_ptr(),
                commit,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "removeReminder failed") })
        }
    }

    pub fn commit(&self) -> Result<(), EventKitError> {
        let mut error = ptr::null_mut();
        let status = unsafe { ffi::ek_store_commit(self.raw.as_ptr(), &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "commit failed") })
        }
    }
}

impl Drop for EKEventStore {
    fn drop(&mut self) {
        unsafe { ffi::ek_store_release(self.raw.as_ptr()) };
    }
}
