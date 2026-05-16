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
    clippy::struct_excessive_bools
)]

pub mod error;
mod ffi;
mod private;
pub mod store;
pub mod types;

pub use error::{EKAuthorizationStatus, EventKitError, NSErrorInfo};
pub use store::EKEventStore;
pub use types::{
    EKAlarm, EKAlarmProximity, EKCalendar, EKCalendarType, EKEntityType, EKEvent, EKEventPredicate,
    EKRecurrenceFrequency, EKRecurrenceRule, EKReminder, EKReminderPredicate, EKSpan,
    NSDateComponents,
};

/// Common imports.
pub mod prelude {
    pub use crate::error::{EKAuthorizationStatus, EventKitError, NSErrorInfo};
    pub use crate::store::EKEventStore;
    pub use crate::types::{
        EKAlarm, EKAlarmProximity, EKCalendar, EKCalendarType, EKEntityType, EKEvent,
        EKEventPredicate, EKRecurrenceFrequency, EKRecurrenceRule, EKReminder, EKReminderPredicate,
        EKSpan, NSDateComponents,
    };
}
