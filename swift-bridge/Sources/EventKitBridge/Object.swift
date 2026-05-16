import EventKit
import Foundation

@_cdecl("ek_object_from_event_json")
public func ek_object_from_event_json(
    _ store: UnsafeMutableRawPointer?,
    _ eventJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let payload = try ekrDecodeJSON(eventJSON, as: EKREventPayload.self)
        let event = try ekrPrepareEvent(
            store: ekrBorrow(store, as: EKEventStore.self),
            payload: payload,
            requireCalendar: false
        )
        return ekrRetain(event as EKObject)
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_object_from_reminder_json")
public func ek_object_from_reminder_json(
    _ store: UnsafeMutableRawPointer?,
    _ reminderJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let payload = try ekrDecodeJSON(reminderJSON, as: EKRReminderPayload.self)
        let reminder = try ekrPrepareReminder(
            store: ekrBorrow(store, as: EKEventStore.self),
            payload: payload,
            requireCalendar: false
        )
        return ekrRetain(reminder as EKObject)
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_object_from_calendar_draft_json")
public func ek_object_from_calendar_draft_json(
    _ store: UnsafeMutableRawPointer?,
    _ calendarJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let payload = try ekrDecodeJSON(calendarJSON, as: EKRCalendarDraftPayload.self)
        let calendar = try ekrPrepareCalendar(
            store: ekrBorrow(store, as: EKEventStore.self),
            payload: payload,
            requireSource: false
        )
        return ekrRetain(calendar as EKObject)
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_object_release")
public func ek_object_release(_ object: UnsafeMutableRawPointer?) {
    guard let object else { return }
    ekrRelease(object)
}

@_cdecl("ek_object_has_changes")
public func ek_object_has_changes(_ object: UnsafeMutableRawPointer?) -> Bool {
    guard let object else { return false }
    return ekrBorrow(object, as: EKObject.self).hasChanges
}

@_cdecl("ek_object_is_new")
public func ek_object_is_new(_ object: UnsafeMutableRawPointer?) -> Bool {
    guard let object else { return false }
    return ekrBorrow(object, as: EKObject.self).isNew
}

@_cdecl("ek_object_reset")
public func ek_object_reset(_ object: UnsafeMutableRawPointer?) {
    guard let object else { return }
    ekrBorrow(object, as: EKObject.self).reset()
}

@_cdecl("ek_object_rollback")
public func ek_object_rollback(_ object: UnsafeMutableRawPointer?) {
    guard let object else { return }
    ekrBorrow(object, as: EKObject.self).rollback()
}

@_cdecl("ek_object_refresh")
public func ek_object_refresh(_ object: UnsafeMutableRawPointer?) -> Bool {
    guard let object else { return false }
    return ekrBorrow(object, as: EKObject.self).refresh()
}
