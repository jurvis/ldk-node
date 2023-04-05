// swift-tools-version:5.5
import PackageDescription

let tag = "0.0.1"
let checksum = "1347f7d73b5a634d7602976e9a6813fe0f8e591e56ced42904c778ba3e654649"
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
