# Defines an environment for `nix-shell` for developing Fornjot on NixOS.
#
# This is not complete, and probably not the correct NixOS way to do it either.
# It's just what works for me (@hannobraun) in combination with my local
# environment. Pull requests to improve this (best coupled with thorough
# explanations, because I understand very little about Nix/NixOS) welcome!

{ pkgs ? import <nixpkgs> { } }:

let
  # `cargo run -p <some-model>` won't work without these libraries on the path.
  libPath = with pkgs; lib.makeLibraryPath [
    libxkbcommon
    vulkan-loader
    wayland
  ];
in
pkgs.mkShell {
  packages = with pkgs; [
    # Used as a local build tool.
    just

    # Required by dependencies.
    openssl
    pkg-config
  ];

  # Otherwise `export-validator` produces an error trying to link `libstdc++`.
  LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib:${libPath}";
}
