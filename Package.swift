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
            url: "https://github.com/ktiays/rounded-qr/releases/download/v0.1.1/RoundedQR.xcframework.zip",
            checksum: "058af7c6ed4e35707a74b4ede72f7d9878dc81092ec1243c39bdc0ff3fd5df8f"
        ),
    ]
)
