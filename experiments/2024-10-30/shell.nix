{ pkgs ? import <nixpkgs> { } }:

let
  libPath = with pkgs; lib.makeLibraryPath [
    libxkbcommon
    wayland
  ];
in
pkgs.mkShell {
  LD_LIBRARY_PATH = "${libPath}";
}
