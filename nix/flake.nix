{
  description = "Fornjot is an early-stage project to create a next-generation, code-first CAD application";
  inputs = {
    flake-compat = { url = "github:edolstra/flake-compat"; flake = false; };
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = { url = "github:oxalica/rust-overlay"; inputs = { nixpkgs.follows = "nixpkgs"; flake-utils.follows = "flake-utils"; }; };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-compat.follows = "flake-compat";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ (import rust-overlay) ]; };
        rustToolchain = pkgs.rust-bin.fromRustupToolchain (
          # extend toolchain with rust-analyzer for better IDE support
          let toolchainToml = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml)).toolchain; in
          {
            channel = toolchainToml.channel;
            components = toolchainToml.components ++ [ "rust-analyzer" ];
          }
        );
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        # Only keeps assets in crates/ (currently shaders and fonts)
        assetsFilter = path: _type: (builtins.match ".*(:?wgsl|ttf)$" path) != null;
        filter = path: type: (assetsFilter path type) || (craneLib.filterCargoSources path type);
        buildInputs = with pkgs; [
          pkg-config
          fontconfig
          cmake
          wayland
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];

        fornjot = craneLib.buildPackage {
          pname = "fj-app";
          src = nixpkgs.lib.cleanSourceWith { src = ./.; inherit filter; };
          inherit buildInputs;
        };

        wrappedFornjot = pkgs.symlinkJoin {
          name = "fj-app";
          paths = [ fornjot ];

          buildInputs = [ pkgs.makeWrapper ];

          postBuild = ''
            wrapProgram $out/bin/fj-app \
              --prefix LD_LIBRARY_PATH : ${nixpkgs.lib.makeLibraryPath [ pkgs.vulkan-loader ]}
          '';
        };
      in
      {
        checks = { inherit fornjot; };

        packages.default = wrappedFornjot;

        apps.default = flake-utils.lib.mkApp { drv = wrappedFornjot; };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          inherit buildInputs;
          nativeBuildInputs = [ rustToolchain ];

          LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath [ pkgs.vulkan-loader ]}";
        };
      });
}
