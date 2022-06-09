# bevy_xtemplate

Cross Platform App bevy template

## Usage

Install [cargo-generate](https://crates.io/crates/cargo-generate):

`cargo install cargo-generate`

Run the template:

`cargo generate --git https://github.com/ManevilleF/bevy_xtemplate`

## Cross platform targets

### WASM

Run the `build_wasm.sh` script. a `public` folder containing everything required to run the game in the browser.

### iOS (Requires Xcode)

You need to install the correct rust targets:

- `aarch64-apple-ios`: iOS devices
- `x86_64-apple-ios`: iOS simulator on x86 processors
- `aarch64-apple-ios-sim`: iOS simulator on Apple processors

```sh
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
```

#### Build & Run

Using bash:

```sh
cd apple
make run
```

In an ideal world, this will boot up, install and run the app for the first
iOS simulator in your `xcrun simctl devices list`. If this fails, you can
specify the simulator device UUID via:

```sh
DEVICE_ID=${YOUR_DEVICE_ID} make run
```

You can also open the `xcodeproj` with `Xcode`

### Android

- Install [cargo-apk](https://crates.io/crates/cargo-apk):

`cargo install cargo-apk`

- Install the desired android targets for `rustup`:

`rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android`

#### Build & Run

Run `cargo apk build` to generate the `apk` file.
Run `cargo apk run` to run the app on your connected emulator/device
