use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EKParticipantStatus {
    #[default]
    Unknown,
    Pending,
    Accepted,
    Declined,
    Tentative,
    Delegated,
    Completed,
    InProcess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EKParticipantRole {
    #[default]
    Unknown,
    Required,
    Optional,
    Chair,
    NonParticipant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EKParticipantScheduleStatus {
    #[default]
    None,
    Pending,
    Sent,
    Delivered,
    RecipientNotRecognized,
    NoPrivileges,
    DeliveryFailed,
    CannotDeliver,
    RecipientNotAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EKParticipantType {
    #[default]
    Unknown,
    Person,
    Room,
    Resource,
    Group,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EKParticipant {
    pub url: Option<String>,
    pub name: Option<String>,
    pub participant_status: EKParticipantStatus,
    pub participant_role: EKParticipantRole,
    pub participant_type: EKParticipantType,
    pub is_current_user: bool,
    pub contact_predicate: Option<String>,
}
