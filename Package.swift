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
      url: "https://github.com/LiterateInk/Scodok/releases/download/0.1.1/scodokFFI.xcframework.zip",
      checksum: "0cd99b8afb400c9023623c9b26d002ac9331bad184d1e06fd2ae270d5403be81"),
    .target(
      name: "Scodok",
      dependencies: [.target(name: "scodokFFI")],
      path: "swift")
  ]
)
