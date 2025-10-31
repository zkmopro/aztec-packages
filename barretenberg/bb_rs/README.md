# bb.rs

Rust bindings for Barretenberg C++ codebase.

## Supported Platforms

bb_rs supports the following platforms:
- **iOS**: aarch64-apple-ios, aarch64-apple-ios-sim, x86_64-apple-ios
- **Android**: aarch64-linux-android, x86_64-linux-android
- **macOS**: aarch64-apple-darwin, x86_64-apple-darwin
- **Linux**: x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu

## Build

```bash
# Build on your own machine
cargo build -vvvv

# Cross-compile for iOS
cargo build -vvvv --target aarch64-apple-ios

# Cross-compile for Android
cargo build -vvvv --target aarch64-linux-android
```

## Known issues

### Missing `sys/random.h`

random.h is not available in the iOS SDK includes but it is available in the MacOS SDK includes. So you can copy it from `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys` and paste it in `/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk/usr/include/sys`. This will work, no compability issues, it's just not there for some reason.

You can also run `scripts/patcher.sh` to do this (you may need to run it as `sudo`).



