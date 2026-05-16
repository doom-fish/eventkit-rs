import Foundation

public let EKR_OK: Int32 = 0
public let EKR_ERROR: Int32 = -1

@_cdecl("ek_string_free")
public func ek_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    guard let string else { return }
    free(string)
}

@inline(__always)
public func ekrCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
public func ekrRetain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func ekrBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
public func ekrRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

public struct EKRErrorPayload: Codable {
    public var domain: String
    public var code: Int
    public var message: String
}

private let ekrFractionalDateFormatter: ISO8601DateFormatter = {
    let formatter = ISO8601DateFormatter()
    formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
    return formatter
}()

private let ekrPlainDateFormatter = ISO8601DateFormatter()

public func ekrDateString(_ date: Date?) -> String? {
    guard let date else { return nil }
    return ekrFractionalDateFormatter.string(from: date)
}

public func ekrDate(from value: String) throws -> Date {
    if let date = ekrFractionalDateFormatter.date(from: value) ?? ekrPlainDateFormatter.date(from: value) {
        return date
    }

    throw NSError(
        domain: "eventkit-rs",
        code: -1,
        userInfo: [NSLocalizedDescriptionKey: "invalid ISO8601 date: \(value)"]
    )
}

public func ekrEncodeJSON<T: Encodable>(_ value: T) throws -> String {
    let encoder = JSONEncoder()
    let data = try encoder.encode(value)
    guard let string = String(data: data, encoding: .utf8) else {
        throw NSError(
            domain: "eventkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "failed to encode JSON as UTF-8"]
        )
    }
    return string
}

public func ekrDecodeJSON<T: Decodable>(_ json: UnsafePointer<CChar>?, as type: T.Type) throws -> T {
    guard let json else {
        throw NSError(
            domain: "eventkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "missing JSON payload"]
        )
    }

    let data = Data(String(cString: json).utf8)
    return try JSONDecoder().decode(T.self, from: data)
}

public func ekrErrorPayload(from error: Error) -> EKRErrorPayload {
    let nsError = error as NSError
    return EKRErrorPayload(
        domain: nsError.domain,
        code: nsError.code,
        message: nsError.localizedDescription
    )
}

public func ekrSetError(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ error: Error
) {
    guard let outError else { return }

    if let json = try? ekrEncodeJSON(ekrErrorPayload(from: error)) {
        outError.pointee = ekrCString(json)
    } else {
        outError.pointee = ekrCString((error as NSError).localizedDescription)
    }
}

public func ekrSetMessageError(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    message: String,
    domain: String = "eventkit-rs",
    code: Int = -1
) {
    guard let outError else { return }

    let payload = EKRErrorPayload(domain: domain, code: code, message: message)
    if let json = try? ekrEncodeJSON(payload) {
        outError.pointee = ekrCString(json)
    } else {
        outError.pointee = ekrCString(message)
    }
}
