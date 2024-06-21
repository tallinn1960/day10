// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "Day10Swift",
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .library(
      name: "Day10Swift",
      type: .static,
      targets: ["Day10Swift"])
  ],
  dependencies: [
    .package(url: "https://github.com/Brendonovich/swift-rs", from: "1.0.5")
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .target(
      name: "Day10Swift",
      dependencies: [.product(name: "SwiftRs", package: "swift-rs")]),
    .testTarget(
      name: "Day10SwiftTests",
      dependencies: ["Day10Swift"]),
  ]
)
