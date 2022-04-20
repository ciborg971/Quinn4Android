#export TARGET_CC=${ANDROID_HOME}/ndk/22.0.7026061/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang
#export AR=${ANDROID_HOME}/ndk/22.0.7026061/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android-ar
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android
cargo clean
cargo build
