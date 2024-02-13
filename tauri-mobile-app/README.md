# Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML, CSS and Typescript.

### Requirement

* Android Studio
    * Android SDK Platform
    * Android SDK Platform-Tools
    * NDK (Side by side)
    * Android SDK Build-Tools
    * Android SDK Command-line Tools
* Java Standard Edition 19
    * <https://jdk.java.net/java-se-ri/19>
* pnpm
* tauri-cli --version ^2.0.0-beta

### Environment

```
export ANDROID_HOME=$HOME/Android/Sdk
export NDK_HOME=$HOME/Android/Sdk/ndk/26.1.10909125
export JAVA_HOME=$HOME/OpenJDK/jdk-19
```

### Setup

```
$ pnpm install
```

### Run

```
# For Desktop development, run:
$ pnpm tauri dev

# For Android development, run:
$ pnpm tauri android dev
```

### Build and Install

```
$ pnpm tauri android build --debug

$ cd ${SDK platform-tools}
$ ./adb -s ${SERIAL NUMBER}  install ${PROJECT}/src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk
```

### Info

```
$ pnpm tauri info

> tauri-mobile-app@0.0.0 tauri /home/botamotch/Git/botamotch/Rust/tauri-mobile-app
> tauri "info"


[✔] Environment
    - OS: Arch Linux Rolling Release X64
    ✔ webkit2gtk-4.1: 2.42.4
    ✔ rsvg2: 2.57.1
    ✔ rustc: 1.75.0 (82e1608df 2023-12-21)
    ✔ cargo: 1.75.0 (1d8b05cdd 2023-11-20)
    ✔ rustup: 1.26.0 (2023-11-14)
    ✔ Rust toolchain: stable-x86_64-unknown-linux-gnu (default)
    - node: 20.10.0
    - pnpm: 8.15.1
    - yarn: 1.22.21
    - npm: 10.2.3
    - bun: 1.0.17

[-] Packages
    - tauri [RUST]: 2.0.0-beta.2
    - tauri-build [RUST]: 2.0.0-beta.1
    - wry [RUST]: 0.35.2
    - tao [RUST]: 0.25.0
    - tauri-cli [RUST]: 2.0.0-beta.1
    - @tauri-apps/api [NPM]: 2.0.0-beta.0
    - @tauri-apps/cli [NPM]: 2.0.0-beta.1

[-] App
    - build-type: bundle
    - CSP: unset
    - frontendDist: ../dist
    - devUrl: http://localhost:1420/
    - bundler: Vite
```

### Android app with wasm

```
$ wasm-pack build --target web -dev
```
