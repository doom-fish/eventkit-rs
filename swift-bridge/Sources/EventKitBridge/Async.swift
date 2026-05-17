import EventKit
import Foundation

// ── Callback convention for all access-request thunks ─────────────────────────
//
//   result non-null (0x1) + error null   →  granted = true
//   result null            + error null  →  granted = false (denied, no error)
//   result null            + error cstr  →  error string

// ── requestFullAccessToEvents ─────────────────────────────────────────────────

@_cdecl("ek_store_request_full_access_events_async")
public func ek_store_request_full_access_events_async(
    _ store: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let store else {
        "missing EKEventStore".withCString { cb(nil, $0, ctx) }
        return
    }
    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    Task {
        do {
            let granted: Bool
            if #available(macOS 14.0, *) {
                granted = try await eventStore.requestFullAccessToEvents()
            } else {
                granted = await withCheckedContinuation { cont in
                    eventStore.requestAccess(to: .event) { g, _ in cont.resume(returning: g) }
                }
            }
            cb(granted ? UnsafeMutableRawPointer(bitPattern: 1) : nil, nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}

// ── requestFullAccessToReminders ──────────────────────────────────────────────

@_cdecl("ek_store_request_full_access_reminders_async")
public func ek_store_request_full_access_reminders_async(
    _ store: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let store else {
        "missing EKEventStore".withCString { cb(nil, $0, ctx) }
        return
    }
    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    Task {
        do {
            let granted: Bool
            if #available(macOS 14.0, *) {
                granted = try await eventStore.requestFullAccessToReminders()
            } else {
                granted = await withCheckedContinuation { cont in
                    eventStore.requestAccess(to: .reminder) { g, _ in cont.resume(returning: g) }
                }
            }
            cb(granted ? UnsafeMutableRawPointer(bitPattern: 1) : nil, nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}

// ── requestWriteOnlyAccessToEvents ────────────────────────────────────────────

@_cdecl("ek_store_request_write_only_access_events_async")
public func ek_store_request_write_only_access_events_async(
    _ store: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let store else {
        "missing EKEventStore".withCString { cb(nil, $0, ctx) }
        return
    }
    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    Task {
        do {
            let granted: Bool
            if #available(macOS 14.0, *) {
                granted = try await eventStore.requestWriteOnlyAccessToEvents()
            } else {
                granted = await withCheckedContinuation { cont in
                    eventStore.requestAccess(to: .event) { g, _ in cont.resume(returning: g) }
                }
            }
            cb(granted ? UnsafeMutableRawPointer(bitPattern: 1) : nil, nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}

// ── fetchReminders(matching:completion:) ──────────────────────────────────────
//
// Callback convention:
//   result = strdup'd JSON C string (caller must free via ek_string_free)
//   error  = null on success

@_cdecl("ek_store_fetch_reminders_async")
public func ek_store_fetch_reminders_async(
    _ store: UnsafeMutableRawPointer?,
    _ predicateJSON: UnsafePointer<CChar>?,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let store else {
        "missing EKEventStore".withCString { cb(nil, $0, ctx) }
        return
    }

    let payload: EKRReminderPredicatePayload
    do {
        payload = try ekrDecodeJSON(predicateJSON, as: EKRReminderPredicatePayload.self)
    } catch {
        error.localizedDescription.withCString { cb(nil, $0, ctx) }
        return
    }

    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    let calendars = ekrResolveCalendars(store: eventStore, identifiers: payload.calendarIdentifiers)

    let predicate: NSPredicate
    do {
        switch payload.kind {
        case .all:
            predicate = eventStore.predicateForReminders(in: calendars)
        case .incomplete:
            predicate = eventStore.predicateForIncompleteReminders(
                withDueDateStarting: try payload.startDate.map(ekrDate(from:)),
                ending: try payload.endDate.map(ekrDate(from:)),
                calendars: calendars
            )
        case .completed:
            predicate = eventStore.predicateForCompletedReminders(
                withCompletionDateStarting: try payload.startDate.map(ekrDate(from:)),
                ending: try payload.endDate.map(ekrDate(from:)),
                calendars: calendars
            )
        }
    } catch {
        error.localizedDescription.withCString { cb(nil, $0, ctx) }
        return
    }

    eventStore.fetchReminders(matching: predicate) { reminders in
        do {
            let json = try ekrEncodeJSON((reminders ?? []).map(ekrEncodeReminder))
            guard let cstr = ekrCString(json) else {
                "failed to allocate fetchReminders JSON result".withCString { cb(nil, $0, ctx) }
                return
            }
            cb(UnsafeRawPointer(cstr), nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}
