# Quinn4Android

## Compile Quinn-rs-binding
First install [cross](https://github.com/cross-rs/cross) (it's a tool to cross compile inside a docker image)
```
cargo install cross
```
Then move into the quinn-rs-binding directory and build the quinn-rs-binding library.

For arm64 :
```
cross build --target aarch64-linux-android --release
```
The .so will be in "target/aarch64-linux-android/release"

For arm32 :
```
cross build --target arm-linux-androideabi --release
```