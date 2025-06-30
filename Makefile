#!/bin/sh

pwd = ${shell pwd}

app-name = slint-template
version = `git describe --tags --abbrev=0`

build-env =
android-build-env = SLINT_STYLE=material $(build-env)
desktop-build-env = SLINT_STYLE=fluent $(build-env)
desktop-debug-build-env = RUSTFLAGS="-Cllvm-args=-inline-threshold=10 -Cllvm-args=-inlinedefault-threshold=10 -Cllvm-args=-inlinehint-threshold=10"
web-build-env = SLINT_STYLE=fluent $(build-env) RUSTFLAGS='--cfg getrandom_backend="wasm_js"'

run-env = RUST_LOG=debug,reqwest=warn,sqlx=warn
proj-features = --features=desktop,database,qrcode,center-window

all: desktop-build-release

android-build:
	$(android-build-env) cargo apk build --lib -p ${app-name} --features=android

android-build-release:
	$(android-build-env) cargo apk build --lib --release -p ${app-name} --features=android

android-debug:
	$(android-build-env) $(run-env) cargo apk run --lib -p ${app-name} --features=android

desktop-build-debug:
	$(desktop-build-env) cargo build --features=desktop

desktop-build-release:
	$(desktop-build-env) cargo build --release --features=desktop

desktop-debug:
	$(desktop-build-env) $(run-env) cargo run --bin ${app-name} --features=desktop

web-build-debug:
	cd $(app-name) && $(web-build-env) wasm-pack build --no-opt --dev --target web --out-dir ./web/pkg --features=web

web-build-release:
	cd $(app-name) && $(web-build-env) wasm-pack build --no-opt --release --target web --out-dir ./web/pkg --features=web

web-build-dist:
	- rm -rf ./web/dist/*
	cd $(app-name) && $(web-build-env) wasm-pack build --no-opt --release --target web --out-dir ./web/dist/pkg --features=web
	cd $(app-name) && cp -f ./web/index.html ./web/dist && cp -f ./ui/images/brand.png ./web/dist/pkg/favicon.png

web-server:
	cd $(app-name) && python3 -m http.server -d web 8000

web-server-dist:
	cd $(app-name) && python3 -m http.server -d web/dist 8800

packing-android:
	cp -f target/release/apk/${app-name}.apk target/${app-name}-${version}-aarch64-linux-android.apk
	echo "${app-name}-${version}-aarch64-linux-android.apk" > target/output-name

packing-linux:
	cp -f target/release/${app-name} target/${app-name}-${version}-x86_64-linux
	echo "${app-name}-${version}-x86_64-linux" > target/output-name

packing-windows:
	cp -f target/release/${app-name}.exe target/${app-name}-${version}-x86_64-windows.exe
	echo "${app-name}-${version}-x86_64-windows.exe" > target/output-name

packing-darwin:
	cp -f target/release/${app-name} target/${app-name}-${version}-x86_64-darwin
	echo "${app-name}-${version}-x86_64-darwin" > target/output-name

packing-web:
	tar -zcf target/$(app-name)-$(version)-web.tar.gz ${app-name}/web/dist
	echo "$(app-name)-$(version)-web.tar.gz" > target/output-name

reduce-linux-binary-size:
	upx -9 target/release/$(app-name)

slint-viewer-android:
	$(android-build-env) slint-viewer --auto-reload -I $(app-name)/ui ${app-name}/ui/android-window.slint

slint-viewer-desktop:
	$(desktop-build-env) slint-viewer --auto-reload -I $(app-name)/ui ${app-name}/ui/desktop-window.slint

slint-viewer-web:
	$(web-build-env) slint-viewer --auto-reload -I $(app-name)/ui ${app-name}/ui/web-window.slint

nix-shell:
	nix-shell

test:
	$(build-env) $(run-env) cargo test -- --nocapture

timings:
	$(build-env) cargo build --timings $(proj-features)

clippy:
	cargo clippy $(proj-features)

check:
	cargo check $(proj-features)

clean-incremental:
	rm -rf ./target/debug/incremental
	rm -rf ./target/aarch64-linux-android/debug/incremental

clean-unused-dependences:
	cargo machete

clean:
	cargo clean

deb:
	cd ./${app-name}/pkg/deb && bash -e "./create_deb.sh"
	mv ./${app-name}/pkg/deb/$(app-name).deb ./target

app-name:
	- mkdir -p target
	echo "$(app-name)" > target/app-name

get-font-name:
	fc-scan ./${app-name}/ui/fonts/*.{ttf,otf} | grep "fullname:"

