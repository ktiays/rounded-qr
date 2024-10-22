#!/bin/sh

rm -rf ./RoundedQR.xcframework
rm -rf .xcframework-intermediate

cargo build --release --target aarch64-apple-ios-sim --features ffi
cargo build --release --target aarch64-apple-ios --features ffi

mkdir -p .xcframework-intermediate/include/RoundedQR
cp include/* .xcframework-intermediate/include/RoundedQR

xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/librounded_qr.a \
    -headers .xcframework-intermediate/include \
    -library target/aarch64-apple-ios-sim/release/librounded_qr.a \
    -headers .xcframework-intermediate/include \
    -output ./RoundedQR.xcframework