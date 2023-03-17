// swift-tools-version:5.5
import PackageDescription

let tag = "0.0.1"
let checksum = "35aa9716c6f6bc5a2b03dcf2b1447175e9c904574390b3a40b8ed4bd4d5fe53c"
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
            targets: ["LightningDevKitNode"]
        )
    ],
    targets: [
        .binaryTarget(
            name: "LightningDevKitNode",
            url: url,
            checksum: checksum
        )
    ]
)
