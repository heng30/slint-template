<div style="display: flex, margin: 8px">
    <img src="./screenshot/1.png" width="100"/>
    <img src="./screenshot/2.png" width="100"/>
    <img src="./screenshot/3.png" width="100"/>
    <img src="./screenshot/4.png" width="100"/>
    <img src="./screenshot/5.png" width="100"/>
    <img src="./screenshot/6.png" width="100"/>
    <img src="./screenshot/7.png" width="100"/>
    <img src="./screenshot/8.png" width="100"/>
</div>

[English Documentation](./README.md)

### 简介

### 功能

### 安卓平台编译信息
- `min-sdk-version = 23`
- `target-sdk-version = 32`

### 如何构建?
- 安装 `Rust` 和 `Cargo`
- 安装 Android `sdk`, `ndk`, `jdk17`, 和设置对应的环境变量
- 运行 `make` 编译安卓平台程序
- 运行 `make debug` 调试桌面平台程序
- 运行 `make build-desktop-release` 编译桌面平台程序
- 参考 [Makefile](./Makefile) 了解更多信息

### 参考
- [Slint Language Documentation](https://slint-ui.com/releases/1.0.0/docs/slint/)
- [github/slint-ui](https://github.com/slint-ui/slint)
- [Viewer for Slint](https://github.com/slint-ui/slint/tree/master/tools/viewer)
- [LSP (Language Server Protocol) Server for Slint](https://github.com/slint-ui/slint/tree/master/tools/lsp)
- [developer.android.com](https://developer.android.com/guide)
