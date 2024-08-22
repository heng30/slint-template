#!/bin/sh

pwd=${shell pwd}
build-evn=SLINT_STYLE=material SLINT_ENABLE_EXPERIMENTAL_FEATURES=1
run-evn=RUST_LOG=debug,sqlx=off,reqwest=off
version=`git describe --tags --abbrev=0`
app-name=slint-template

all: desktop-build-release

android-build:
	$(build-evn) cargo apk build --lib --features=android

android-build-release:
	$(build-evn) cargo apk build --lib --release --features=android
	cp -f target/release/apk/${app-name}.apk target/${app-name}-${version}.apk

android-debug:
	$(run-evn) cargo apk run --lib --features=android

desktop-debug:
	$(build-evn) $(run-evn) cargo run --features=desktop

desktop-run-debug:
	$(run-evn) ./target/debug/${app-name}

desktop-run-release:
	$(run-evn) ./target/release/${app-name}

desktop-build-debug:
	$(build-evn) $(run-evn) cargo build --features=desktop

desktop-build-release:
	$(build-evn) $(run-evn) cargo build --release --features=desktop

desktop-build-debug-nixos:
	nix-shell --run "$(build-evn) $(run-evn) cargo build --features=desktop"

desktop-build-release-nixos:
	nix-shell --run "$(build-evn) $(run-evn) cargo build --release --features=desktop"

test:
	$(build-evn) $(run-evn) cargo test -- --nocapture

clippy:
	cargo clippy

clean-incremental:
	rm -rf ./target/debug/incremental
	rm -rf ./target/aarch64-linux-android/debug/incremental

clean-unused-dependences:
	cargo machete

clean:
	cargo clean

slint-view:
	SLINT_ENABLE_EXPERIMENTAL_FEATURES=1 slint-viewer --style material --auto-reload -I ui ./ui/appwindow.slint

slint-view-light:
	SLINT_ENABLE_EXPERIMENTAL_FEATURES=1 slint-viewer --style material-light --auto-reload -I ui ./ui/appwindow.slint

slint-view-dark:
	SLINT_ENABLE_EXPERIMENTAL_FEATURES=1 slint-viewer --style material-dark --auto-reload -I ui ./ui/appwindow.slint

get-font-name:
	fc-scan ./ui/fonts/SourceHanSerifCN.ttf | grep fullname
