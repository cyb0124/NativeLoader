set -eux
export SDKROOT=D:/MacOSX15.2.sdk
cargo clean
cargo build --target x86_64-pc-windows-msvc --release
cargo build --target x86_64-unknown-linux-gnu --release
cargo build --target aarch64-unknown-linux-gnu --release
cargo build --target x86_64-apple-darwin --release
cargo build --target aarch64-apple-darwin --release
cp target/x86_64-pc-windows-msvc/release/NativeLoader.dll bin/NativeLoader-x64.dll
cp target/x86_64-unknown-linux-gnu/release/libNativeLoader.so bin/NativeLoader-x64.so
cp target/aarch64-unknown-linux-gnu/release/libNativeLoader.so bin/NativeLoader-aarch64.so
cp target/x86_64-apple-darwin/release/libNativeLoader.dylib bin/NativeLoader-x64.dylib
cp target/aarch64-apple-darwin/release/libNativeLoader.dylib bin/NativeLoader-aarch64.dylib
