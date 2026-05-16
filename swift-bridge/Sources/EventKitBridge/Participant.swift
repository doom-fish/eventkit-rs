import EventKit
import Foundation

enum EKRParticipantStatus: String, Codable {
    case unknown
    case pending
    case accepted
    case declined
    case tentative
    case delegated
    case completed
    case inProcess
}

enum EKRParticipantRole: String, Codable {
    case unknown
    case required
    case optional
    case chair
    case nonParticipant
}

enum EKRParticipantScheduleStatus: String, Codable {
    case none
    case pending
    case sent
    case delivered
    case recipientNotRecognized
    case noPrivileges
    case deliveryFailed
    case cannotDeliver
    case recipientNotAllowed
}

enum EKRParticipantType: String, Codable {
    case unknown
    case person
    case room
    case resource
    case group
}

struct EKRParticipantPayload: Codable {
    var url: String?
    var name: String?
    var participantStatus: EKRParticipantStatus
    var participantRole: EKRParticipantRole
    var participantType: EKRParticipantType
    var isCurrentUser: Bool
    var contactPredicate: String?
}

func ekrParticipantStatusPayload(from status: EKParticipantStatus) -> EKRParticipantStatus {
    switch status {
    case .unknown:
        return .unknown
    case .pending:
        return .pending
    case .accepted:
        return .accepted
    case .declined:
        return .declined
    case .tentative:
        return .tentative
    case .delegated:
        return .delegated
    case .completed:
        return .completed
    case .inProcess:
        return .inProcess
    @unknown default:
        return .unknown
    }
}

func ekrParticipantRolePayload(from role: EKParticipantRole) -> EKRParticipantRole {
    switch role {
    case .unknown:
        return .unknown
    case .required:
        return .required
    case .optional:
        return .optional
    case .chair:
        return .chair
    case .nonParticipant:
        return .nonParticipant
    @unknown default:
        return .unknown
    }
}

func ekrParticipantTypePayload(from type: EKParticipantType) -> EKRParticipantType {
    switch type {
    case .unknown:
        return .unknown
    case .person:
        return .person
    case .room:
        return .room
    case .resource:
        return .resource
    case .group:
        return .group
    @unknown default:
        return .unknown
    }
}

func ekrEncodeParticipant(_ participant: EKParticipant) -> EKRParticipantPayload {
    EKRParticipantPayload(
        url: participant.url.absoluteString,
        name: participant.name,
        participantStatus: ekrParticipantStatusPayload(from: participant.participantStatus),
        participantRole: ekrParticipantRolePayload(from: participant.participantRole),
        participantType: ekrParticipantTypePayload(from: participant.participantType),
        isCurrentUser: participant.isCurrentUser,
        contactPredicate: participant.contactPredicate.predicateFormat
    )
}
