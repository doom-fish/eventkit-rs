//! EventKit participant snapshots.

use serde::{Deserialize, Serialize};

use crate::private::redacted;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit participant status.
pub enum EKParticipantStatus {
    #[default]
    /// Matches the EventKit `unknown` case.
    Unknown,
    /// Matches the EventKit `pending` case.
    Pending,
    /// Matches the EventKit `accepted` case.
    Accepted,
    /// Matches the EventKit `declined` case.
    Declined,
    /// Matches the EventKit `tentative` case.
    Tentative,
    /// Matches the EventKit `delegated` case.
    Delegated,
    /// Matches the EventKit `completed` case.
    Completed,
    /// Matches the EventKit `inProcess` case.
    InProcess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit participant role.
pub enum EKParticipantRole {
    #[default]
    /// Matches the EventKit `unknown` case.
    Unknown,
    /// Matches the EventKit `required` case.
    Required,
    /// Matches the EventKit `optional` case.
    Optional,
    /// Matches the EventKit `chair` case.
    Chair,
    /// Matches the EventKit `nonParticipant` case.
    NonParticipant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit participant schedule-delivery status.
pub enum EKParticipantScheduleStatus {
    #[default]
    /// Matches the EventKit `none` case.
    None,
    /// Matches the EventKit `pending` case.
    Pending,
    /// Matches the EventKit `sent` case.
    Sent,
    /// Matches the EventKit `delivered` case.
    Delivered,
    /// Matches the EventKit `recipientNotRecognized` case.
    RecipientNotRecognized,
    /// Matches the EventKit `noPrivileges` case.
    NoPrivileges,
    /// Matches the EventKit `deliveryFailed` case.
    DeliveryFailed,
    /// Matches the EventKit `cannotDeliver` case.
    CannotDeliver,
    /// Matches the EventKit `recipientNotAllowed` case.
    RecipientNotAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents the EventKit participant type.
pub enum EKParticipantType {
    #[default]
    /// Matches the EventKit `unknown` case.
    Unknown,
    /// Matches the EventKit `person` case.
    Person,
    /// Matches the EventKit `room` case.
    Room,
    /// Matches the EventKit `resource` case.
    Resource,
    /// Matches the EventKit `group` case.
    Group,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Represents EventKit participant data.
pub struct EKParticipant {
    /// Mirrors the EventKit `url` property.
    pub url: Option<String>,
    /// Mirrors the EventKit `name` property.
    pub name: Option<String>,
    /// Mirrors the EventKit `participantStatus` property.
    pub participant_status: EKParticipantStatus,
    /// Mirrors the EventKit `participantRole` property.
    pub participant_role: EKParticipantRole,
    /// Mirrors the EventKit `participantType` property.
    pub participant_type: EKParticipantType,
    /// Mirrors the EventKit `isCurrentUser` property.
    pub is_current_user: bool,
    /// Mirrors the EventKit `contactPredicate` property.
    pub contact_predicate: Option<String>,
}

impl core::fmt::Debug for EKParticipant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EKParticipant")
            .field("url", &redacted(self.url.as_ref()))
            .field("name", &redacted(self.name.as_ref()))
            .field("participant_status", &self.participant_status)
            .field("participant_role", &self.participant_role)
            .field("participant_type", &self.participant_type)
            .field("is_current_user", &self.is_current_user)
            .field(
                "contact_predicate",
                &redacted(self.contact_predicate.as_ref()),
            )
            .finish()
    }
}
