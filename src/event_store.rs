//! EventKit store access, predicates, and save helpers.

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

/// Names the EventKit store-changed notification.
pub const EK_EVENT_STORE_CHANGED_NOTIFICATION: &str = "EKEventStoreChangedNotification";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit entity types supported by `EKEventStore`.
pub enum EKEntityType {
    /// Matches the EventKit `event` case.
    Event,
    /// Matches the EventKit `reminder` case.
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
/// Represents the EventKit span used when saving or removing recurring items.
pub enum EKSpan {
    #[default]
    /// Matches the EventKit `thisEvent` case.
    ThisEvent,
    /// Matches the EventKit `futureEvents` case.
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
/// Represents the EventKit event-search predicate payload.
pub struct EKEventPredicate {
    /// Mirrors the EventKit `startDate` property.
    pub start_date: String,
    /// Mirrors the EventKit `endDate` property.
    pub end_date: String,
    /// Mirrors the EventKit `calendarIdentifiers` property.
    pub calendar_identifiers: Option<Vec<String>>,
}

impl EKEventPredicate {
    /// Creates a new EventKit `EKEventPredicate` value.
    pub fn new(start_date: impl Into<String>, end_date: impl Into<String>) -> Self {
        Self {
            start_date: start_date.into(),
            end_date: end_date.into(),
            calendar_identifiers: None,
        }
    }

    /// Sets the EventKit `calendarIdentifiers` property on this `EKEventPredicate` value.
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
/// Represents the EventKit reminder predicate kind.
pub enum EKReminderPredicateKind {
    #[default]
    /// Matches the EventKit `all` case.
    All,
    /// Matches the EventKit `incomplete` case.
    Incomplete,
    /// Matches the EventKit `completed` case.
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit reminder-search predicate payload.
pub struct EKReminderPredicate {
    /// Mirrors the EventKit `calendarIdentifiers` property.
    pub calendar_identifiers: Option<Vec<String>>,
    /// Mirrors the EventKit `kind` property.
    pub kind: EKReminderPredicateKind,
    /// Mirrors the EventKit `startDate` property.
    pub start_date: Option<String>,
    /// Mirrors the EventKit `endDate` property.
    pub end_date: Option<String>,
}

impl EKReminderPredicate {
    /// Creates a new EventKit `EKReminderPredicate` value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an EventKit reminder predicate for incomplete reminders.
    pub fn incomplete() -> Self {
        Self {
            kind: EKReminderPredicateKind::Incomplete,
            ..Self::default()
        }
    }

    /// Creates an EventKit reminder predicate for completed reminders.
    pub fn completed() -> Self {
        Self {
            kind: EKReminderPredicateKind::Completed,
            ..Self::default()
        }
    }

    /// Sets the EventKit `calendarIdentifiers` property on this `EKReminderPredicate` value.
    pub fn with_calendar_identifiers(
        mut self,
        calendar_identifiers: impl IntoIterator<Item = String>,
    ) -> Self {
        self.calendar_identifiers = Some(calendar_identifiers.into_iter().collect());
        self
    }

    /// Sets the EventKit `startDate` property on this `EKReminderPredicate` value.
    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    /// Sets the EventKit `endDate` property on this `EKReminderPredicate` value.
    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit calendar-item kind.
pub enum EKCalendarItemKind {
    /// Matches the EventKit `event` case.
    Event,
    /// Matches the EventKit `reminder` case.
    Reminder,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Represents either an EventKit event or reminder result.
pub struct EKCalendarItem {
    /// Mirrors the EventKit `kind` property.
    pub kind: EKCalendarItemKind,
    /// Mirrors the EventKit `event` property.
    pub event: Option<EKEvent>,
    /// Mirrors the EventKit `reminder` property.
    pub reminder: Option<EKReminder>,
}

impl EKCalendarItem {
    /// Returns the wrapped EventKit event when present.
    pub fn as_event(&self) -> Option<&EKEvent> {
        self.event.as_ref()
    }

    /// Returns the wrapped EventKit reminder when present.
    pub fn as_reminder(&self) -> Option<&EKReminder> {
        self.reminder.as_ref()
    }
}

#[derive(Debug)]
/// Wraps an EventKit `EKEventStore` instance.
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
    /// Creates a new EventKit event store.
    pub fn new() -> Result<Self, EventKitError> {
        let raw = NonNull::new(unsafe { ffi::event_store::ek_store_new() }).ok_or_else(|| {
            EventKitError::OperationFailed("failed to create EKEventStore".to_owned())
        })?;
        Ok(Self { raw })
    }

    /// Creates an EventKit event store limited to the given source identifiers.
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

