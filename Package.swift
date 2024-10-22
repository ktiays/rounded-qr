// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "RoundedQRWrapper",
    platforms: [
        .macOS(.v11),
        .iOS(.v13),
        .watchOS(.v6),
        .tvOS(.v13),
    ],
    products: [
        .library(name: "RoundedQRWrapper", targets: ["RoundedQRWrapper"])
    ],
    targets: [
        .target(
            name: "RoundedQRWrapper",
            dependencies: [
                .target(name: "RoundedQR")
            ]
        ),
        .binaryTarget(
            name: "RoundedQR",
            url: "https://github.com/ktiays/rounded-qr/releases/download/v0.1.0/RoundedQR.xcframework.zip",
            checksum: "f5897450e2399de444139d24c8eed85041e65848ebb33c650096a75d4e89a8b9"
        ),
    ]
)
