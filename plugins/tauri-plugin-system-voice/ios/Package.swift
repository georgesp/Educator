// swift-tools-version:5.3

import PackageDescription

let package = Package(
  name: "tauri-plugin-system-voice",
  platforms: [
    .macOS(.v10_13),
    .iOS(.v13),
  ],
  products: [
    .library(
      name: "tauri-plugin-system-voice",
      type: .static,
      targets: ["tauri-plugin-system-voice"])
  ],
  dependencies: [
    .package(name: "Tauri", path: "../.tauri/tauri-api")
  ],
  targets: [
    .target(
      name: "tauri-plugin-system-voice",
      dependencies: [
        .byName(name: "Tauri")
      ],
      path: "Sources")
  ]
)
