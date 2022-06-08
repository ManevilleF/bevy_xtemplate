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

- Install the desired iOS targets for `rustup`:

`rustup target add aarch64-apple-ios`

Run the `apple/Makefile` to launch the app on the Xcode emulator or your connected device

### Android

- Install [cargo-apk](https://crates.io/crates/cargo-apk):

`cargo install cargo-apk`

- Install the desired android targets for `rustup`:

`rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android`

Run `cargo apk build` to generate the `apk` file.
Run `cargo apk run` to run the app on your connected emulator/device
