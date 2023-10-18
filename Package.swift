// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "sweetrpg-kv-objects",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "sweetrpg-kv-objects",
            targets: ["kvObjects"]),
    ],
     dependencies: [
         .package(url: "https://github.com/httpswift/swifter.git", .upToNextMajor(from: "1.4.5"))
     ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "kvObjects"  , dependencies: ["Swifter"]),
        .testTarget(
            name: "kvObjectsTests",
            dependencies: ["kvObjects"]),
    ]
)
