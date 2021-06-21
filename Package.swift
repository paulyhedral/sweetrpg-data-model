// swift-tools-version:5.4
import PackageDescription


let package = Package(
        name: "sweetrpg-data-model",
        platforms: [
            .macOS(.v10_15),
        ],
        products: [
            .library(name: "DataModel", targets: [ "DataModel" ])
        ],
        dependencies: [
            .package(url: "https://github.com/vapor/vapor.git", from: "4.0.0"),
            .package(url: "https://github.com/vapor/fluent.git", from: "4.0.0"),
        ],
        targets: [
            .target(
                    name: "DataModel",
                    dependencies: [
                        .product(name: "Fluent", package: "fluent"),
                        .product(name: "Vapor", package: "vapor"),
                    ],
                    swiftSettings: [
                        // Enable better optimizations when building in Release configuration. Despite the use of
                        // the `.unsafeFlags` construct required by SwiftPM, this flag is recommended for Release
                        // builds. See <https://github.com/swift-server/guides/blob/main/docs/building.md#building-for-production> for details.
                        .unsafeFlags([ "-cross-module-optimization" ], .when(configuration: .release)),
                    ]
            ),
            .testTarget(name: "DataModelTests", dependencies: [
                .target(name: "DataModel"),
                .product(name: "XCTVapor", package: "vapor"),
            ]),
        ]
)
