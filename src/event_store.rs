use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::calendar::{EKCalendar, EKCalendarDraft};
use crate::error::{EKAuthorizationStatus, EventKitError};
use crate::event::EKEvent;
use crate::ffi;
use crate::private::{cstring_from_str, json_cstring, parse_json_ptr, take_string};
use crate::reminder::EKReminder;
use crate::source::EKSource;

pub const EK_EVENT_STORE_CHANGED_NOTIFICATION: &str = "EKEventStoreChangedNotification";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EKEntityType {
    Event,
    Reminder,
}

impl EKEntityType {
    pub(crate) const fn as_raw(self) -> i32 {
        match self {
            Self::Event => 0,
            Self::Reminder => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EKSpan {
    #[default]
    ThisEvent,
    FutureEvents,
}

impl EKSpan {
    pub(crate) const fn as_raw(self) -> i32 {
        match self {
            Self::ThisEvent => 0,
            Self::FutureEvents => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EKEventPredicate {
    pub start_date: String,
    pub end_date: String,
    pub calendar_identifiers: Option<Vec<String>>,
}

impl EKEventPredicate {
    pub fn new(start_date: impl Into<String>, end_date: impl Into<String>) -> Self {
        Self {
            start_date: start_date.into(),
            end_date: end_date.into(),
            calendar_identifiers: None,
        }
    }

    pub fn with_calendar_identifiers(
        mut self,
        calendar_identifiers: impl IntoIterator<Item = String>,
    ) -> Self {
        self.calendar_identifiers = Some(calendar_identifiers.into_iter().collect());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EKReminderPredicateKind {
    #[default]
    All,
    Incomplete,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EKReminderPredicate {
    pub calendar_identifiers: Option<Vec<String>>,
    pub kind: EKReminderPredicateKind,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl EKReminderPredicate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn incomplete() -> Self {
        Self {
            kind: EKReminderPredicateKind::Incomplete,
            ..Self::default()
        }
    }

    pub fn completed() -> Self {
        Self {
            kind: EKReminderPredicateKind::Completed,
            ..Self::default()
        }
    }

    pub fn with_calendar_identifiers(
        mut self,
        calendar_identifiers: impl IntoIterator<Item = String>,
    ) -> Self {
        self.calendar_identifiers = Some(calendar_identifiers.into_iter().collect());
        self
    }

    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EKCalendarItemKind {
    Event,
    Reminder,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EKCalendarItem {
    pub kind: EKCalendarItemKind,
    pub event: Option<EKEvent>,
    pub reminder: Option<EKReminder>,
}

impl EKCalendarItem {
    pub fn as_event(&self) -> Option<&EKEvent> {
        self.event.as_ref()
    }

    pub fn as_reminder(&self) -> Option<&EKReminder> {
        self.reminder.as_ref()
    }
}

#[derive(Debug)]
pub struct EKEventStore {
    raw: NonNull<c_void>,
}

fn calendar_identifiers(calendars: Option<&[EKCalendar]>) -> Option<Vec<String>> {
    calendars.map(|calendars| {
        calendars
            .iter()
            .map(|calendar| calendar.identifier.clone())
            .collect()
    })
}

unsafe fn parse_optional_json_ptr<T: serde::de::DeserializeOwned>(
    payload: *mut core::ffi::c_char,
    error: *mut core::ffi::c_char,
    context: &str,
) -> Result<Option<T>, EventKitError> {
    if payload.is_null() {
        if error.is_null() {
            Ok(None)
        } else {
            Err(EventKitError::from_error_ptr(
                error,
                &format!("{context} failed"),
            ))
        }
    } else {
        parse_json_ptr(payload, context).map(Some)
    }
}

impl EKEventStore {
    pub fn new() -> Result<Self, EventKitError> {
        let raw = NonNull::new(unsafe { ffi::event_store::ek_store_new() }).ok_or_else(|| {
            EventKitError::OperationFailed("failed to create EKEventStore".to_owned())
        })?;
        Ok(Self { raw })
    }

    pub fn with_source_identifiers<I, S>(source_identifiers: I) -> Result<Self, EventKitError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let source_identifiers: Vec<String> =
            source_identifiers.into_iter().map(Into::into).collect();
        if source_identifiers.is_empty() {
            return Self::new();
        }

        let json = json_cstring(&source_identifiers, "EKSource identifier list")?;
        let mut error = ptr::null_mut();
        let raw = NonNull::new(unsafe {
            ffi::event_store::ek_store_new_with_sources_json(json.as_ptr(), &mut error)
        })
        .ok_or_else(|| unsafe {
            EventKitError::from_error_ptr(error, "failed to create EKEventStore with sources")
        })?;
        Ok(Self { raw })
    }

    pub(crate) const fn as_raw_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    pub fn authorization_status(entity_type: EKEntityType) -> EKAuthorizationStatus {
        EKAuthorizationStatus::from_raw(unsafe {
            ffi::event_store::ek_authorization_status(entity_type.as_raw())
        })
    }

    pub fn request_access_to_events(&self) -> Result<bool, EventKitError> {
        let mut error = ptr::null_mut();
        let granted = unsafe {
            ffi::event_store::ek_store_request_access_events(self.raw.as_ptr(), &mut error)
        };
        if error.is_null() {
            Ok(granted)
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "request access to events failed") })
        }
    }

    pub fn request_full_access_to_events(&self) -> Result<bool, EventKitError> {
        let mut error = ptr::null_mut();
        let granted = unsafe {
            ffi::event_store::ek_store_request_full_access_events(self.raw.as_ptr(), &mut error)
        };
        if error.is_null() {
            Ok(granted)
        } else {
            Err(unsafe {
                EventKitError::from_error_ptr(error, "request full access to events failed")
            })
        }
    }

    pub fn request_write_only_access_to_events(&self) -> Result<bool, EventKitError> {
        let mut error = ptr::null_mut();
        let granted = unsafe {
            ffi::event_store::ek_store_request_write_only_access_events(
                self.raw.as_ptr(),
                &mut error,
            )
        };
        if error.is_null() {
            Ok(granted)
        } else {
            Err(unsafe {
                EventKitError::from_error_ptr(error, "request write-only access to events failed")
            })
        }
    }

    pub fn request_access_to_reminders(&self) -> Result<bool, EventKitError> {
        let mut error = ptr::null_mut();
        let granted = unsafe {
            ffi::event_store::ek_store_request_access_reminders(self.raw.as_ptr(), &mut error)
        };
        if error.is_null() {
            Ok(granted)
        } else {
            Err(unsafe {
                EventKitError::from_error_ptr(error, "request access to reminders failed")
            })
        }
    }

    pub fn request_full_access_to_reminders(&self) -> Result<bool, EventKitError> {
        let mut error = ptr::null_mut();
        let granted = unsafe {
            ffi::event_store::ek_store_request_full_access_reminders(self.raw.as_ptr(), &mut error)
        };
        if error.is_null() {
            Ok(granted)
        } else {
            Err(unsafe {
                EventKitError::from_error_ptr(error, "request full access to reminders failed")
            })
        }
    }

    pub fn event_store_identifier(&self) -> Result<String, EventKitError> {
        let payload = unsafe { ffi::event_store::ek_store_identifier(self.raw.as_ptr()) };
        unsafe { take_string(payload) }.ok_or_else(|| {
            EventKitError::OperationFailed("missing event store identifier".to_owned())
        })
    }

    pub fn sources(&self) -> Result<Vec<EKSource>, EventKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::source::ek_store_sources_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "sources failed") })
        } else {
            unsafe { parse_json_ptr(payload, "EKSource list") }
        }
    }

    pub fn delegate_sources(&self) -> Result<Vec<EKSource>, EventKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::source::ek_store_delegate_sources_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "delegateSources failed") })
        } else {
            unsafe { parse_json_ptr(payload, "EKSource list") }
        }
    }

    pub fn source_with_identifier(
        &self,
        identifier: impl AsRef<str>,
    ) -> Result<Option<EKSource>, EventKitError> {
        let identifier = cstring_from_str(identifier.as_ref(), "EKSource identifier")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::source::ek_store_source_json(self.raw.as_ptr(), identifier.as_ptr(), &mut error)
        };
        unsafe { parse_optional_json_ptr(payload, error, "sourceWithIdentifier") }
    }

    pub fn calendars_for_entity_type(
        &self,
        entity_type: EKEntityType,
    ) -> Result<Vec<EKCalendar>, EventKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::calendar::ek_store_calendars_json(
                self.raw.as_ptr(),
                entity_type.as_raw(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "calendarsForEntityType failed") })
        } else {
            unsafe { parse_json_ptr(payload, "EKCalendar list") }
        }
    }

    pub fn default_calendar_for_new_events(&self) -> Result<Option<EKCalendar>, EventKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::calendar::ek_store_default_event_calendar_json(self.raw.as_ptr(), &mut error)
        };
        unsafe { parse_optional_json_ptr(payload, error, "defaultCalendarForNewEvents") }
    }

    pub fn default_calendar_for_new_reminders(&self) -> Result<Option<EKCalendar>, EventKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::calendar::ek_store_default_reminder_calendar_json(self.raw.as_ptr(), &mut error)
        };
        unsafe { parse_optional_json_ptr(payload, error, "defaultCalendarForNewReminders") }
    }

    pub fn calendar_with_identifier(
        &self,
        identifier: impl AsRef<str>,
    ) -> Result<Option<EKCalendar>, EventKitError> {
        let identifier = cstring_from_str(identifier.as_ref(), "EKCalendar identifier")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::calendar::ek_store_calendar_json(
                self.raw.as_ptr(),
                identifier.as_ptr(),
                &mut error,
            )
        };
        unsafe { parse_optional_json_ptr(payload, error, "calendarWithIdentifier") }
    }

    pub fn save_calendar(
        &self,
        calendar: &EKCalendarDraft,
        commit: bool,
    ) -> Result<EKCalendar, EventKitError> {
        let calendar_json = json_cstring(calendar, "EKCalendarDraft")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::calendar::ek_store_save_calendar_json(
                self.raw.as_ptr(),
                calendar_json.as_ptr(),
                commit,
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "saveCalendar failed") })
        } else {
            unsafe { parse_json_ptr(payload, "EKCalendar") }
        }
    }

    pub fn remove_calendar(
        &self,
        calendar: &EKCalendar,
        commit: bool,
    ) -> Result<(), EventKitError> {
        self.remove_calendar_by_identifier(&calendar.identifier, commit)
    }

    pub fn remove_calendar_by_identifier(
        &self,
        identifier: impl AsRef<str>,
        commit: bool,
    ) -> Result<(), EventKitError> {
        let identifier = cstring_from_str(identifier.as_ref(), "EKCalendar identifier")?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::calendar::ek_store_remove_calendar(
                self.raw.as_ptr(),
                identifier.as_ptr(),
                commit,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "removeCalendar failed") })
        }
    }

    pub fn calendar_item_with_identifier(
        &self,
        identifier: impl AsRef<str>,
    ) -> Result<Option<EKCalendarItem>, EventKitError> {
        let identifier = cstring_from_str(identifier.as_ref(), "EKCalendarItem identifier")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::event_store::ek_store_calendar_item_json(
                self.raw.as_ptr(),
                identifier.as_ptr(),
                &mut error,
            )
        };
        unsafe { parse_optional_json_ptr(payload, error, "calendarItemWithIdentifier") }
    }

    pub fn calendar_items_with_external_identifier(
        &self,
        external_identifier: impl AsRef<str>,
    ) -> Result<Vec<EKCalendarItem>, EventKitError> {
        let external_identifier = cstring_from_str(
            external_identifier.as_ref(),
            "EKCalendarItem external identifier",
        )?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::event_store::ek_store_calendar_items_external_json(
                self.raw.as_ptr(),
                external_identifier.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                EventKitError::from_error_ptr(error, "calendarItemsWithExternalIdentifier failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "EKCalendarItem list") }
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

    pub fn events_matching(
        &self,
        predicate: &EKEventPredicate,
    ) -> Result<Vec<EKEvent>, EventKitError> {
        let predicate_json = json_cstring(predicate, "EKEventPredicate")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::event_store::ek_store_events_matching_json(
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

    pub fn enumerate_events_matching<F>(
        &self,
        predicate: &EKEventPredicate,
        mut callback: F,
    ) -> Result<(), EventKitError>
    where
        F: FnMut(&EKEvent) -> bool,
    {
        for event in self.events_matching(predicate)? {
            if !callback(&event) {
                break;
            }
        }
        Ok(())
    }

    pub fn event_with_identifier(
        &self,
        identifier: impl AsRef<str>,
    ) -> Result<Option<EKEvent>, EventKitError> {
        let identifier = cstring_from_str(identifier.as_ref(), "EKEvent identifier")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::event_store::ek_store_event_json(
                self.raw.as_ptr(),
                identifier.as_ptr(),
                &mut error,
            )
        };
        unsafe { parse_optional_json_ptr(payload, error, "eventWithIdentifier") }
    }

    pub fn refresh_event(
        &self,
        identifier: impl AsRef<str>,
    ) -> Result<Option<EKEvent>, EventKitError> {
        let identifier = cstring_from_str(identifier.as_ref(), "EKEvent identifier")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::event::ek_store_refresh_event_json(
                self.raw.as_ptr(),
                identifier.as_ptr(),
                &mut error,
            )
        };
        unsafe { parse_optional_json_ptr(payload, error, "EKEvent refresh") }
    }

    pub fn predicate_for_reminders(&self, calendars: Option<&[EKCalendar]>) -> EKReminderPredicate {
        let mut predicate = EKReminderPredicate::new();
        predicate.calendar_identifiers = calendar_identifiers(calendars);
        predicate
    }

    pub fn predicate_for_incomplete_reminders(
        &self,
        start_date: Option<impl Into<String>>,
        end_date: Option<impl Into<String>>,
        calendars: Option<&[EKCalendar]>,
    ) -> EKReminderPredicate {
        let mut predicate = EKReminderPredicate::incomplete();
        predicate.calendar_identifiers = calendar_identifiers(calendars);
        predicate.start_date = start_date.map(Into::into);
        predicate.end_date = end_date.map(Into::into);
        predicate
    }

    pub fn predicate_for_completed_reminders(
        &self,
        start_date: Option<impl Into<String>>,
        end_date: Option<impl Into<String>>,
        calendars: Option<&[EKCalendar]>,
    ) -> EKReminderPredicate {
        let mut predicate = EKReminderPredicate::completed();
        predicate.calendar_identifiers = calendar_identifiers(calendars);
        predicate.start_date = start_date.map(Into::into);
        predicate.end_date = end_date.map(Into::into);
        predicate
    }

    pub fn fetch_reminders_matching(
        &self,
        predicate: &EKReminderPredicate,
    ) -> Result<Vec<EKReminder>, EventKitError> {
        let predicate_json = json_cstring(predicate, "EKReminderPredicate")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::event_store::ek_store_fetch_reminders_json(
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
            ffi::event_store::ek_store_save_event(
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
            ffi::event_store::ek_store_remove_event(
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
            ffi::event_store::ek_store_save_reminder(
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
            ffi::event_store::ek_store_remove_reminder(
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
        let status = unsafe { ffi::event_store::ek_store_commit(self.raw.as_ptr(), &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "commit failed") })
        }
    }

    pub fn reset(&self) {
        unsafe { ffi::event_store::ek_store_reset(self.raw.as_ptr()) };
    }

    pub fn refresh_sources_if_necessary(&self) {
        unsafe { ffi::event_store::ek_store_refresh_sources_if_necessary(self.raw.as_ptr()) };
    }
}

impl Drop for EKEventStore {
    fn drop(&mut self) {
        unsafe { ffi::event_store::ek_store_release(self.raw.as_ptr()) };
    }
}
