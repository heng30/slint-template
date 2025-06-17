{ pkgs ? import <nixpkgs> { } }:

let
  libPath = with pkgs;
    lib.makeLibraryPath [
      wayland
      openssl
      qt6.full
      libGL.dev
      xorg.libxcb
      libxkbcommon
      stdenv.cc.cc.lib
    ];
in {
  runShell = pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      gcc
      mold
      llvm
      clang
      python3
      gnumake
      pkg-config

      # find rustc and cargo in system's path
      # rustc
      # cargo
    ];

    buildInputs = with pkgs; [
      openssl
      qt6.full
      libGL.dev
      xorg.libxcb
      stdenv.cc.cc.lib
    ];

    env = {
      LD_LIBRARY_PATH = libPath;
      PKG_CONFIG_PATH = "${pkgs.opencv}/lib/pkgconfig";
      LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
      CPLUS_INCLUDE_PATH =
        "${pkgs.gcc-unwrapped}/include/c++/${pkgs.gcc.version}";
    };

    shellHook = ''
      echo "Rust development environment with Slint support is ready!"
      echo "Rust version: $(rustc --version)"
      echo "Clang version: ${pkgs.llvmPackages.clang.version}"
    '';
  };
}
