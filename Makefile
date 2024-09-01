#!/bin/sh

pwd=${shell pwd}

app-name=slint-template
version=`git describe --tags --abbrev=0`

build-env=SLINT_ENABLE_EXPERIMENTAL_FEATURES=1
android-build-env=SLINT_STYLE=material $(build-env)
desktop-build-env=SLINT_STYLE=fluent $(build-env)
web-build-env=SLINT_STYLE=fluent $(build-env)

run-env=RUST_LOG=debug,sqlx=off,reqwest=off

all: desktop-build-release

android-build:
	$(android-build-env) cargo apk build --lib --features=android

android-build-release:
	$(android-build-env) cargo apk build --lib --release --features=android

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
	$(web-build-env) wasm-pack build --target web --out-dir ./web/pkg --features=web

web-build-release:
	$(web-build-env) wasm-pack build --release --target web --out-dir ./web/pkg --features=web

web-build-dist:
	- rm -rf ./web/dist/*
	$(web-build-env) wasm-pack build --release --target web --out-dir ./web/dist/pkg --features=web
	cp -f ./web/index.html ./web/dist
	cp -f ./ui/images/brand.png ./web/dist/pkg/favicon.png

web-server:
	python3 -m http.server -d web 8000

web-server-dist:
	python3 -m http.server -d web/dist 8800

packing-android:
	cp -f target/release/apk/${app-name}.apk target/${app-name}-${version}-aarch64-linux-android.apk
	echo "${app-name}-${version}-aarch64-linux-android.apk" > target/output.name

packing-linux:
	cp -f target/release/${app-name} target/${app-name}-${version}-x86_64-linux
	echo "${app-name}-${version}-x86_64-linux" > target/output.name

packing-windows:
	cp -f target/release/${app-name}.exe target/${app-name}-${version}-x86_64-windows.exe
	echo "${app-name}-${version}-x86_64-windows.exe" > target/output.name

packing-darwin:
	cp -f target/release/${app-name} target/${app-name}-${version}-x86_64-darwin
	echo "${app-name}-${version}-x86_64-darwin" > target/output.name

packing-web:
	tar -zcf target/$(app-name)-$(version)-web.tar.gz web/dist
	echo "$(app-name)-$(version)-web.tar.gz" > target/output.name


slint-view-android:
	$(android-build-env) slint-viewer --auto-reload -I ui ./ui/android-window.slint

slint-view-desktop:
	$(desktop-build-env) slint-viewer --auto-reload -I ui ./ui/desktop-window.slint

slint-view-web:
	$(web-build-env) slint-viewer --auto-reload -I ui ./ui/web-window.slint

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

get-font-name:
	fc-scan ./ui/fonts/SourceHanSerifCN.ttf | grep fullname
