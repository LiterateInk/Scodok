// swift-tools-version: 6.0

import PackageDescription

let package = Package(
  name: "scodok",
  platforms: [
    .iOS(.v12),
    .macOS(.v10_15)
  ],
  products: [
    .library(
      name: "Scodok",
      targets: ["Scodok"]),
  ],
  targets: [
    .binaryTarget(
      name: "scodokFFI",
      url: "https://github.com/LiterateInk/Scodok/releases/download/0.1.0/scodokFFI.xcframework.zip",
      checksum: "something"),
    .target(
      name: "Scodok",
      dependencies: [.target(name: "scodokFFI")],
      path: "swift")
  ]
)
