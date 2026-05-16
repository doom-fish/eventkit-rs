#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cargo_common_metadata,
    clippy::doc_markdown,
    clippy::manual_map,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::option_if_let_else,
    clippy::return_self_not_must_use,
    clippy::single_option_map,
    clippy::struct_excessive_bools,
    clippy::unsafe_derive_deserialize
)]

pub mod alarm;
pub mod calendar;
pub mod error;
pub mod event;
pub mod event_store;
mod ffi;
pub mod object;
pub mod participant;
mod private;
pub mod recurrence_rule;
pub mod reminder;
pub mod source;
pub mod store;
pub mod structured_location;
pub mod types;
pub mod virtual_conference_provider;

pub use alarm::{EKAlarm, EKAlarmProximity, EKAlarmType};
pub use calendar::{EKCalendar, EKCalendarDraft, EKCalendarEventAvailability, EKCalendarType};
pub use error::{EKAuthorizationStatus, EventKitError, NSErrorInfo};
pub use event::{EKEvent, EKEventAvailability, EKEventStatus};
pub use event_store::{
    EKCalendarItem, EKCalendarItemKind, EKEntityType, EKEventPredicate, EKEventStore,
    EKReminderPredicate, EKReminderPredicateKind, EKSpan, EK_EVENT_STORE_CHANGED_NOTIFICATION,
};
pub use object::EKObject;
pub use participant::{
    EKParticipant, EKParticipantRole, EKParticipantScheduleStatus, EKParticipantStatus,
    EKParticipantType,
};
pub use recurrence_rule::{
    EKRecurrenceDayOfWeek, EKRecurrenceEnd, EKRecurrenceFrequency, EKRecurrenceRule, EKWeekday,
};
pub use reminder::{EKReminder, EKReminderPriority, NSDateComponents};
pub use source::{EKSource, EKSourceType};
pub use structured_location::{EKGeoLocation, EKStructuredLocation};
pub use virtual_conference_provider::{
    EKVirtualConferenceDescriptor, EKVirtualConferenceRoomTypeDescriptor,
    EKVirtualConferenceURLDescriptor, EK_VIRTUAL_CONFERENCE_PROVIDER_IS_EXTENSION_ONLY,
};

/// Common imports.
pub mod prelude {
    pub use crate::alarm::{EKAlarm, EKAlarmProximity, EKAlarmType};
    pub use crate::calendar::{
        EKCalendar, EKCalendarDraft, EKCalendarEventAvailability, EKCalendarType,
    };
    pub use crate::error::{EKAuthorizationStatus, EventKitError, NSErrorInfo};
    pub use crate::event::{EKEvent, EKEventAvailability, EKEventStatus};
    pub use crate::event_store::{
        EKCalendarItem, EKCalendarItemKind, EKEntityType, EKEventPredicate, EKEventStore,
        EKReminderPredicate, EKReminderPredicateKind, EKSpan, EK_EVENT_STORE_CHANGED_NOTIFICATION,
    };
    pub use crate::object::EKObject;
    pub use crate::participant::{
        EKParticipant, EKParticipantRole, EKParticipantScheduleStatus, EKParticipantStatus,
        EKParticipantType,
    };
    pub use crate::recurrence_rule::{
        EKRecurrenceDayOfWeek, EKRecurrenceEnd, EKRecurrenceFrequency, EKRecurrenceRule, EKWeekday,
    };
    pub use crate::reminder::{EKReminder, EKReminderPriority, NSDateComponents};
    pub use crate::source::{EKSource, EKSourceType};
    pub use crate::structured_location::{EKGeoLocation, EKStructuredLocation};
    pub use crate::virtual_conference_provider::{
        EKVirtualConferenceDescriptor, EKVirtualConferenceRoomTypeDescriptor,
        EKVirtualConferenceURLDescriptor, EK_VIRTUAL_CONFERENCE_PROVIDER_IS_EXTENSION_ONLY,
    };
}
