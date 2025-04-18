{ pkgs ? import <nixpkgs> { } }:

let libPath = with pkgs; lib.makeLibraryPath [ libGL libxkbcommon wayland ];
in {
  runShell = pkgs.mkShell {
      buildInputs = with pkgs; [ openssl qt5.full xorg.libxcb ];
      nativeBuildInputs = with pkgs; [ pkg-config python3 libsForQt5.qmake ];

      LD_LIBRARY_PATH = libPath;
    };
}
