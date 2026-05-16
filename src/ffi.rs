#![allow(missing_docs)]

pub mod alarm;
pub mod calendar;
pub mod core;
pub mod event;
pub mod event_store;
pub mod object;
pub mod participant;
pub mod recurrence_rule;
pub mod reminder;
pub mod source;
pub mod structured_location;
pub mod virtual_conference_provider;

pub use core::ek_string_free;

pub mod status {
    pub const OK: i32 = 0;
}
