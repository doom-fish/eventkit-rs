// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "EventKitBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "EventKitBridge",
            type: .static,
            targets: ["EventKitBridge"])
    ],
    targets: [
        .target(
            name: "EventKitBridge",
            path: "Sources/EventKitBridge",
            publicHeadersPath: "include")
    ]
)
