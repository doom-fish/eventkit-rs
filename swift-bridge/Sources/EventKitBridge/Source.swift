import EventKit
import Foundation

enum EKRSourceType: String, Codable {
    case local
    case exchange
    case calDav
    case mobileMe
    case subscribed
    case birthdays
}

struct EKRSourcePayload: Codable {
    var identifier: String
    var sourceType: EKRSourceType
    var title: String
    var isDelegate: Bool
}

func ekrSourceTypePayload(from sourceType: EKSourceType) -> EKRSourceType {
    switch sourceType {
    case .local:
        return .local
    case .exchange:
        return .exchange
    case .calDAV:
        return .calDav
    case .mobileMe:
        return .mobileMe
    case .subscribed:
        return .subscribed
    case .birthdays:
        return .birthdays
    @unknown default:
        return .local
    }
}

func ekrEncodeSource(_ source: EKSource) -> EKRSourcePayload {
    EKRSourcePayload(
        identifier: source.sourceIdentifier,
        sourceType: ekrSourceTypePayload(from: source.sourceType),
        title: source.title,
        isDelegate: source.isDelegate
    )
}

@_cdecl("ek_store_sources_json")
public func ek_store_sources_json(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let sources = ekrBorrow(store, as: EKEventStore.self).sources.map(ekrEncodeSource)
        return ekrCString(try ekrEncodeJSON(sources))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_delegate_sources_json")
public func ek_store_delegate_sources_json(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let sources = ekrBorrow(store, as: EKEventStore.self).delegateSources.map(ekrEncodeSource)
        return ekrCString(try ekrEncodeJSON(sources))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_source_json")
public func ek_store_source_json(
    _ store: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }
    guard let identifier else {
        ekrSetMessageError(outError, message: "missing EKSource identifier")
        return nil
    }

    let source = ekrBorrow(store, as: EKEventStore.self).source(withIdentifier: String(cString: identifier))
    guard let source else { return nil }

    do {
        return ekrCString(try ekrEncodeJSON(ekrEncodeSource(source)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_source_calendars_json")
public func ek_store_source_calendars_json(
    _ store: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ entityTypeRaw: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }
    guard let identifier else {
        ekrSetMessageError(outError, message: "missing EKSource identifier")
        return nil
    }

    do {
        let eventStore = ekrBorrow(store, as: EKEventStore.self)
        guard let source = eventStore.source(withIdentifier: String(cString: identifier)) else {
            return nil
        }
        let calendars = Array(source.calendars(for: try ekrEntityType(from: entityTypeRaw))).map(ekrEncodeCalendar)
        return ekrCString(try ekrEncodeJSON(calendars))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}
