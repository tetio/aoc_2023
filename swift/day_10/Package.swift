// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "day_10",
    // products: [
    //     // Products define the executables and libraries a package produces, making them visible to other packages.
    //     .library(
    //         name: "day_10",
    //         targets: ["day_10"]),
    // ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
                // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .executableTarget(
            name: "day_10"),
        // .target(
        //     name: "day_10"),
        .testTarget(
            name: "day_10Tests",
            dependencies: ["day_10"]),
    ]
)
