use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EKCalendarType {
    Local,
    CalDav,
    Exchange,
    Subscription,
    Birthday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EKRecurrenceFrequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EKAlarmProximity {
    None,
    Enter,
    Leave,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NSDateComponents {
    pub era: Option<i32>,
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
    pub hour: Option<i32>,
    pub minute: Option<i32>,
    pub second: Option<i32>,
    pub is_leap_month: Option<bool>,
    pub time_zone_identifier: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EKAlarm {
    pub absolute_date: Option<String>,
    pub relative_offset: Option<f64>,
    pub proximity: Option<EKAlarmProximity>,
    pub email_address: Option<String>,
    pub sound_name: Option<String>,
}

impl EKAlarm {
    pub fn relative(relative_offset: f64) -> Self {
        Self {
            relative_offset: Some(relative_offset),
            ..Self::default()
        }
    }

    pub fn absolute(absolute_date: impl Into<String>) -> Self {
        Self {
            absolute_date: Some(absolute_date.into()),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EKRecurrenceRule {
    pub frequency: EKRecurrenceFrequency,
    pub interval: i64,
    pub end_date: Option<String>,
    pub occurrence_count: Option<u64>,
}

impl EKRecurrenceRule {
    pub fn new(frequency: EKRecurrenceFrequency) -> Self {
        Self {
            frequency,
            interval: 1,
            end_date: None,
            occurrence_count: None,
        }
    }

    pub fn with_interval(mut self, interval: i64) -> Self {
        self.interval = interval;
        self
    }

    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    pub fn with_occurrence_count(mut self, occurrence_count: u64) -> Self {
        self.occurrence_count = Some(occurrence_count);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EKCalendar {
    pub identifier: String,
    pub title: String,
    pub calendar_type: EKCalendarType,
    #[serde(default)]
    pub allowed_entity_types: Vec<EKEntityType>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EKEvent {
    pub identifier: Option<String>,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub calendar_identifier: Option<String>,
    pub calendar: Option<EKCalendar>,
    pub notes: Option<String>,
    pub location: Option<String>,
    #[serde(default)]
    pub alarms: Vec<EKAlarm>,
    #[serde(default)]
    pub recurrence_rules: Vec<EKRecurrenceRule>,
}

impl EKEvent {
    pub fn new(
        title: impl Into<String>,
        start_date: impl Into<String>,
        end_date: impl Into<String>,
    ) -> Self {
        Self {
            identifier: None,
            title: title.into(),
            start_date: start_date.into(),
            end_date: end_date.into(),
            calendar_identifier: None,
            calendar: None,
            notes: None,
            location: None,
            alarms: Vec::new(),
            recurrence_rules: Vec::new(),
        }
    }

    pub fn with_calendar_identifier(mut self, calendar_identifier: impl Into<String>) -> Self {
        self.calendar_identifier = Some(calendar_identifier.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EKReminder {
    pub identifier: Option<String>,
    pub title: String,
    pub calendar_identifier: Option<String>,
    pub calendar: Option<EKCalendar>,
    pub due_date_components: Option<NSDateComponents>,
    pub is_completed: bool,
    pub priority: u64,
    pub notes: Option<String>,
    #[serde(default)]
    pub alarms: Vec<EKAlarm>,
    #[serde(default)]
    pub recurrence_rules: Vec<EKRecurrenceRule>,
}

impl EKReminder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            identifier: None,
            title: title.into(),
            calendar_identifier: None,
            calendar: None,
            due_date_components: None,
            is_completed: false,
            priority: 0,
            notes: None,
            alarms: Vec::new(),
            recurrence_rules: Vec::new(),
        }
    }

    pub fn with_calendar_identifier(mut self, calendar_identifier: impl Into<String>) -> Self {
        self.calendar_identifier = Some(calendar_identifier.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EKReminderPredicate {
    pub calendar_identifiers: Option<Vec<String>>,
}

impl EKReminderPredicate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_calendar_identifiers(
        mut self,
        calendar_identifiers: impl IntoIterator<Item = String>,
    ) -> Self {
        self.calendar_identifiers = Some(calendar_identifiers.into_iter().collect());
        self
    }
}