    /// Returns the current EventKit authorization status for the given entity type.
    pub fn authorization_status(entity_type: EKEntityType) -> EKAuthorizationStatus {
        EKAuthorizationStatus::from_raw(unsafe {
            ffi::event_store::ek_authorization_status(entity_type.as_raw())
        })
    }

    /// Requests calendar event access from EventKit using the legacy API.
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

    /// Requests full EventKit access to calendars and events.
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

    /// Requests write-only EventKit access to calendars and events.
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

    /// Requests reminder access from EventKit using the legacy API.
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

    /// Requests full EventKit access to reminders.
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

    /// Returns the EventKit identifier for this store instance.
    pub fn event_store_identifier(&self) -> Result<String, EventKitError> {
        let payload = unsafe { ffi::event_store::ek_store_identifier(self.raw.as_ptr()) };
        unsafe { take_string(payload) }.ok_or_else(|| {
            EventKitError::OperationFailed("missing event store identifier".to_owned())
        })
    }

    /// Returns the EventKit sources visible to this store.
    pub fn sources(&self) -> Result<Vec<EKSource>, EventKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::source::ek_store_sources_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { EventKitError::from_error_ptr(error, "sources failed") })
        } else {
            unsafe { parse_json_ptr(payload, "EKSource list") }
        }
    }

    /// Returns the delegate EventKit sources visible to this store.
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

    /// Looks up an EventKit source by identifier.
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

    /// Returns the EventKit calendars for the given entity type.
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

    /// Returns the default EventKit calendar for new events.
    pub fn default_calendar_for_new_events(&self) -> Result<Option<EKCalendar>, EventKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::calendar::ek_store_default_event_calendar_json(self.raw.as_ptr(), &mut error)
        };
        unsafe { parse_optional_json_ptr(payload, error, "defaultCalendarForNewEvents") }
    }

    /// Returns the default EventKit calendar for new reminders.
    pub fn default_calendar_for_new_reminders(&self) -> Result<Option<EKCalendar>, EventKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::calendar::ek_store_default_reminder_calendar_json(self.raw.as_ptr(), &mut error)
        };
        unsafe { parse_optional_json_ptr(payload, error, "defaultCalendarForNewReminders") }
    }

    /// Looks up an EventKit calendar by identifier.
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

    /// Saves an EventKit calendar draft through this store.
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

    /// Removes the given EventKit calendar from this store.
    pub fn remove_calendar(
        &self,
        calendar: &EKCalendar,
        commit: bool,
    ) -> Result<(), EventKitError> {
        self.remove_calendar_by_identifier(&calendar.identifier, commit)
    }

    /// Removes an EventKit calendar by identifier.
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

    /// Looks up an EventKit calendar item by identifier.
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

    /// Looks up EventKit calendar items by external identifier.
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

    /// Builds an EventKit event predicate for the given date range.
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

    /// Returns the EventKit events that match the given predicate.
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

    /// Enumerates the EventKit events that match the given predicate.
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

    /// Looks up an EventKit event by identifier.
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

    /// Refreshes an EventKit event by identifier.
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

    /// Builds an EventKit reminder predicate for the given calendars.
    pub fn predicate_for_reminders(&self, calendars: Option<&[EKCalendar]>) -> EKReminderPredicate {
        let mut predicate = EKReminderPredicate::new();
        predicate.calendar_identifiers = calendar_identifiers(calendars);
        predicate
    }

    /// Builds an EventKit predicate for incomplete reminders.
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

    /// Builds an EventKit predicate for completed reminders.
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

    /// Returns the EventKit reminders that match the given predicate.
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

    /// Saves an EventKit event through this store.
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

    /// Removes an EventKit event through this store.
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

    /// Saves an EventKit reminder through this store.
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

    /// Removes an EventKit reminder through this store.
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

    /// Commits pending EventKit changes in this store.
    pub fn commit(&self) -> Result<(), EventKitError> {
        let mut error = ptr::null_mut();
        let status = unsafe { ffi::event_store::ek_store_commit(self.raw.as_ptr(), &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { EventKitError::from_error_ptr(error, "commit failed") })
        }
    }

    /// Resets pending EventKit changes in this store.
    pub fn reset(&self) {
        unsafe { ffi::event_store::ek_store_reset(self.raw.as_ptr()) };
    }

    /// Refreshes EventKit sources when the framework indicates they changed.
    pub fn refresh_sources_if_necessary(&self) {
        unsafe { ffi::event_store::ek_store_refresh_sources_if_necessary(self.raw.as_ptr()) };
    }
}

impl Drop for EKEventStore {
    fn drop(&mut self) {
        unsafe { ffi::event_store::ek_store_release(self.raw.as_ptr()) };
    }
}
