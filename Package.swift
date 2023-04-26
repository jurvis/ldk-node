// swift-tools-version:5.5
import PackageDescription

let tag = "0.0.3"
let checksum = "ff4ccadb02d33b085e2358745e18b1a4e3539ed2c4861f5b58c374be88cf7a7d"
let url = "https://github.com/jurvis/ldk-node/releases/download/\(tag)/ldk_nodeFFI.xcframework.zip"

let package = Package(
    name: "LightningDevKitNode",
    platforms: [
        .iOS(.v15),
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "LightningDevKitNode",
            targets: ["ldk_nodeFFI", "LightningDevKitNode"]
        )
    ],
    targets: [
        .target(
            name: "LightningDevKitNode",
            dependencies: ["ldk_nodeFFI"],
            path: "./bindings/swift/Sources"
        ),
        .binaryTarget(
            name: "ldk_nodeFFI",
            url: url,
            checksum: checksum
        )
    ]
)
