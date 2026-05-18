//! Re-exports common EventKit data types.

/// Re-exports EventKit alarm types.
pub use crate::alarm::{EKAlarm, EKAlarmProximity, EKAlarmType};
/// Re-exports EventKit calendar types.
pub use crate::calendar::{
    EKCalendar, EKCalendarDraft, EKCalendarEventAvailability, EKCalendarType,
};
/// Re-exports EventKit event types.
pub use crate::event::{EKEvent, EKEventAvailability, EKEventStatus};
/// Re-exports EventKit store types.
pub use crate::event_store::{
    EKCalendarItem, EKCalendarItemKind, EKEntityType, EKEventPredicate, EKReminderPredicate,
    EKReminderPredicateKind, EKSpan,
};
/// Re-exports EventKit participant types.
pub use crate::participant::{
    EKParticipant, EKParticipantRole, EKParticipantStatus, EKParticipantType,
};
/// Re-exports EventKit recurrence types.
pub use crate::recurrence_rule::{
    EKRecurrenceDayOfWeek, EKRecurrenceEnd, EKRecurrenceFrequency, EKRecurrenceRule, EKWeekday,
};
/// Re-exports EventKit reminder types.
pub use crate::reminder::{EKReminder, EKReminderPriority, NSDateComponents};
/// Re-exports EventKit source types.
pub use crate::source::{EKSource, EKSourceType};
/// Re-exports EventKit structured-location types.
pub use crate::structured_location::{EKGeoLocation, EKStructuredLocation};
/// Re-exports EventKit virtual conference types.
pub use crate::virtual_conference_provider::{
    EKVirtualConferenceDescriptor, EKVirtualConferenceRoomTypeDescriptor,
    EKVirtualConferenceURLDescriptor, EK_VIRTUAL_CONFERENCE_PROVIDER_IS_EXTENSION_ONLY,
};
