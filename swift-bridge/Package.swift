// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "SpriteKitBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "SpriteKitBridge",
            type: .static,
            targets: ["SpriteKitBridge"])
    ],
    targets: [
        .target(
            name: "SpriteKitBridge",
            path: "Sources/SpriteKitBridge",
            publicHeadersPath: "include")
    ]
)
