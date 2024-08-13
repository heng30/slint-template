{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [ libGL.dev openssl qt5.full xorg.libxcb ];
  nativeBuildInputs = with pkgs; [ pkg-config python3 libsForQt5.qmake ];
}
