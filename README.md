<!-- <div style="display: flex, margin: 8px"> -->
    <!-- <img src="./screenshot/1.png" width="100"/> -->
    <!-- <img src="./screenshot/2.png" width="100"/> -->
    <!-- <img src="./screenshot/3.png" width="100"/> -->
    <!-- <img src="./screenshot/4.png" width="100"/> -->
    <!-- <img src="./screenshot/5.png" width="100"/> -->
    <!-- <img src="./screenshot/6.png" width="100"/> -->
    <!-- <img src="./screenshot/7.png" width="100"/> -->
    <!-- <img src="./screenshot/8.png" width="100"/> -->
<!-- </div> -->

[中文文档](./README.zh-CN.md)

### Introduction

### Features

### Android platform build information
- `min-sdk-version = 23`
- `target-sdk-version = 32`

#### How to build?
- Install `Rust` and `Cargo`
- Install Android `sdk`, `ndk`, `jdk17`, and set environment variables
- Run `make android-build-release` to build a release version android APK
- Run `make desktop-debug` to run it on desktop platform
- Run `make desktop-build-release` to build a release version desktop application
- Refer to [Makefile](./Makefile) for more information

### Reference
- [Slint Language Documentation](https://slint-ui.com/releases/1.0.0/docs/slint/)
- [slint::android](https://snapshots.slint.dev/master/docs/rust/slint/android/#building-and-deploying)
- [github/slint-ui](https://github.com/slint-ui/slint)
- [Viewer for Slint](https://github.com/slint-ui/slint/tree/master/tools/viewer)
- [LSP (Language Server Protocol) Server for Slint](https://github.com/slint-ui/slint/tree/master/tools/lsp)
- [developer.android.com](https://developer.android.com/guide)
- [color4bg](https://www.color4bg.com/zh-hans/)
