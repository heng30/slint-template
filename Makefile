#!/bin/sh

pwd=${shell pwd}
build-env=SLINT_ENABLE_EXPERIMENTAL_FEATURES=1
desktop-build-env=SLINT_STYLE=fluent $(build-env)
android-build-env=SLINT_STYLE=material $(build-env)
run-env=RUST_LOG=debug,sqlx=off,reqwest=off
version=`git describe --tags --abbrev=0`
app-name=slint-template

all: desktop-build-release

android-build:
	$(android-build-env) cargo apk build --lib --features=android

android-build-release:
	$(android-build-env) cargo apk build --lib --release --features=android
	cp -f target/release/apk/${app-name}.apk target/${app-name}-${version}.apk

android-debug:
	$(android-build-env) $(run-env) cargo apk run --lib --features=android

desktop-build-debug:
	$(desktop-build-env) cargo build --features=desktop

desktop-build-release:
	$(desktop-build-env) cargo build --release --features=desktop

desktop-build-debug-nixos:
	nix-shell --run "$(desktop-build-env) cargo build --features=desktop"

desktop-build-release-nixos:
	nix-shell --run "$(desktop-build-env) cargo build --release --features=desktop"

desktop-debug:
	$(desktop-build-env) $(run-env) cargo run --features=desktop

web-build-debug:
	wasm-pack build --target web --out-dir target/pkg --features=web

web-build-release:
	wasm-pack build --release --target web --out-dir target/pkg --features=web

web-debug:
	python3 -m http.server

test:
	$(build-env) $(run-env) cargo test -- --nocapture

clippy:
	cargo clippy

clean-incremental:
	rm -rf ./target/debug/incremental
	rm -rf ./target/aarch64-linux-android/debug/incremental

clean-unused-dependences:
	cargo machete

clean:
	cargo clean

slint-view-android:
	$(android-build-env) slint-viewer --auto-reload -I ui ./ui/appwindow.slint

slint-view-desktop:
	$(desktop-build-env) slint-viewer --auto-reload -I ui ./ui/appwindow.slint

get-font-name:
	fc-scan ./ui/fonts/SourceHanSerifCN.ttf | grep fullname
