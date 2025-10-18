{
  description = "A very basic flake for Rust Slint GUI framework";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      # Create a custom stdenv that uses gcc instead of clang
      gccStdenv = pkgs.overrideCC pkgs.stdenv pkgs.gcc;

      shellBuildInputs = with pkgs; [
        wayland
        openssl
        libGL.dev
        xorg.libxcb
        libxkbcommon
        stdenv.cc.cc.lib
        qt6.full
        pkg-config
      ];

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        targets = [ "aarch64-linux-android" "wasm32-unknown-unknown" ];
      };
    in {
      devShells.${system}.default =
        pkgs.mkShell.override { stdenv = gccStdenv; } {
          nativeBuildInputs = with pkgs; [
            gcc
            llvm
            clang
            python3
            gnumake
            rustToolchain
            rust-analyzer
          ];

          buildInputs = shellBuildInputs;

          env = {
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath shellBuildInputs;

            # Force GCC for C/C++ compilation
            CC = "${pkgs.gcc}/bin/gcc";
            CXX = "${pkgs.gcc}/bin/g++";

            # Set include paths
            C_INCLUDE_PATH = with pkgs;
              lib.concatStringsSep ":" [ "${glibc.dev}/include" ];

            CPLUS_INCLUDE_PATH = with pkgs;
              lib.concatStringsSep ":" [ "${glibc.dev}/include" ];

            PKG_CONFIG_PATH = with pkgs;
              lib.concatStringsSep ":" [ "${wayland.dev}/lib/pkgconfig" ];

            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            RUST_SRC_PATH =
              "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };

          shellHook = ''
            export PS1="($(basename $(pwd)))> ";
            alias ee=exit

            echo "[INFO] ✨ Rust development environment with Slint GUI framework support is ready!"
            echo "[INFO] 📦 Rust version: $(rustc --version)"
            echo "[INFO] 📦 C++ compiler: $(g++ --version | head -n1)"
            echo "[INFO] 📦 Clang version: ${pkgs.llvmPackages.clang.version}"
            echo "[INFO] 📦 Qt version: ${pkgs.qt6.qtbase.version}"
          '';
        };
    };
}
